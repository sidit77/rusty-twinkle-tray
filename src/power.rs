use std::sync::Once;
use windows::core::{PCWSTR, w};
use windows::Win32::Foundation::{HANDLE, HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Power::{HPOWERNOTIFY, POWERBROADCAST_SETTING, RegisterPowerSettingNotification, UnregisterPowerSettingNotification};
use windows::Win32::System::SystemServices::GUID_SESSION_DISPLAY_STATUS;
use windows::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DestroyWindow, DEVICE_NOTIFY_WINDOW_HANDLE, HMENU, HWND_MESSAGE, IsWindow, RegisterClassW, WINDOW_EX_STYLE, WINDOW_STYLE, WM_DESTROY, WM_POWERBROADCAST, WNDCLASSW};
use crate::{Result, win_assert};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum  PowerEvent {
    MonitorOff,
    MonitorOn,
    MonitorDim
}

const POWER_SUBCLASS_ID: usize = 6666;

pub struct PowerStateListener {
    hwnd: HWND,
    registration: HPOWERNOTIFY
}

impl PowerStateListener {

    pub fn new<F: FnMut(PowerEvent)>(callback: F) -> Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                get_class_name(instance),
                PCWSTR::null(),
                WINDOW_STYLE::default(),
                0, 0,
                0, 0,
                HWND_MESSAGE,
                HMENU::default(),
                instance,
                None
            )
        };
        win_assert!(hwnd != HWND::default());
        unsafe { SetWindowSubclass(hwnd, Some(power_proc::<F>), POWER_SUBCLASS_ID, Box::into_raw(Box::new((true, callback))) as _).ok()?; }
        let registration = unsafe {
            RegisterPowerSettingNotification(HANDLE(hwnd.0),  &GUID_SESSION_DISPLAY_STATUS , DEVICE_NOTIFY_WINDOW_HANDLE.0)?
        };

        Ok(Self {
            hwnd,
            registration,
        })
    }

}

impl Drop for PowerStateListener {
    fn drop(&mut self) {
        unsafe {
            if IsWindow(self.hwnd).as_bool() {
                UnregisterPowerSettingNotification(self.registration)
                    .unwrap_or_else(|err| log::warn!("Failed to remove power notification registration: {err}"));
                DestroyWindow(self.hwnd)
                    .unwrap_or_else(|err| log::warn!("Failed to destroy message window: {err}"));
            }
        }
    }
}

unsafe extern "system" fn power_proc<F: FnMut(PowerEvent)>(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, _id: usize, subclass_input_ptr: usize) -> LRESULT {
    let callback_ptr = subclass_input_ptr as *mut (bool, F);
    match msg {
        WM_DESTROY => {
            drop(Box::from_raw(callback_ptr));
        },
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
        },
        _ => {}
    }
    DefSubclassProc(hwnd, msg, wparam, lparam)
}

fn get_class_name(hinstance: HMODULE) -> PCWSTR {
    static INITIALIZED: Once = Once::new();

    let class_name = w!("power_state_listener");

    INITIALIZED.call_once(|| {
        unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };
        unsafe { RegisterClassW(&wnd_class) };
    });

    class_name
}
