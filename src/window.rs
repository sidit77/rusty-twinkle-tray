use std::any::Any;
use std::panic::{catch_unwind, resume_unwind};
use std::process::exit;
use windows::core::{PCWSTR, w};
use windows::Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::utils::error::{Result, TracedError};
use crate::{win_assert, WM_PANIC};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Destroy,
    Resize
}

impl Event {
    fn from_msg(message: u32) -> Option<Self> {
        match message {
            WM_DESTROY => Some(Self::Destroy),
            WM_SIZE | WM_SIZING => Some(Self::Resize),
            _ => None
        }
    }
}


pub trait WindowClass: Sized {
    const NAME: PCWSTR;

    fn initialize(parent: HWND) -> Result<Self>;
    fn on_event(&mut self, event: Event) -> Result<()>;

    fn register(instance: HMODULE) -> Result<()> {
        let class = WNDCLASSW {
            hCursor: HCURSOR::default(),
            hInstance: instance.into(),
            lpszClassName: Self::NAME,
            lpfnWndProc: Some(unsafe_wnd_proc::<Self>),
            ..Default::default()
        };
        win_assert!(unsafe { RegisterClassW(&class) } != 0);
        Ok(())
    }
}

pub struct Window {
    hwnd: HWND
}

impl Window {

    pub fn new<C: WindowClass>(instance: HMODULE) -> Result<Self> {
        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_NOREDIRECTIONBITMAP,
                C::NAME,
                w!("XAML Test"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT, CW_USEDEFAULT, 400, 400,
                None,
                None,
                instance,
                None
            )
        };

        let mut message = MSG::default();

        unsafe {
            while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() {
                check_for_failure(&message)?;
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }

        win_assert!(hwnd != HWND::default());

        unsafe {
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
        }

        Ok(Self {
            hwnd,
        })
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if IsWindow(self.hwnd).as_bool() {
                DestroyWindow(self.hwnd)
                    .unwrap_or_else(|err| log::warn!("Failed to destroy window: {}", err));
            }
        }
    }
}

fn safe_wnd_proc<W: WindowClass>(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT> {
    match message {
        WM_NCCREATE => {
            let window = W::initialize(hwnd)?;
            unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(Box::new(window)) as _) };
        },
        WM_NCDESTROY => {
            let this = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) as *mut W };
            if !this.is_null() {
                let _window = unsafe { Box::from_raw(this) };
            }
        },
        _ => {
            let this = unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut W).as_mut() };
            if let Some(this) = this {
                if let Some(event) = Event::from_msg(message) {
                    this.on_event(event)?;
                    return Ok(LRESULT::default());
                }
            }
        }
    }
    Ok(unsafe { DefWindowProcW(hwnd, message, wparam, lparam) })
}

unsafe extern "system" fn unsafe_wnd_proc<W: WindowClass>(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match catch_unwind(|| safe_wnd_proc::<W>(hwnd, message, wparam, lparam)) {
        Ok(Ok(lr)) => lr,
        Ok(Err(err)) => {
            post_failure(WndProcFailure::Error(err));
            LRESULT::default()
        }
        Err(panic_payload) => {
            post_failure(WndProcFailure::Panic(panic_payload));
            LRESULT::default()
        }
    }
}

enum WndProcFailure {
    Panic(Box<dyn Any + Send>),
    Error(TracedError)
}

fn post_failure(failure: WndProcFailure) {
    let ptr = Box::into_raw(Box::new(failure));
    unsafe {
        PostMessageW(None, WM_PANIC, None, LPARAM(ptr as _))
            .unwrap_or_else(|err| {
                log::error!("Failed to propagate failure: {}", err);
                exit(-1)
            });
    }
}

pub fn check_for_failure(msg: &MSG) -> Result<()> {
    if msg.message == WM_PANIC {
        let failure = *unsafe { Box::from_raw(msg.lParam.0 as *mut WndProcFailure)};
        match failure {
            WndProcFailure::Panic(payload) => resume_unwind(payload),
            WndProcFailure::Error(err) => Err(err)
        }
    } else {
        Ok(())
    }
}