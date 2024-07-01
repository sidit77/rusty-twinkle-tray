use std::mem::size_of;
use std::sync::Once;
use windows::core::{ComInterface, PCWSTR, TryIntoParam, w};
use windows::Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, CW_USEDEFAULT, DefWindowProcW, DestroyWindow, GetClientRect, GetWindowLongPtrW, GWLP_USERDATA, HWND_TOPMOST, IsWindow, PostQuitMessage, RegisterClassExW, SetForegroundWindow, SetWindowLongPtrW, SetWindowPos, SWP_HIDEWINDOW, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, WM_DESTROY, WM_NCCREATE, WM_SIZE, WM_SIZING, WNDCLASSEXW, WS_EX_NOREDIRECTIONBITMAP, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use windows_ext::UI::Xaml::Hosting::DesktopWindowXamlSource;
use windows_ext::UI::Xaml::UIElement;
use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use crate::ui::controls::TextBlock;
use crate::{win_assert, Result};

pub struct ProxyWindow {
    hwnd: HWND,
    source: DesktopWindowXamlSource
}

impl ProxyWindow {
    const CLASS_NAME: PCWSTR = w!("rusty-twinkle-tray.window");
    pub fn new() -> crate::Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };
        static REGISTER_WINDOW_CLASS: Once = Once::new();
        REGISTER_WINDOW_CLASS.call_once(|| {
            Self::register(instance)
                .unwrap_or_else(|err| log::warn!("Failed to register window class: {}", err));
        });

        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_NOREDIRECTIONBITMAP,
                Self::CLASS_NAME,
                w!("XAML Test"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                0, 0, 100, 100,
                None,
                None,
                instance,
                None
            )
        };

        win_assert!(hwnd != HWND::default());

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
            source: desktop_source,
        })
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn set_foreground(&self) {
        unsafe {
            SetForegroundWindow(self.hwnd)
                .ok()
                .unwrap_or_else(|e| log::warn!("Failed to set foreground window: {e}"));
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            match visible {
                true => SetWindowPos(self.hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_SHOWWINDOW | SWP_NOSIZE | SWP_NOMOVE),
                false => SetWindowPos(self.hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_HIDEWINDOW | SWP_NOZORDER | SWP_NOSIZE | SWP_NOMOVE)
            }.unwrap_or_else(|err| log::warn!("Failed to set window visibility: {}", err));
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
                    sync_size(hwnd, HWND(island))
                        .unwrap_or_else(|err| log::warn!("Failed to sync window size: {}", err));
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

impl Drop for ProxyWindow {
    fn drop(&mut self) {
        unsafe {
            if IsWindow(self.hwnd).as_bool() {
                DestroyWindow(self.hwnd)
                    .unwrap_or_else(|err| log::warn!("Failed to destroy window: {}", err));
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