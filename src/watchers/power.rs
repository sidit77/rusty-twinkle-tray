use log::trace;
use windows::Win32::Foundation::{HANDLE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Power::{RegisterPowerSettingNotification, UnregisterPowerSettingNotification, HPOWERNOTIFY, POWERBROADCAST_SETTING};
use windows::Win32::System::SystemServices::{GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS, GUID_SESSION_DISPLAY_STATUS};
use windows::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{
    DEVICE_NOTIFY_WINDOW_HANDLE, PBT_APMRESUMEAUTOMATIC, PBT_APMRESUMESUSPEND, PBT_APMSUSPEND, PBT_POWERSETTINGCHANGE, WM_DESTROY, WM_POWERBROADCAST
};

use crate::Result;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum PowerEvent {
    MonitorOff,
    MonitorOn,
    MonitorDim,
    Resume,
    /// Windows' own brightness value ("Display brightness" in the active power plan) changed.
    /// `initial` marks the notification sent right after registering, which only reports the current value.
    WindowsBrightness { value: u32, initial: bool }
}

pub struct PowerEventRegistration {
    display_status: HPOWERNOTIFY,
    brightness: HPOWERNOTIFY
}

impl PowerEventRegistration {
    pub fn register<F: FnMut(PowerEvent)>(hwnd: HWND, callback: F) -> Result<Self> {
        unsafe {
            SetWindowSubclass(
                hwnd,
                Some(Self::wnd_proc::<F>),
                super::POWER_SUBCLASS_ID,
                Box::into_raw(Box::new((true, true, callback))) as _
            )
            .ok()?;
        }
        let display_status =
            unsafe { RegisterPowerSettingNotification(HANDLE(hwnd.0), &GUID_SESSION_DISPLAY_STATUS, DEVICE_NOTIFY_WINDOW_HANDLE.0)? };
        let brightness =
            unsafe { RegisterPowerSettingNotification(HANDLE(hwnd.0), &GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS, DEVICE_NOTIFY_WINDOW_HANDLE.0)? };
        Ok(Self { display_status, brightness })
    }

    unsafe extern "system" fn wnd_proc<F: FnMut(PowerEvent)>(
        hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, _id: usize, subclass_input_ptr: usize
    ) -> LRESULT {
        let callback_ptr = subclass_input_ptr as *mut (bool, bool, F);
        match msg {
            WM_DESTROY => {
                drop(Box::from_raw(callback_ptr));
            }
            WM_POWERBROADCAST => {
                let (first, first_brightness, callback) = &mut *callback_ptr;
                match wparam.0 as u32 {
                    PBT_APMRESUMEAUTOMATIC => callback(PowerEvent::Resume),
                    PBT_POWERSETTINGCHANGE => {
                        let data = &*(lparam.0 as *const POWERBROADCAST_SETTING);
                        match data.PowerSetting {
                            GUID_SESSION_DISPLAY_STATUS => {
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
                            GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS if data.DataLength >= 4 => {
                                let value = (data.Data.as_ptr() as *const u32).read_unaligned();
                                let initial = std::mem::replace(first_brightness, false);
                                callback(PowerEvent::WindowsBrightness { value, initial });
                            }
                            e => trace!("Unknown power setting: {e:?}")
                        }
                    }
                    PBT_APMSUSPEND | PBT_APMRESUMESUSPEND => { /* Ignore */ }
                    e => trace!("Unknown power event: {e}")
                }
            }
            _ => {}
        }
        DefSubclassProc(hwnd, msg, wparam, lparam)
    }
}

impl Drop for PowerEventRegistration {
    fn drop(&mut self) {
        for registration in [self.display_status, self.brightness] {
            unsafe {
                UnregisterPowerSettingNotification(registration)
                    .unwrap_or_else(|err| log::warn!("Failed to remove power notification registration: {err}"));
            };
        }
    }
}
