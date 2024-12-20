use std::cell::Cell;
use std::mem::size_of;
use std::sync::Once;

use log::trace;
use windows::core::{h, w, ComInterface, TryIntoParam, HSTRING, PCWSTR};
use windows::Win32::Foundation::{BOOL, COLORREF, HANDLE, HMODULE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMWA_CAPTION_COLOR, DWMWA_COLOR_NONE, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE,
    DWM_SYSTEMBACKDROP_TYPE
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use windows_ext::UI::Xaml::Hosting::DesktopWindowXamlSource;
use windows_ext::UI::Xaml::UIElement;

use crate::{win_assert, Result};

#[derive(Default)]
pub struct WindowBuilder {
    hidden: bool,
    close_handler: Option<Box<dyn FnMut() + 'static>>,
    title: Option<HSTRING>,
    icon: Option<u16>,
    x: Option<i32>,
    y: Option<i32>,
    w: Option<i32>,
    h: Option<i32>
}

impl WindowBuilder {
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn with_close_handler<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.close_handler = Some(Box::new(handler));
        self
    }

    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn with_size(mut self, w: i32, h: i32) -> Self {
        self.w = Some(w);
        self.h = Some(h);
        self
    }

    pub fn with_title<T: Into<HSTRING>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_icon_resource(mut self, icon: u16) -> Self {
        self.icon = Some(icon);
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
            WindowLevel::Normal => HWND_TOP
        }
    }
}

pub struct Window {
    pub hwnd: HWND,
    source: DesktopWindowXamlSource,
    icon: Option<HANDLE>
}

impl Window {
    const CLASS_NAME: PCWSTR = w!("rusty-twinkle-tray.window");
    fn new(builder: WindowBuilder) -> Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };
        static REGISTER_WINDOW_CLASS: Once = Once::new();
        REGISTER_WINDOW_CLASS.call_once(|| {
            Self::register(instance).unwrap_or_else(|err| log::warn!("Failed to register window class: {}", err));
        });

        let mut ex_style = WS_EX_NOREDIRECTIONBITMAP; // | WS_EX_TOPMOST;
        if builder.hidden {
            ex_style |= WS_EX_LAYERED | WS_EX_NOACTIVATE;
        }

        let style = match builder.hidden {
            true => WS_POPUP,
            false => WS_OVERLAPPEDWINDOW
        };

        let hwnd = unsafe {
            CreateWindowExW(
                ex_style,
                Self::CLASS_NAME,
                builder.title.as_ref().unwrap_or(h!("XAML Island Window")),
                style,
                builder.x.unwrap_or(CW_USEDEFAULT),
                builder.y.unwrap_or(CW_USEDEFAULT),
                builder.w.unwrap_or(CW_USEDEFAULT),
                builder.h.unwrap_or(CW_USEDEFAULT),
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
            let window_data = Box::new(WindowData {
                island,
                close_handler: Cell::new(builder.close_handler)
            });
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(window_data) as _);
            sync_size(hwnd, island)?;
        }

        let icon = match builder.icon {
            None => None,
            Some(id) => unsafe {
                let icon = LoadImageW(instance, PCWSTR(id as *const u16), IMAGE_ICON, 0, 0, LR_DEFAULTSIZE)?;
                SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(icon.0));
                SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_SMALL as usize), LPARAM(icon.0));
                Some(icon)
            }
        };

        Ok(Self {
            hwnd,
            source: desktop_source,
            icon
        })
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
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
            SetWindowPos(self.hwnd, after, x, y, w, h, flags).unwrap_or_else(|err| log::warn!("Failed to set window position: {}", err));
        }
    }

    pub fn set_visible(&self, visible: bool) {
        self.set_window_pos(None, None, None, Some(visible))
    }

    pub fn focus(&self) {
        unsafe {
            SetFocus(self.hwnd);
        }
    }

    pub fn dpi(&self) -> f32 {
        unsafe { GetDpiForWindow(self.hwnd) as f32 / USER_DEFAULT_SCREEN_DPI as f32 }
    }

    pub fn set_content<T: TryIntoParam<UIElement>>(&self, content: T) -> Result<()> {
        self.source.SetContent(content)?;
        Ok(())
    }

    pub fn apply_mica_backdrop(&self) -> Result<()> {
        unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &DWMSBT_MAINWINDOW as *const _ as _,
                size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32
            )?
        }
        Ok(())
    }

    pub fn make_titlebar_transparent(&self) -> Result<()> {
        unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_CAPTION_COLOR,
                &DWMWA_COLOR_NONE as *const _ as _,
                size_of::<COLORREF>() as u32
            )?
        }
        Ok(())
    }

    pub fn enable_immersive_dark_mode(&self, enabled: bool) -> Result<()> {
        unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &BOOL::from(&enabled) as *const _ as _,
                size_of::<BOOL>() as u32
            )?
        }
        Ok(())
    }

    fn register(instance: HMODULE) -> Result<()> {
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
        // Treat the pointer as 'const' instead of 'mut' as I'm not sure if this function is reentrant
        let window_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const WindowData;
        match msg {
            WM_NCCREATE => {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            }
            WM_SIZING | WM_SIZE => {
                if let Some(data) = window_data.as_ref() {
                    sync_size(hwnd, data.island).unwrap_or_else(|err| log::warn!("Failed to sync window size: {}", err));
                }
            }
            /*
            This doesn't seem to produce correct results on Win11 despite what the docs say:
            https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged
            WM_DPICHANGED => {
                let suggested_rect = *(lparam.0 as *const RECT);
                SetWindowPos(
                    hwnd,
                    None,
                    suggested_rect.left,
                    suggested_rect.right,
                    suggested_rect.right - suggested_rect.left,
                    suggested_rect.bottom- suggested_rect.top,
                    SWP_NOZORDER | SWP_NOACTIVATE
                ).unwrap();
            },
            */
            WM_CLOSE => {
                if let Some(data) = window_data.as_ref() {
                    if let Some(mut callback) = data.close_handler.take() {
                        callback();
                        data.close_handler.set(Some(callback))
                    }
                }
                return LRESULT::default();
            }
            WM_DESTROY => {
                trace!("Destroying window data");
                // Here we really need a 'mut' pointer, but we erase the pointer first so it should be fine
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                drop(Box::from_raw(window_data as *mut WindowData));
                // the event loop implementation currently doesn't handle quit messages
                // PostQuitMessage(0);
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
            if let Some(icon) = self.icon.take() {
                DestroyIcon(HICON(icon.0)).unwrap_or_else(|err| log::warn!("Failed to destroy window icon: {}", err))
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

struct WindowData {
    island: HWND,
    close_handler: Cell<Option<Box<dyn FnMut() + 'static>>>
}
