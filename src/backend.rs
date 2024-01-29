use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};
use winit::event_loop::{EventLoop, EventLoopProxy};
use crate::{CustomEvent, Result};
use crate::config::Config;
use crate::monitors::{Monitor, MonitorPath};
use crate::utils::extensions::MutexExt;

#[derive(Debug, Clone)]
enum  Command {
    Stop,
    QueryBrightness(Option<Duration>),
    SetBrightness(MonitorPath, u32)
}

pub struct MonitorController {
    sender: Sender<Command>,
    thread: Option<JoinHandle<()>>
}

impl MonitorController {

    pub fn new(eventloop: &EventLoop<CustomEvent>, config: Arc<Mutex<Config>>) -> Self {
        let proxy = eventloop.create_proxy();
        let (sender, receiver) = channel();
        let _ = sender.send(Command::QueryBrightness(None));
        let thread = Some(spawn(move || worker_thread(proxy, receiver, config)));
        Self {
            sender,
            thread,
        }
    }
    pub fn create_proxy(&self) -> MonitorControllerProxy {
        MonitorControllerProxy(self.sender.clone())
    }

    pub fn refresh_brightness(&self) {
        self.send_command(Command::QueryBrightness(None));
    }

    pub fn shutdown(&self) {
        self.send_command(Command::Stop);
    }

    fn send_command(&self, command: Command) {
        self.sender.send(command)
            .expect("Worker thread disappeared!");
    }

}

pub struct MonitorControllerProxy(Sender<Command>);

impl MonitorControllerProxy {
    pub fn set_brightness(&self, monitor: MonitorPath, value: u32) {
        self.send_command(Command::SetBrightness(monitor, value));
    }

    pub fn refresh_brightness_in(&self, delay: Duration) {
        self.send_command(Command::QueryBrightness(Some(delay)));
    }

    fn send_command(&self, command: Command) {
        self.0.send(command)
            .expect("Worker thread disappeared!");
    }
}

impl Drop for MonitorController {
    fn drop(&mut self) {
        let _ = self.sender.send(Command::Stop);
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

fn worker_thread(sender: EventLoopProxy<CustomEvent>, receiver: Receiver<Command>, config: Arc<Mutex<Config>>) {
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
                Command::QueryBrightness(delay) => {
                    let time = Instant::now() + delay.unwrap_or(Duration::ZERO);
                    delayed_query = Some(delayed_query
                        .map(|t| t.min(time))
                        .unwrap_or(time));
                },
                Command::Stop => break,
                Command::SetBrightness(monitor, value) => {
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

#[track_caller]
fn retry<R, E: Display, F: FnMut() -> std::result::Result<R, E>>(mut op: F)-> std::result::Result<R, E> {
    let mut tries = 0;
    let mut backoff_ms = 100;
    loop {
        match op() {
            Ok(result) => return Ok(result),
            Err(err) if tries <= 4 => {
                log::debug!("Retrying in {}: {}", backoff_ms, err);
                sleep(Duration::from_millis(backoff_ms));
                tries += 1;
                backoff_ms *= 2;
            },
            Err(err) => return Err(err)
        }
    }
}
