use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};
use async_executor::LocalExecutor;
use async_io::{block_on, Timer};
use either::Either;
use flume::{Sender, unbounded};
use futures_lite::{FutureExt, StreamExt};
use futures_lite::stream::iter;
use winit::event_loop::{EventLoop, EventLoopProxy};
use crate::{CustomEvent, Result};
use crate::config::Config;
use crate::monitors::{Monitor, MonitorPath};
use crate::utils::error::TracedError;
use crate::utils::extensions::MutexExt;
use crate::utils::runtime::{Sink, SinkExt, Source};

#[derive(Debug, Clone)]
enum BackendCommand {
    Stop,
    QueryBrightness(Option<Duration>),
    SetBrightness(MonitorPath, u32)
}

#[derive(Debug, Clone)]
pub enum BackendEvent {
    RegisterMonitor(String, MonitorPath),
    UpdateBrightness(MonitorPath, u32)
}

pub struct MonitorController {
    sender: Sender<BackendCommand>,
    thread: Option<JoinHandle<()>>
}

impl MonitorController {

    pub fn new(eventloop: &EventLoop<CustomEvent>, config: Arc<Mutex<Config>>) -> Self {
        let proxy = eventloop.create_proxy();
        let (sender, receiver) = unbounded();
        let _ = sender.send(BackendCommand::QueryBrightness(None));
        let thread = Some(spawn(move || block_on(worker_task(proxy.map(CustomEvent::Backend), receiver, config))));
        Self {
            sender,
            thread,
        }
    }
    pub fn create_proxy(&self) -> MonitorControllerProxy {
        MonitorControllerProxy(self.sender.clone())
    }

    pub fn refresh_brightness(&self) {
        self.send_command(BackendCommand::QueryBrightness(None));
    }

    pub fn shutdown(&self) {
        self.send_command(BackendCommand::Stop);
    }

    fn send_command(&self, command: BackendCommand) {
        self.sender.send(command)
            .expect("Worker thread disappeared!");
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

    fn send_command(&self, command: BackendCommand) {
        self.0.send(command)
            .expect("Worker thread disappeared!");
    }
}

impl Drop for MonitorController {
    fn drop(&mut self) {
        let _ = self.sender.send(BackendCommand::Stop);
        if let Some(handle) = self.thread.take() {
            log::debug!("Waiting for worker thread to shutdown!");
            handle
                .join()
                .expect("worker thread panic");
        }
    }
}

#[derive(Debug, Default)]
struct MonitorBrightness {
    current: Option<u32>,
    requested: Option<u32>,
    last_update: Option<Instant>
}

impl MonitorBrightness {
    const UPDATE_INTERVAL: Duration = Duration::from_millis(500);

    pub fn can_update(&self) -> bool {
        match self.last_update {
            None => true,
            Some(last) => last.elapsed() >= Self::UPDATE_INTERVAL
        }
    }

    pub fn set_request(&mut self, request: u32) {
        self.requested = (self.current != Some(request))
            .then_some(request)
    }

    pub fn set_current(&mut self, brightness: u32, updated: bool) {
        self.current = Some(brightness);
        if updated {
            self.last_update = Some(Instant::now());
            self.requested = None;
        }
    }

    pub fn update_in(&self) -> Option<Duration> {
        self.requested
            .map(|_| self
                .last_update
                .map(|last| Self::UPDATE_INTERVAL.saturating_sub(last.elapsed()))
                .unwrap_or(Duration::ZERO))
    }

}

#[derive(Debug, Copy, Clone)]
enum MonitorCommand {
    QueryBrightness,
    UpdateBrightness(u32)
}


async fn worker_task<S, R>(sender: S, receiver: R, config: Arc<Mutex<Config>>)
    where S: Sink<BackendEvent> + Clone,
          R: Source<BackendCommand>
{
    let sync_with_config = config.lock_no_poison().restore_from_config;

    let executor = LocalExecutor::new();
    let monitor_map = {
        let mut monitors = Monitor::find_all()
            .map_err(|err| log::warn!("Failed to enumerate monitors: {err}"))
            .unwrap_or_default();
        log::debug!("Skipping over unnamed monitors as they are likely integrated displays");
        monitors.retain(|m| !m.name().is_empty());
        iter(monitors.into_iter())
            .then(|m| async {
                let path = m.path().clone();
                sender.send(BackendEvent::RegisterMonitor(m.name().to_string(), path.clone())).await;
                let (ms, mr) = unbounded();
                executor.spawn(monitor_task(m, sender.clone(), mr, config.clone())).detach();
                (path, ms)
            })
            .collect::<BTreeMap<_, _>>()
            .await
    };

    let mut delayed_query: Option<Instant> = None;

    executor.run(async {
        loop {
            let query_event = async {
                delayed_query
                    .map(Timer::at)
                    .unwrap_or_else(Timer::never)
                    .await;
                BackendCommand::QueryBrightness(None)
            };
            let command_event = async {
                receiver
                    .recv()
                    .await
                    .expect("Controller disappeared")
            };
            match query_event.or(command_event).await {
                BackendCommand::QueryBrightness(None) => {
                    delayed_query.take();
                    monitor_map
                        .iter()
                        .for_each(|(p, s)| s
                            .send(MonitorCommand::QueryBrightness)
                            .unwrap_or_else(|_| log::error!("Task for monitor {:?} disappeared", p)));
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
                        .map(|s| s
                            .send(MonitorCommand::UpdateBrightness(v))
                            .unwrap_or_else(|_| log::error!("Task for monitor {:?} disappeared", p)))
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
    }).await;
}

async fn monitor_task<S, R>(monitor: Monitor, sender: S, receiver: R, config: Arc<Mutex<Config>>)
    where S: Sink<BackendEvent>,
          R: Source<MonitorCommand>
{
    let sync_with_config = config.lock_no_poison().restore_from_config;

    let mut state = MonitorBrightness::default();

    loop {
        let update_event = async {
            state
                .update_in()
                .map(Timer::after)
                .unwrap_or_else(Timer::never)
                .await;
            Either::Left(())
        };
        let cmd_event = async {
            Either::Right(receiver
                .recv()
                .await)
        };
        match update_event.or(cmd_event).await {
            Either::Left(()) => {
                if let Some(brightness) = state.requested {
                    if state.can_update() {
                        if state.current == Some(brightness) {
                            log::trace!("Brightness already at requested level: {:?}", state);
                            continue;
                        }
                        log::trace!("Setting brightness for {} to {}", monitor.name(), brightness);
                        async {
                            let connection = retry(|| monitor.open()).await?;
                            retry(|| connection.set_brightness(brightness)).await?;
                            Ok(())
                        }.await.unwrap_or_else(|err: TracedError| log::warn!("Failed to set brightness: {err}"));

                        state.set_current(brightness, true);
                    }
                }
            }
            Either::Right(cmd) => match cmd {
                Some(MonitorCommand::QueryBrightness) => {
                    let brightness = async {
                        let connection = retry(|| monitor.open()).await?;
                        let (mut brightness, range) = retry(|| connection.get_brightness()).await?;
                        if range != (0..=100) {
                            log::warn!("unexpected brightness range: {:?}", range);
                        }
                        if sync_with_config {
                            let saved_brightness = config
                                .lock_no_poison()
                                .monitors
                                .get(monitor.path())
                                .and_then(|s| s.saved_brightness);
                            //log::trace!("{}: current: {}, saved: {:?}", monitor.name(), brightness, saved_brightness);
                            if let Some(saved) = saved_brightness {
                                if saved != brightness {
                                    log::info!("Restoring saved brightness for {} (current: {}, saved: {})", monitor.name(), brightness, saved);
                                    brightness = saved;
                                    retry(|| connection.set_brightness(saved)).await?;
                                    state.last_update = Some(Instant::now());
                                }
                            }
                        }
                        state.set_current(brightness, false);
                        Ok::<u32, TracedError>(brightness)
                    };
                    match brightness.await {
                        Ok(brightness) => {
                            sender.send(BackendEvent::UpdateBrightness(monitor.path().clone(), brightness)).await;
                        },
                        Err(err) => log::warn!("Failed to query brightness: {err}")
                    }
                }
                Some(MonitorCommand::UpdateBrightness(v)) => {
                    state.set_request(v);
                },
                None => break
            }
        }
    }
}

/*
fn worker_thread(sender: EventLoopProxy<CustomEvent>, receiver: Receiver<BackendCommand>, config: Arc<Mutex<Config>>) {
    let send = move |event| sender
        .send_event(event)
        .unwrap_or_else(|_| log::warn!("Eventloop closed"));

    let sync_with_config = config.lock_no_poison().restore_from_config;

    let mut monitors = Monitor::find_all()
        .map_err(|err| log::warn!("Failed to enumerate monitors: {err}"))
        .unwrap_or_default();
    log::debug!("Skipping over unnamed monitors as they are likely integrated displays");
    monitors.retain(|m| !m.name().is_empty());
    for monitor in &monitors {
        send(CustomEvent::RegisterMonitor(monitor.name().to_string(), monitor.path().clone()));
    }
    //send(CustomEvent::RegisterMonitor(String::from("Test"), MonitorPath(Arc::from(Path::new("DUMMY_ID")))));
    //send(CustomEvent::RegisterMonitor(String::from("Test 2"), MonitorPath(Arc::from(Path::new("DUMMY_ID2")))));

    let mut brightness_map = BTreeMap::<MonitorPath, MonitorBrightness>::new();
    let mut delayed_query: Option<Instant> = None;

    loop {
        let next_update = brightness_map
            .values()
            .filter_map(MonitorBrightness::update_in)
            .min()
            .unwrap_or(Duration::MAX);
        let next_query = delayed_query
            .map(|t| t.saturating_duration_since(Instant::now()))
            .unwrap_or(Duration::MAX);
        match receiver.recv_timeout(next_update.min(next_query)) {
            Ok(command) => match command {
                BackendCommand::QueryBrightness(delay) => {
                    let time = Instant::now() + delay.unwrap_or(Duration::ZERO);
                    delayed_query = Some(delayed_query
                        .map(|t| t.min(time))
                        .unwrap_or(time));
                },
                BackendCommand::Stop => break,
                BackendCommand::SetBrightness(monitor, value) => {
                    brightness_map
                        .entry(monitor.clone())
                        .or_default()
                        .set_request(value);
                    if sync_with_config {
                        let mut config = config.lock_no_poison();
                        config.monitors.entry(monitor).or_default().saved_brightness = Some(value);
                        config.dirty = true;
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                if delayed_query.is_some_and(|t| t <= Instant::now()) {
                    delayed_query = None;
                    for monitor in &monitors {
                        match query_brightness(&monitor, &mut brightness_map, &config, sync_with_config) {
                            Ok(brightness) => send(CustomEvent::UpdateBrightness(monitor.path().clone(), brightness)),
                            Err(err) => log::warn!("Failed to query brightness: {err}")
                        }
                    }
                }
                for (path, entry) in brightness_map.iter_mut() {
                    if let Some(brightness) = entry.requested {
                        if entry.can_update() {
                            if entry.current == Some(brightness) {
                                log::trace!("Brightness already at requested level: {:?}", entry);
                                continue;
                            }
                            match monitors.iter().find(|m| m.path() == path) {
                                None => log::debug!("Couldn't find monitor"),
                                Some(monitor) => {
                                    log::trace!("Setting brightness for {} to {}", monitor.name(), brightness);
                                    retry(|| monitor.open())
                                        .and_then(|conn| retry(|| conn.set_brightness(brightness)))
                                        .unwrap_or_else(|err| log::warn!("Failed to set brightness: {err}"));
                                    entry.set_current(brightness, true);
                                }
                            }
                        }
                    }
                }
            },
            Err(RecvTimeoutError::Disconnected) => panic!("Controller disappeared")
        }
    }
}

fn query_brightness(
    monitor: &Monitor,
    brightness_map: &mut BTreeMap::<MonitorPath, MonitorBrightness>,
    config: &Mutex<Config>,
    sync_with_config: bool
) -> Result<u32> {
    let connection = retry(|| monitor.open())?;
    let (mut brightness, range) = retry(|| connection.get_brightness())?;
    if range != (0..=100) {
        log::warn!("unexpected brightness range: {:?}", range);
    }
    let entry = brightness_map
        .entry(monitor.path().clone())
        .or_default();
    if sync_with_config {
        let saved_brightness = config
            .lock_no_poison()
            .monitors
            .get(monitor.path())
            .and_then(|s| s.saved_brightness);
        //log::trace!("{}: current: {}, saved: {:?}", monitor.name(), brightness, saved_brightness);
        if let Some(saved) = saved_brightness {
            if saved != brightness {
                log::info!("Restoring saved brightness for {} (current: {}, saved: {})", monitor.name(), brightness, saved);
                brightness = saved;
                retry(|| connection.set_brightness(saved))?;
                entry.last_update = Some(Instant::now());
            }
        }
    }
    entry.set_current(brightness, false);
    Ok(brightness)
}

 */

#[track_caller]
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
            },
            Err(err) => return Err(err)
        }
    }
}
