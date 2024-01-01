use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};
use std::time::{Duration, Instant};
use winit::event_loop::{EventLoop, EventLoopProxy};
use crate::{CustomEvent, Result};
use crate::config::Config;
use crate::monitors::{Monitor, MonitorPath};
use crate::utils::extensions::MutexExt;

#[derive(Debug, Clone)]
enum  Command {
    Stop,
    QueryBrightness,
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
        let _ = sender.send(Command::QueryBrightness);
        let thread = Some(spawn(move || worker_thread(proxy, receiver, config)
            .unwrap_or_else(|err| log::error!("Worker thread failure: {err}"))));
        Self {
            sender,
            thread,
        }
    }
    pub fn create_proxy(&self) -> MonitorControllerProxy {
        MonitorControllerProxy(self.sender.clone())
    }

    pub fn refresh_brightness(&self) {
        self.send_command(Command::QueryBrightness);
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

fn worker_thread(sender: EventLoopProxy<CustomEvent>, receiver: Receiver<Command>, config: Arc<Mutex<Config>>) -> Result<()> {
    let send = move |event| sender
        .send_event(event)
        .unwrap_or_else(|_| log::warn!("Eventloop closed"));

    let sync_with_config = config.lock_no_poison().restore_from_config;

    let monitors = Monitor::find_all()?;
    for monitor in &monitors {
        send(CustomEvent::RegisterMonitor(monitor.name().to_string(), monitor.path().clone()));
    }

    let mut brightness_map = BTreeMap::<MonitorPath, MonitorBrightness>::new();

    loop {
        let next_update = brightness_map
            .values()
            .filter_map(MonitorBrightness::update_in)
            .min()
            .unwrap_or(Duration::MAX);
        match receiver.recv_timeout(next_update) {
            Ok(command) => match command {
                Command::QueryBrightness => {
                    for monitor in &monitors {
                        let connection = monitor.open()?;
                        let (mut brightness, range) = connection.get_brightness()?;
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
                                    connection.set_brightness(saved)?;
                                    entry.last_update = Some(Instant::now());
                                }
                            }
                        }
                        entry.set_current(brightness, false);
                        send(CustomEvent::UpdateBrightness(monitor.path().clone(), brightness));
                    }
                }
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
                                    monitor.open()?.set_brightness(brightness)?;
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

    Ok(())
}
