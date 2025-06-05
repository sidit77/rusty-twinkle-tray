use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;
use std::mem::take;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::{Duration, Instant};

use futures_lite::future::yield_now;
use futures_lite::FutureExt;
use log::{debug, trace, warn};
use loole::{unbounded, Receiver, Sender};

use crate::config::Config;
use crate::monitors::{Monitor, MonitorConnection, MonitorPath};
use crate::runtime::reducing_spsc::{Reducible, ReducingReceiver, ReducingSender, TryRecvError};
use crate::runtime::{block_on, reducing_spsc, LocalExecutor, Timer};
use crate::utils::extensions::{ChannelExt, MutexExt};
use crate::CustomEvent;

#[derive(Debug, Clone)]
enum BackendCommand {
    Stop,
    RefreshMonitors,
    QueryBrightness(Option<Duration>),
    SetBrightness(MonitorPath, u32)
}

pub struct MonitorController {
    sender: Sender<BackendCommand>,
    thread: Option<JoinHandle<()>>
}

impl MonitorController {
    pub fn new(main_sender: Sender<CustomEvent>, config: Arc<Mutex<Config>>) -> Self {
        let (sender, receiver) = unbounded();
        sender.send_ignore(BackendCommand::RefreshMonitors);
        let thread = Some(spawn(move || block_on(Self::worker_task(main_sender, receiver, config))));
        Self { sender, thread }
    }

    async fn worker_task(sender: Sender<CustomEvent>, receiver: Receiver<BackendCommand>, config: Arc<Mutex<Config>>) {
        let executor = LocalExecutor::default();
        let mut monitor_map: BTreeMap<MonitorPath, ReducingSender<MonitorCommand>> = BTreeMap::new();
        let mut delayed_query: Option<Instant> = None;

        executor
            .run(async {
                loop {
                    let query_event = async {
                        delayed_query
                            .map(Timer::at)
                            .unwrap_or_else(Timer::never)
                            .await;
                        BackendCommand::QueryBrightness(None)
                    };
                    let command_event = async { receiver.recv_async().await.expect("Controller disappeared") };
                    match query_event.or(command_event).await {
                        BackendCommand::QueryBrightness(None) => {
                            delayed_query.take();
                            let config = config.lock_no_poison();
                            monitor_map.iter().for_each(|(p, s)| {
                                s.send_ignore(MonitorCommand::QueryBrightness {
                                    target: config
                                        .restore_from_config
                                        .then(|| config.monitors.get(p))
                                        .flatten()
                                        .and_then(|s| s.saved_brightness)
                                })
                            });
                        }
                        BackendCommand::RefreshMonitors => {
                            trace!("Refreshing monitor list");
                            let mut current_monitors = Monitor::find_all()
                                .map_err(|err| log::warn!("Failed to enumerate monitors: {err}"))
                                .unwrap_or_default();
                            debug!("Skipping over unnamed monitors as they are likely integrated displays");
                            current_monitors.retain(|m| !m.name().is_empty());

                            let old_monitors = take(&mut monitor_map).into_keys().collect::<BTreeSet<_>>();

                            // Yield to allow the monitor tasks to finish
                            yield_now().await;
                            executor.clean_pending_tasks();

                            for path in &old_monitors {
                                if current_monitors.iter().all(|m| m.path() != path) {
                                    sender.send_ignore(CustomEvent::MonitorRemoved { path: path.clone() });
                                }
                            }

                            for monitor in current_monitors {
                                let path = monitor.path().clone();
                                if !old_monitors.contains(&path) {
                                    sender.send_ignore(CustomEvent::MonitorAdded {
                                        path: path.clone(),
                                        name: monitor.name().to_string()
                                    });
                                }
                                let (tx, rx) = reducing_spsc::channel();
                                executor.spawn(monitor_task(monitor, sender.clone(), rx));
                                monitor_map.insert(path, tx);
                            }

                            delayed_query = Some(Instant::now());
                        }
                        BackendCommand::QueryBrightness(Some(delay)) => {
                            delayed_query = delayed_query
                                .into_iter()
                                .chain([Instant::now() + delay])
                                .min();
                        }
                        BackendCommand::SetBrightness(p, v) => {
                            monitor_map
                                .get(&p)
                                .map(|s| s.send_ignore(MonitorCommand::SetBrightness { value: v, notify: false }))
                                .unwrap_or_else(|| warn!("Unknown monitor {:?}", p));
                            let mut config = config.lock_no_poison();
                            if config.restore_from_config {
                                config.monitors.entry(p).or_default().saved_brightness = Some(v);
                                config.dirty = true;
                            }
                        }
                        BackendCommand::Stop => break
                    }
                }
            })
            .await;
    }

    pub fn refresh_brightness(&self) {
        self.send_command(BackendCommand::QueryBrightness(None));
    }

    pub fn shutdown(&self) {
        self.send_command(BackendCommand::Stop);
    }

    fn send_command(&self, command: BackendCommand) {
        self.sender.send_ignore(command)
    }

    pub fn create_proxy(&self) -> MonitorControllerProxy {
        MonitorControllerProxy(self.sender.clone())
    }
}

impl Drop for MonitorController {
    fn drop(&mut self) {
        let _ = self.sender.send(BackendCommand::Stop);
        if let Some(handle) = self.thread.take() {
            debug!("Waiting for worker thread to shutdown!");
            handle.join().expect("worker thread panic");
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MonitorCommand {
    QueryBrightness { target: Option<u32> },
    SetBrightness { value: u32, notify: bool }
}

impl Reducible for MonitorCommand {
    fn reduce(self, other: Self) -> Self {
        use MonitorCommand::*;
        match (self, other) {
            (QueryBrightness { target: current }, QueryBrightness { target: next }) => QueryBrightness { target: next.or(current) },
            (QueryBrightness { .. }, SetBrightness { value, .. }) => QueryBrightness { target: Some(value) },
            (SetBrightness { value, .. }, QueryBrightness { target }) => QueryBrightness {
                target: Some(target.unwrap_or(value))
            },
            (SetBrightness { .. }, SetBrightness { value, notify }) => SetBrightness { value, notify }
        }
    }
}

async fn monitor_task(monitor: Monitor, sender: Sender<CustomEvent>, mut control: ReducingReceiver<MonitorCommand>) {
    let mut current_brightness = None;
    let mut cached_connection: Option<MonitorConnection> = None;

    let mut next_command = None;
    let mut dirty = false;

    loop {
        match next_command.take().ok_or(()).or(control.try_recv()) {
            Err(TryRecvError::Closed) => break,
            Err(TryRecvError::Empty) => {
                if dirty {
                    debug!("Attempting to write save new settings for {}", monitor.name());
                    if let Some(connection) = &cached_connection {
                        retry(|| connection.save_settings())
                            .await
                            .unwrap_or_else(|err| warn!("Failed to save new settings for {}: {}", monitor.name(), err));
                    }
                    dirty = false;
                }
                trace!("Closing connection to {} and going to sleep", monitor.name());
                cached_connection = None;
                next_command = control.recv().await;
            }
            Ok(MonitorCommand::SetBrightness { value, .. }) if Some(value) == current_brightness => {
                trace!("Brightness already at requested level {}", value);
            }
            Ok(command) => {
                if cached_connection.is_none() {
                    trace!("Opening connection to {}", monitor.name());
                    cached_connection = retry(|| monitor.open())
                        .await
                        .map_err(|err| warn!("Failed to connect to monitor: {err}"))
                        .ok();
                }
                let connection = match cached_connection.as_ref() {
                    Some(conn) => conn,
                    None => continue
                };
                match command {
                    MonitorCommand::QueryBrightness { target } => {
                        trace!("Attempting to read brightness of {}", monitor.name());
                        match retry(|| connection.get_brightness()).await {
                            Ok((brightness, range)) => {
                                if range != (0..=100) {
                                    warn!("unexpected brightness range: {:?}", range);
                                }
                                current_brightness = Some(brightness);
                                match target {
                                    Some(target) if target != brightness => {
                                        log::info!(
                                            "Restoring saved brightness for {} (current: {}, saved: {})",
                                            monitor.name(),
                                            brightness,
                                            target
                                        );
                                        next_command = Some(MonitorCommand::SetBrightness { value: target, notify: true });
                                    }
                                    _ => {
                                        sender.send_ignore(CustomEvent::BrightnessChanged {
                                            path: monitor.path().clone(),
                                            value: brightness
                                        });
                                    }
                                }
                            }
                            Err(err) => warn!("Failed to query brightness: {err}")
                        }
                    }
                    MonitorCommand::SetBrightness { value, notify } => {
                        trace!("Attempting to set brightness of {}", monitor.name());
                        let success = retry(|| connection.set_brightness(value))
                            .await
                            .map_err(|err| warn!("Failed to set brightness: {err}"))
                            .is_ok();
                        if success {
                            current_brightness = Some(value);
                            dirty = true;
                        }
                        if notify {
                            if let Some(current) = current_brightness {
                                sender.send_ignore(CustomEvent::BrightnessChanged {
                                    path: monitor.path().clone(),
                                    value: current
                                });
                            }
                        }
                    }
                }
                Timer::after(Duration::from_millis(250)).await;
            }
        }
    }

    trace!("Monitor task for {} is shutting down", monitor.name());
}

async fn retry<R, E: Display, F: FnMut() -> std::result::Result<R, E>>(mut op: F) -> std::result::Result<R, E> {
    let mut tries = 0;
    let mut backoff_ms = 100;
    loop {
        match op() {
            Ok(result) => return Ok(result),
            Err(err) if tries <= 4 => {
                debug!("Retrying in {}: {}", backoff_ms, err);
                Timer::after(Duration::from_millis(backoff_ms)).await;
                tries += 1;
                backoff_ms *= 2;
            }
            Err(err) => return Err(err)
        }
    }
}

pub struct MonitorControllerProxy(Sender<BackendCommand>);

impl MonitorControllerProxy {
    pub fn set_brightness(&self, monitor: MonitorPath, value: u32) {
        self.send_command(BackendCommand::SetBrightness(monitor, value));
    }

    pub fn refresh_brightness_in(&self, delay: Duration) {
        self.send_command(BackendCommand::QueryBrightness(Some(delay)));
    }

    pub fn refresh_monitors(&self) {
        self.send_command(BackendCommand::RefreshMonitors);
    }

    fn send_command(&self, command: BackendCommand) {
        self.0.send_ignore(command);
    }
}
