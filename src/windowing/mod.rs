mod event_loop;
mod window;

use std::mem::size_of;

pub use event_loop::event_loop;
#[allow(unused_imports)]
pub use window::{Window, WindowBuilder};
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTOPRIMARY};

use crate::Result;

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
