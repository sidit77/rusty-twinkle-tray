mod proxy;

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;
use std::mem::take;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::{Duration, Instant};

use async_executor::{LocalExecutor, Task};
use futures_lite::{FutureExt};
use log::{trace, warn};
use loole::{unbounded, Receiver, Sender};
use crate::config::Config;
use crate::monitors::{Monitor, MonitorConnection, MonitorPath};
use crate::runtime::{block_on, Event, Timer};
use crate::utils::extensions::{ChannelExt, MutexExt};
use crate::CustomEvent;

pub use proxy::MonitorControllerProxy;

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
        let thread = Some(spawn(move || {
            block_on(Self::worker_task(main_sender, receiver, config))
        }));
        Self { sender, thread }
    }

    async fn worker_task(sender: Sender<CustomEvent>, receiver: Receiver<BackendCommand>, config: Arc<Mutex<Config>>) {
        let sync_with_config = config.lock_no_poison().restore_from_config;

        let executor = LocalExecutor::new();
        let mut monitor_map: BTreeMap<MonitorPath, (Rc<MonitorControl>, Task<()>)> = BTreeMap::new();
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
                            let saved = sync_with_config.then(|| config.lock_no_poison());
                            monitor_map.iter().for_each(|(p, (c, _))| {
                                c.query_brightness(
                                    saved
                                        .as_ref()
                                        .and_then(|c| c.monitors.get(p))
                                        .and_then(|s| s.saved_brightness)
                                )
                            });
                        }
                        BackendCommand::RefreshMonitors => {
                            trace!("Refreshing monitor list");
                            let mut current_monitors = Monitor::find_all()
                                .map_err(|err| log::warn!("Failed to enumerate monitors: {err}"))
                                .unwrap_or_default();
                            log::debug!("Skipping over unnamed monitors as they are likely integrated displays");
                            current_monitors.retain(|m| !m.name().is_empty());

                            let old_monitors = take(&mut monitor_map)
                                .into_keys()
                                .collect::<BTreeSet<_>>();

                            for path in &old_monitors {
                                if current_monitors.iter().all(|m| m.path() != path) {
                                    sender.send_ignore(CustomEvent::MonitorRemoved { path: path.clone() });
                                }
                            }

                            for monitor in current_monitors {
                                let path = monitor.path().clone();
                                if !old_monitors.contains(&path) {
                                    sender.send_ignore(CustomEvent::MonitorAdded { path: path.clone(), name: monitor.name().to_string() });
                                }
                                let control = Rc::new(MonitorControl::default());
                                let task = executor.spawn(monitor_task(monitor, sender.clone(), control.clone()));
                                monitor_map.insert(path, (control, task));
                            }

                            delayed_query = Some(Instant::now());
                        },
                        BackendCommand::QueryBrightness(Some(delay)) => {
                            delayed_query = delayed_query
                                .into_iter()
                                .chain([Instant::now() + delay])
                                .min();
                        }
                        BackendCommand::SetBrightness(p, v) => {
                            monitor_map
                                .get(&p)
                                .map(|(c, _)| c.request_brightness(v))
                                .unwrap_or_else(|| log::warn!("Unknown monitor {:?}", p));
                            if sync_with_config {
                                let mut config = config.lock_no_poison();
                                config.monitors.entry(p).or_default().saved_brightness = Some(v);
                                config.dirty = true;
                            }
                        }
                        BackendCommand::Stop => break,
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
        self.sender
            .send(command)
            .expect("Worker thread disappeared!");
    }
}

impl Drop for MonitorController {
    fn drop(&mut self) {
        let _ = self.sender.send(BackendCommand::Stop);
        if let Some(handle) = self.thread.take() {
            log::debug!("Waiting for worker thread to shutdown!");
            handle.join().expect("worker thread panic");
        }
    }
}



#[derive(Debug, Copy, Clone)]
enum MonitorCommand {
    QueryBrightness(Option<u32>),
    SetBrightness(u32, bool)
}

#[derive(Default)]
struct MonitorControl {
    command: Cell<Option<MonitorCommand>>,
    event: Event
}

impl MonitorControl {
    fn request_brightness(&self, value: u32) {
        self.command.set(Some(match self.command.take() {
            None => MonitorCommand::SetBrightness(value, false),
            Some(MonitorCommand::SetBrightness(_, notify)) => MonitorCommand::SetBrightness(value, notify),
            Some(MonitorCommand::QueryBrightness(_)) => MonitorCommand::QueryBrightness(Some(value))
        }));
        self.event.signal();
    }

    fn query_brightness(&self, target: Option<u32>) {
        self.command.set(Some(match self.command.take() {
            None => MonitorCommand::QueryBrightness(target),
            Some(MonitorCommand::QueryBrightness(value)) => MonitorCommand::QueryBrightness(target.or(value)),
            Some(MonitorCommand::SetBrightness(value, _)) => MonitorCommand::QueryBrightness(Some(target.unwrap_or(value)))
        }));
        self.event.signal();
    }
}

async fn monitor_task(monitor: Monitor, sender: Sender<CustomEvent>, control: Rc<MonitorControl>)  {
    let mut current_brightness = None;
    let mut cached_connection: Option<MonitorConnection> = None;

    loop {
        control.event.reset();
        match control.command.take() {
            None => {
                trace!("Closing connection to {} and going to sleep", monitor.name());
                cached_connection = None;
                control.event.wait().await;
            }
            Some(MonitorCommand::SetBrightness(value, _)) if Some(value) == current_brightness => {
                trace!("Brightness already at requested level {}", value);
                continue;
            }
            Some(command) => {
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
                    MonitorCommand::QueryBrightness(target) => {
                        trace!("Attempting to read brightness of {}", monitor.name());
                        match retry(|| connection.get_brightness()).await {
                            Ok((brightness, range)) => {
                                if range != (0..=100) {
                                    log::warn!("unexpected brightness range: {:?}", range);
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
                                        control
                                            .command
                                            .set(Some(MonitorCommand::SetBrightness(target, true)));
                                    }
                                    _ => {
                                        sender
                                            .send_ignore(CustomEvent::BrightnessChanged{ path: monitor.path().clone(), value: brightness });
                                    }
                                }
                            }
                            Err(err) => log::warn!("Failed to query brightness: {err}")
                        }
                    }
                    MonitorCommand::SetBrightness(value, notify) => {
                        log::trace!("Attempting to set brightness of {}", monitor.name());
                        let success = retry(|| connection.set_brightness(value))
                            .await
                            .map_err(|err| log::warn!("Failed to set brightness: {err}"))
                            .is_ok();
                        if success {
                            current_brightness = Some(value);
                        }
                        if notify {
                            if let Some(current) = current_brightness {
                                sender
                                    .send_ignore(CustomEvent::BrightnessChanged{ path: monitor.path().clone(), value: current });
                            }
                        }
                    }
                }
                Timer::after(Duration::from_millis(500)).await;
            }
        }
    }
}

async fn retry<R, E: Display, F: FnMut() -> std::result::Result<R, E>>(mut op: F) -> std::result::Result<R, E> {
    let mut tries = 0;
    let mut backoff_ms = 100;
    loop {
        match op() {
            Ok(result) => return Ok(result),
            Err(err) if tries <= 4 => {
                log::debug!("Retrying in {}: {}", backoff_ms, err);
                Timer::after(Duration::from_millis(backoff_ms)).await;
                tries += 1;
                backoff_ms *= 2;
            }
            Err(err) => return Err(err)
        }
    }
}
