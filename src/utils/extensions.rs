use std::mem::size_of;
use std::sync::{Mutex, MutexGuard};
use windows::Win32::Foundation::{COLORREF, HWND, RECT};
use windows::Win32::Graphics::Dwm::{DWM_SYSTEMBACKDROP_TYPE, DWMSBT_NONE, DwmSetWindowAttribute, DWMWA_BORDER_COLOR, DWMWA_COLOR_NONE, DWMWA_SYSTEMBACKDROP_TYPE};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};
use windows_version::OsVersion;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::monitor::MonitorHandle;
use winit::platform::windows::MonitorHandleExtWindows;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::Window;
use crate::Result;

#[repr(transparent)]
pub struct BorderColor(COLORREF);

impl BorderColor {
    //pub const DEFAULT: BorderColor = BorderColor::from_u32(DWMWA_COLOR_DEFAULT);
    pub const NONE: BorderColor = BorderColor::from_u32(DWMWA_COLOR_NONE);

    const fn from_u32(color: u32) -> Self {
        Self(COLORREF(color))
    }
}

pub trait WindowExt {
    fn hwnd(&self) -> HWND;
    fn set_border_color(&self, color: BorderColor);
}

impl WindowExt for Window {
    fn hwnd(&self) -> HWND {
        let handle = self.window_handle().unwrap().as_raw();
        match handle {
            RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get()),
            _ => unreachable!()
        }
    }

    fn set_border_color(&self, color: BorderColor) {
        if OsVersion::current().build >= 22000 {
            unsafe {
                DwmSetWindowAttribute(self.hwnd(), DWMWA_BORDER_COLOR, &color as *const _ as _, 4)
                    .unwrap_or_else(|err|  log::debug!("Failed to set window border color: {err}"));
                DwmSetWindowAttribute(self.hwnd(), DWMWA_SYSTEMBACKDROP_TYPE,
                                      &DWMSBT_NONE as *const _ as _, size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32)
                    .unwrap_or_else(|err|  log::debug!("Failed to set window border color: {err}"));
            }
        }
    }
}

pub trait MonitorHandleExt {
    fn get_work_area(&self) -> Result<RECT>;
}

impl MonitorHandleExt for MonitorHandle {
    fn get_work_area(&self) -> Result<RECT> {
        let mut mi = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        unsafe {
            GetMonitorInfoW(HMONITOR(self.hmonitor()), &mut mi).ok()?
        };
        Ok(mi.rcWork)
    }
}

pub trait EventLoopExt<T> {
    fn run_result<F>(self, event_handler: F) -> Result<()>
        where F: FnMut(Event<T>, &EventLoopWindowTarget<T>) -> Result<()>;
}

impl<T> EventLoopExt<T> for EventLoop<T> {
    fn run_result<F>(self, mut event_handler: F) -> Result<()>
        where F: FnMut(Event<T>, &EventLoopWindowTarget<T>) -> Result<()>
    {
        let mut result1 = Ok(());
        {
            let result1 = &mut result1;
            self.run(move |event, target| {
                if let Err(err) = event_handler(event, target) {
                    *result1 = Err(err);
                    target.exit();
                }
            })?;
        }
        result1
    }
}

pub trait MutexExt {
    type Guard<'a> where Self: 'a;

    fn lock_no_poison(&self) -> Self::Guard<'_>;
}

impl<T> MutexExt for Mutex<T> {
    type Guard<'a> = MutexGuard<'a, T> where T: 'a;

    fn lock_no_poison(&self) -> Self::Guard<'_> {
        self.lock().unwrap_or_else(|err| err.into_inner())
    }
}