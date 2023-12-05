use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use std::sync::Once;
use windows::core::{PCWSTR, w, Result, ComInterface};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize, RoUninitialize};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, CW_USEDEFAULT, DefWindowProcW, DispatchMessageW, GetMessageW, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassW, SetWindowPos, ShowWindow, SW_SHOW, SWP_SHOWWINDOW, TranslateMessage, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use windows_ext::UI::Xaml::Controls::{TextBox};
use windows_ext::UI::Xaml::Hosting::{DesktopWindowXamlSource, WindowsXamlManager};

static REGISTER_WINDOW_CLASS: Once = Once::new();
const WINDOW_CLASS_NAME: PCWSTR = w!("modern-gui.Window");

fn main() -> Result<()> {
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

    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;
    let desktop_source = DesktopWindowXamlSource::new()?;
    let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
    unsafe { interop.AttachToWindow(hwnd)?; }
    let island = unsafe { interop.WindowHandle() }?;
    unsafe  {
        SetWindowPos(island, HWND::default(), 200, 100, 800, 200, SWP_SHOWWINDOW)?;
    }

    let button = TextBox::new()?;
    //button.SetContent(&IInspectable::try_from("Hello World")?)?;
    desktop_source.SetContent(&button)?;

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

unsafe extern "system" fn wnd_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if message == WM_DESTROY {
        PostQuitMessage(0);
        return LRESULT::default();
    }
    DefWindowProcW(window, message, wparam, lparam)
}