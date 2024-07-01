mod event_loop;
mod window;

use std::mem::size_of;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTOPRIMARY, MonitorFromPoint, MONITORINFO};
use crate::Result;

pub use event_loop::event_loop;
pub use window::ProxyWindow;

pub fn get_primary_work_area() -> Result<RECT> {
    unsafe {
        let primary_monitor = MonitorFromPoint(POINT::default(), MONITOR_DEFAULTTOPRIMARY);
        let mut mi = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        GetMonitorInfoW(primary_monitor, &mut mi).ok()?;
        Ok(mi.rcWork)
    }
}