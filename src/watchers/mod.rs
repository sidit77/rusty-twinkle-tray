mod power;
mod display_change;

use std::sync::Once;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DestroyWindow, IsWindow, RegisterClassW, HMENU, WINDOW_STYLE, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT};

use crate::Result;
use display_change::DisplayChangeEventRegistration;
use power::PowerEventRegistration;

pub use power::PowerEvent;

pub struct EventWatcher {
    hwnd: HWND,
    power_state_listener: Option<PowerEventRegistration>,
    display_change_listener: Option<DisplayChangeEventRegistration>
}

const POWER_SUBCLASS_ID: usize = 6666;
const DISPLAY_CHANGE_SUBCLASS_ID: usize = 6667;

impl EventWatcher {
    pub fn new() -> Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };
        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW,
                Self::build_class(instance),
                PCWSTR::null(),
                WINDOW_STYLE::default(),
                0,
                0,
                0,
                0,
                HWND::default(),
                HMENU::default(),
                instance,
                None
            )
        };
        Ok(Self { hwnd, power_state_listener: None, display_change_listener: None })
    }

    pub fn on_power_event<F: FnMut(PowerEvent)>(mut self, callback: F) -> Result<Self> {
        assert!(self.power_state_listener.is_none(), "Power event listener already registered");
        self.power_state_listener = Some(PowerEventRegistration::register(self.hwnd, callback)?);
        Ok(self)
    }

    pub fn on_display_change<F: FnMut()>(mut self, callback: F) -> Result<Self> {
        assert!(self.display_change_listener.is_none(), "Display change listener already registered");
        self.display_change_listener = Some(DisplayChangeEventRegistration::register(self.hwnd, callback)?);
        Ok(self)
    }

    fn build_class(hinstance: HMODULE) -> PCWSTR {
        static INITIALIZED: Once = Once::new();

        let class_name = w!("tt_event_watcher");

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

}

impl Drop for EventWatcher {
    fn drop(&mut self) {
        drop(self.power_state_listener.take());
        drop(self.display_change_listener.take());
        unsafe {
            if IsWindow(self.hwnd).as_bool() {
                DestroyWindow(self.hwnd).unwrap_or_else(|err| log::warn!("Failed to destroy message window: {err}"));
            }
        }
    }
}
