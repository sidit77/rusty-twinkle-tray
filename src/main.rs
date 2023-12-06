use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use std::sync::Once;
use windows::core::{PCWSTR, w, Result, ComInterface};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize, RoUninitialize};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, CW_USEDEFAULT, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW, GetWindowLongPtrW, GWLP_USERDATA, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassW, SetWindowLongPtrW, SetWindowPos, ShowWindow, SW_SHOW, SWP_SHOWWINDOW, TranslateMessage, WINDOW_EX_STYLE, WM_DESTROY, WM_NCCREATE, WM_NCDESTROY, WM_SIZE, WM_SIZING, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use windows_ext::UI::Xaml::Controls::{TextBox};
use windows_ext::UI::Xaml::Hosting::{DesktopWindowXamlSource, WindowsXamlManager};

static REGISTER_WINDOW_CLASS: Once = Once::new();
const WINDOW_CLASS_NAME: PCWSTR = w!("modern-gui.Window");

fn main() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let instance = unsafe { GetModuleHandleW(None)? };
    REGISTER_WINDOW_CLASS.call_once(|| {
        let class = WNDCLASSW {
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
            hInstance: instance.into(),
            lpszClassName: WINDOW_CLASS_NAME,
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };
        assert_ne!(unsafe { RegisterClassW(&class) }, 0);
    });

    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            WINDOW_CLASS_NAME,
            w!("XAML Test"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
            None,
            None,
            instance,
            None
        )
    };




    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
    };

    let mut message = MSG::default();
    unsafe {
        while GetMessageW(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }

    unsafe { RoUninitialize() }
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Event {
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

unsafe extern "system" fn wnd_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_NCCREATE => {
            let window = Window::new(hwnd).expect("Failed to create window");
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(Box::new(window)) as _);
        },
        WM_NCDESTROY => {
            let this = SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) as *mut Window;
            if !this.is_null() {
                let _window = Box::from_raw(this);
            }
        },
        _ => {
            let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window;
            if let Some(this) = this.as_mut() {
                if let Some(event) = Event::from_msg(message) {
                    this.on_event(event);
                    return LRESULT::default();
                }
            }
        }
    }
    DefWindowProcW(hwnd, message, wparam, lparam)
}

struct Window {
    parent_hwnd: HWND,
    child_hwnd: HWND,
    desktop_source: DesktopWindowXamlSource
}

impl Window {
    fn new(parent: HWND) -> Result<Self> {
        let desktop_source = DesktopWindowXamlSource::new()?;
        let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
        unsafe { interop.AttachToWindow(parent)?; }
        let island = unsafe { interop.WindowHandle() }?;

        let button = TextBox::new()?;
        //button.SetContent(&IInspectable::try_from("Hello World")?)?;
        desktop_source.SetContent(&button)?;

        Ok(Self {
            parent_hwnd: parent,
            child_hwnd: island,
            desktop_source,
        })
    }

    fn on_event(&mut self, event: Event) {
        match event {
            Event::Destroy => unsafe { PostQuitMessage(0); }
            Event::Resize => unsafe {
                let mut rect = RECT::default();
                GetClientRect(self.parent_hwnd, &mut rect).unwrap();
                SetWindowPos(self.child_hwnd, HWND::default(), 0, 0,
                             rect.right - rect.left,
                             rect.bottom - rect.top,
                             SWP_SHOWWINDOW).unwrap();
            }
        }
    }
}