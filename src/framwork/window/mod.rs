use std::cell::Cell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use futures_lite::Stream;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::framwork::window::class::WindowClass;
use crate::framwork::window::data::{WindowData, WindowEvents};
use crate::utils::error::Result;

mod class;
mod data;

pub struct Window {
    hwnd: HWND,
    data: Rc<WindowData>
}

impl Window {
    pub fn new() -> Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };



        let data = Rc::new(WindowData::default());

        let hwnd = WindowClass::create_instance(instance, WindowOptions::default(), data.clone())?;

        unsafe {
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
        }

        Ok(Self {
            hwnd,
            data,
        })
    }

    pub fn events(&self) -> WindowEvents<'_> {
        self.data.stream()
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


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Close,
    Resize
}

pub struct WindowOptions {
    title: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    style: WINDOW_STYLE,
    ex_style: WINDOW_EX_STYLE
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Hello World".to_string(),
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            style: WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            ex_style: WS_EX_NOREDIRECTIONBITMAP,
        }
    }
}