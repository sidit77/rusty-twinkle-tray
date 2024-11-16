use std::time::Duration;
use loole::Sender;
use crate::backend::{BackendCommand, MonitorController};
use crate::monitors::MonitorPath;

pub struct MonitorControllerProxy(Sender<BackendCommand>);

impl MonitorControllerProxy {
    pub fn set_brightness(&self, monitor: MonitorPath, value: u32) {
        self.send_command(BackendCommand::SetBrightness(monitor, value));
    }

    pub fn refresh_brightness_in(&self, delay: Duration) {
        self.send_command(BackendCommand::QueryBrightness(Some(delay)));
    }

    fn send_command(&self, command: BackendCommand) {
        self.0.send(command).expect("Worker thread disappeared!");
    }
}

impl MonitorController {
    pub fn create_proxy(&self) -> MonitorControllerProxy {
        MonitorControllerProxy(self.sender.clone())
    }
}