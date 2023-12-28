use std::cell::Cell;
use std::rc::Rc;
use std::sync::Once;
use windows::core::{PCWSTR, w};
use windows::Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::framwork::window::{Event, WindowOptions};
use crate::framwork::window::data::WindowData;
use crate::utils::error::Result;
use crate::win_assert;

pub struct WindowClass {
    data: Rc<WindowData>
}

impl WindowClass {
    const NAME: PCWSTR = w!("rusty-twinkle-tray.window");

    pub fn create_instance(instance: HMODULE, _options: WindowOptions, data: Rc<WindowData>) -> Result<HWND> {
        static REGISTER_WINDOW_CLASS: Once = Once::new();
        REGISTER_WINDOW_CLASS.call_once(|| {
            WindowClass::register(instance)
                .unwrap_or_else(|err| log::warn!("Failed to register window class: {}", err));
        });

        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_NOREDIRECTIONBITMAP,
                WindowClass::NAME,
                w!("XAML Test"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT, CW_USEDEFAULT, 400, 400,
                None,
                None,
                instance,
                Some(Rc::into_raw(data) as _)
            )
        };

        win_assert!(hwnd != HWND::default());

        Ok(hwnd)
    }

    fn register(instance: HMODULE) -> Result<()> {
        let class = WNDCLASSW {
            hCursor: HCURSOR::default(),
            hInstance: instance.into(),
            lpszClassName: Self::NAME,
            lpfnWndProc: Some(Self::wnd_proc),
            ..Default::default()
        };
        win_assert!(unsafe { RegisterClassW(&class) } != 0);
        Ok(())
    }

    unsafe extern "system" fn wnd_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_NCCREATE => {
                let create = &*(lparam.0 as *const CREATESTRUCTW);
                let data = Rc::from_raw(create.lpCreateParams as *const WindowData);
                let data = Box::new(Self {
                    data,
                });
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _);
            },
            WM_NCDESTROY => {
                let this = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) as *mut Self };
                if !this.is_null() {
                    drop(unsafe { Box::from_raw(this) });
                }
            },
            _ => {
                let this = unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self).as_mut() };
                if let Some(this) = this {
                    if let Some(event) = parse_event(message) {
                        this.data.push_event(event);
                        return LRESULT::default();
                    }
                }
            }
        }
        DefWindowProcW(hwnd, message, wparam, lparam)
    }
}

fn parse_event(message: u32) -> Option<Event> {
    match message {
        WM_CLOSE => Some(Event::Close),
        WM_SIZE | WM_SIZING => Some(Event::Resize),
        _ => None
    }
}