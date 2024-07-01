use std::mem::{MaybeUninit, size_of};
use std::sync::{Arc, Mutex, MutexGuard, Weak};
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

pub trait ChannelExt<T> {
    fn filter_send_ignore(&self, msg: Option<T>);
}

impl<T> ChannelExt<T> for flume::Sender<T> {
    #[track_caller]
    fn filter_send_ignore(&self, msg: Option<T>) {
        if let Some(msg) = msg {
            self.send(msg).unwrap_or_else(|err| log::warn!("Failed to send message: {}", err));
        }
    }
}

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

pub trait ArcExt<T> {
    fn try_new_cyclic<F, E>(data_fn: F) -> std::result::Result<Arc<T>, E>
        where
            F: FnOnce(&Weak<T>) -> std::result::Result<T, E>;
}

impl<T> ArcExt<T> for Arc<T> {
    fn try_new_cyclic<F, E>(data_fn: F) -> std::result::Result<Arc<T>, E>
        where F: FnOnce(&Weak<T>) -> std::result::Result<T, E> {

        // hopefully this is safe
        let mut error: std::result::Result<(), E> = Ok(());
        let arc = Arc::<MaybeUninit<T>>::new_cyclic(|inner| {
             match data_fn(unsafe { std::mem::transmute(inner) }) {
                 Ok(r) => MaybeUninit::new(r),
                 Err(e) => {
                     error = Err(e);
                     MaybeUninit::uninit()
                 }
             }
        });
        error.map(|_| {
            let md_self = Arc::into_raw(arc);
            unsafe { Arc::<T>::from_raw(md_self.cast()) }
        })
        //Ok(Arc::new_cyclic(|i| data_fn(i).unwrap_or_else(|_| panic!("Closure panic"))))
    }
}