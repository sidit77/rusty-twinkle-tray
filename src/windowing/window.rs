use std::mem::size_of;
use std::sync::Once;

use windows::core::{w, ComInterface, TryIntoParam, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HMODULE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use windows_ext::UI::Xaml::Hosting::DesktopWindowXamlSource;
use windows_ext::UI::Xaml::UIElement;

use crate::{win_assert, Result};

#[derive(Debug, Clone, Default)]
pub struct WindowBuilder {
    hidden: bool,
}

impl WindowBuilder {
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn build(self) -> Result<Window> {
        Window::new(self)
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum WindowLevel {
    AlwaysOnTop,
    #[default]
    Normal
}

impl From<WindowLevel> for HWND {
    fn from(value: WindowLevel) -> Self {
        match value {
            WindowLevel::AlwaysOnTop => HWND_TOPMOST,
            WindowLevel::Normal => HWND_TOP,
        }
    }
}

pub struct Window {
    pub hwnd: HWND,
    source: DesktopWindowXamlSource
}

impl Window {
    const CLASS_NAME: PCWSTR = w!("rusty-twinkle-tray.window");
    fn new(builder: WindowBuilder) -> crate::Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };
        static REGISTER_WINDOW_CLASS: Once = Once::new();
        REGISTER_WINDOW_CLASS.call_once(|| {
            Self::register(instance).unwrap_or_else(|err| log::warn!("Failed to register window class: {}", err));
        });

        let mut ex_style = WS_EX_NOREDIRECTIONBITMAP | WS_EX_NOACTIVATE; // | WS_EX_TOPMOST;
        if builder.hidden {
            ex_style |= WS_EX_LAYERED;
        }

        let hwnd = unsafe {
            CreateWindowExW(
                ex_style,
                Self::CLASS_NAME,
                w!("XAML Island Window"),
                WS_POPUP,
                0,
                0,
                10,
                10,
                None,
                None,
                instance,
                None
            )
        };
        win_assert!(hwnd != HWND::default());
        if builder.hidden {
            unsafe {
                SetLayeredWindowAttributes(hwnd, COLORREF::default(), 0, LWA_ALPHA)?;
            }
        }

        let desktop_source = DesktopWindowXamlSource::new()?;
        let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
        unsafe {
            interop.AttachToWindow(hwnd)?;
            let island = interop.WindowHandle()?;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, island.0);
            sync_size(hwnd, island)?;
        }

        Ok(Self {
            hwnd,
            source: desktop_source
        })
    }

    pub fn set_foreground(&self) -> bool {
        unsafe { SetForegroundWindow(self.hwnd).as_bool() }
    }

    pub fn set_window_pos(&self, order: Option<WindowLevel>, pos: Option<(i32, i32)>, size: Option<(i32, i32)>, visible: Option<bool>) {
        let (x, y) = pos.unwrap_or_default();
        let (w, h) = size.unwrap_or_default();
        let after = HWND::from(order.unwrap_or_default());
        let mut flags = SET_WINDOW_POS_FLAGS::default();
        if pos.is_none() {
            flags |= SWP_NOMOVE;
        }
        if size.is_none() {
            flags |= SWP_NOSIZE;
        }
        if order.is_none() {
            flags |= SWP_NOZORDER;
        }
        if let Some(visible) = visible {
            flags |= if visible { SWP_SHOWWINDOW } else { SWP_HIDEWINDOW };
        }
        unsafe {
            SetWindowPos(self.hwnd, after, x, y, w, h, flags)
                .unwrap_or_else(|err| log::warn!("Failed to set window position: {}", err));
        }
    }

    //pub fn set_visible(&self, visible: bool) {
    //    unsafe {
    //        match visible {
    //            true => SetWindowPos(self.hwnd, HWND_TOP, 0, 0, 0, 0, SWP_SHOWWINDOW | SWP_NOSIZE | SWP_NOMOVE),
    //            false => SetWindowPos(
    //                self.hwnd,
    //                HWND_TOPMOST,
    //                0,
    //                0,
    //                0,
    //                0,
    //                SWP_HIDEWINDOW | SWP_NOZORDER | SWP_NOSIZE | SWP_NOMOVE
    //            )
    //        }
    //        .unwrap_or_else(|err| log::warn!("Failed to set window visibility: {}", err));
    //    }
    //}

    pub fn focus(&self) {
        unsafe {
            SetFocus(self.hwnd);
        }
    }

    pub fn dpi(&self) -> f32 {
        unsafe {
            GetDpiForWindow(self.hwnd) as f32 / USER_DEFAULT_SCREEN_DPI as f32
        }
    }

    pub fn set_content<T: TryIntoParam<UIElement>>(&self, content: T) -> Result<()> {
        self.source.SetContent(content)?;
        Ok(())
    }

    fn register(instance: HMODULE) -> crate::Result<()> {
        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(Self::wnd_proc),
            hInstance: instance.into(),
            lpszClassName: Self::CLASS_NAME,
            ..Default::default()
        };
        win_assert!(unsafe { RegisterClassExW(&class) } != 0);
        Ok(())
    }

    unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_NCCREATE => {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            }
            WM_SIZING | WM_SIZE => {
                let island = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
                if island != 0 {
                    sync_size(hwnd, HWND(island)).unwrap_or_else(|err| log::warn!("Failed to sync window size: {}", err));
                }
            }
            WM_DESTROY => {
                PostQuitMessage(0);
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if IsWindow(self.hwnd).as_bool() {
                DestroyWindow(self.hwnd).unwrap_or_else(|err| log::warn!("Failed to destroy window: {}", err));
            }
        }
    }
}

fn sync_size(hwnd: HWND, island: HWND) -> Result<()> {
    unsafe {
        let mut rect = RECT::default();
        GetClientRect(hwnd, &mut rect)?;
        SetWindowPos(
            island,
            HWND::default(),
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_SHOWWINDOW
        )?;
        Ok(())
    }
}
