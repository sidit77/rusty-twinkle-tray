use windows::Win32::Foundation::{HANDLE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Power::{RegisterPowerSettingNotification, UnregisterPowerSettingNotification, HPOWERNOTIFY, POWERBROADCAST_SETTING};
use windows::Win32::System::SystemServices::GUID_SESSION_DISPLAY_STATUS;
use windows::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{DEVICE_NOTIFY_WINDOW_HANDLE, WM_DESTROY, WM_POWERBROADCAST};

use crate::Result;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum PowerEvent {
    MonitorOff,
    MonitorOn,
    MonitorDim
}

pub struct PowerEventRegistration(HPOWERNOTIFY);

impl PowerEventRegistration {
    pub fn register<F: FnMut(PowerEvent)>(hwnd: HWND, callback: F) -> Result<Self> {
        unsafe {
            SetWindowSubclass(
                hwnd,
                Some(Self::wnd_proc::<F>),
                super::POWER_SUBCLASS_ID,
                Box::into_raw(Box::new((true, callback))) as _
            )
            .ok()?;
        }
        let registration = unsafe { RegisterPowerSettingNotification(HANDLE(hwnd.0), &GUID_SESSION_DISPLAY_STATUS, DEVICE_NOTIFY_WINDOW_HANDLE.0)? };
        Ok(Self(registration))
    }

    unsafe extern "system" fn wnd_proc<F: FnMut(PowerEvent)>(
        hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, _id: usize, subclass_input_ptr: usize
    ) -> LRESULT {
        let callback_ptr = subclass_input_ptr as *mut (bool, F);
        match msg {
            WM_DESTROY => {
                drop(Box::from_raw(callback_ptr));
            }
            WM_POWERBROADCAST => {
                let data = &*(lparam.0 as *const POWERBROADCAST_SETTING);
                let (first, callback) = &mut *callback_ptr;
                let event = match data.Data[0] {
                    0 => Some(PowerEvent::MonitorOff),
                    1 => Some(PowerEvent::MonitorOn),
                    2 => Some(PowerEvent::MonitorDim),
                    _ => None
                };
                if let Some(event) = event {
                    if *first {
                        *first = false;
                    } else {
                        callback(event);
                    }
                }
            }
            _ => {}
        }
        DefSubclassProc(hwnd, msg, wparam, lparam)
    }
}

impl Drop for PowerEventRegistration {
    fn drop(&mut self) {
        unsafe {
            UnregisterPowerSettingNotification(self.0).unwrap_or_else(|err| log::warn!("Failed to remove power notification registration: {err}"));
        };
    }
}
