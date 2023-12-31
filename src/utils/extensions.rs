use std::mem::size_of;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::monitor::MonitorHandle;
use winit::platform::windows::MonitorHandleExtWindows;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::Window;
use crate::Result;

pub trait WindowExt {
    fn hwnd(&self) -> HWND;
}

impl WindowExt for Window {
    fn hwnd(&self) -> HWND {
        let handle = self.window_handle().unwrap().as_raw();
        match handle {
            RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get()),
            _ => unreachable!()
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