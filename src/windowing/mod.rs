mod event_loop;
mod window;

use std::mem::size_of;
use std::time::Duration;
use log::{trace, warn};
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTOPRIMARY};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_LBUTTON};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::Result;
use crate::runtime::Timer;

pub use event_loop::event_loop;
#[allow(unused_imports)]
pub use window::{Window, WindowLevel, WindowBuilder};


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

pub async fn poll_for_click_outside_of_rect(interval: Duration, rect: RECT) {
    trace!("Polling for click outside of rect {:?}", rect);
    let mut last = None;
    loop {
        let kc = unsafe { GetKeyState(VK_LBUTTON.0 as _) } & 1 != 0;
        if Some(kc) != last {
            if last.is_some() {
                let mut pt = POINT::default();
                if let Err(err) = unsafe { GetCursorPos(&mut pt) } {
                    warn!("Failed to get cursor position: {}", err);
                    continue;
                }
                trace!("Click at {}, {} (inside: {})", pt.x, pt.y, pt_in_rect(pt, rect));
                if !pt_in_rect(pt, rect) {
                    break;
                }
            }
            last = Some(kc);
        }
        Timer::after(interval).await;
    }
}

fn pt_in_rect(pt: POINT, rect: RECT) -> bool {
    pt.x >= rect.left && pt.x <= rect.right && pt.y >= rect.top && pt.y <= rect.bottom
}