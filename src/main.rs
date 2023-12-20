use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use std::sync::Once;
use windows::core::{PCWSTR, w, Result, ComInterface, HSTRING, Error};
use windows::UI::Color;
use windows::UI::Text::FontWeight;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, NO_ERROR, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize, RoUninitialize};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows_ext::UI::Xaml::Controls::{ColumnDefinition, FontIcon, Grid, Orientation, Slider, StackPanel, TextBlock};
use windows_ext::UI::Xaml::Hosting::{DesktopWindowXamlSource, WindowsXamlManager};
use windows_ext::UI::Xaml::{ElementTheme, GridLength, GridUnitType, TextAlignment, Thickness, VerticalAlignment};
use windows_ext::UI::Xaml::Controls::Primitives::RangeBaseValueChangedEventHandler;
use windows_ext::UI::Xaml::Input::PointerEventHandler;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};

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
            WS_EX_NOREDIRECTIONBITMAP,
            WINDOW_CLASS_NAME,
            w!("XAML Test"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 400, 400,
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
        while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        ShowWindow(hwnd, SW_MINIMIZE);
        ShowWindow(hwnd, SW_RESTORE);
    }

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
    _desktop_source: DesktopWindowXamlSource
}

impl Window {
    fn new(parent: HWND) -> Result<Self> {
        let desktop_source = DesktopWindowXamlSource::new()?;
        let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
        unsafe { interop.AttachToWindow(parent)?; }
        let island = unsafe { interop.WindowHandle() }?;

        //let icon_font = FontFamily::new(&HSTRING::from("Segoe Fluent Icons"))?;
        let stack_panel = StackPanel::new()?;
        stack_panel.SetBackground(&{
            let brush = AcrylicBrush::new()?;
            let color = Color { R: 70, G: 70, B: 70, A: 255 };
            brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
            brush.SetFallbackColor(color)?;
            brush.SetTintColor(color)?;
            brush.SetTintOpacity(0.7)?;
            brush
        })?;
        stack_panel.SetRequestedTheme(ElementTheme::Dark)?;
        stack_panel.SetSpacing(8.0)?;
        stack_panel.SetPadding(Thickness {
            Left: 8.0,
            Top: 8.0,
            Right: 8.0,
            Bottom: 8.0,
        })?;
        let children = stack_panel.Children()?;
        children.Append(&{
            let stack_panel = StackPanel::new()?;
            let children = stack_panel.Children()?;
            children.Append(&{
                let stack_panel = StackPanel::new()?;
                stack_panel.SetOrientation(Orientation::Horizontal)?;
                stack_panel.SetSpacing(8.0)?;
                let children = stack_panel.Children()?;
                children.Append(&{
                    let icon = FontIcon::new()?;
                    //icon.SetFontFamily(&icon_font)?;
                    icon.SetGlyph(&HSTRING::from("\u{E7f4}"))?;
                    icon.SetFontWeight(FontWeight { Weight: 500 })?;
                    icon
                })?;
                children.Append(&{
                    let text_block = TextBlock::new()?;
                    text_block.SetText(&HSTRING::from("Monitor 1"))?;
                    text_block.SetFontSize(20.0)?;
                    text_block
                })?;
                stack_panel
            })?;
            children.Append(&{
                //let grid = StackPanel::new()?;
                let grid = Grid::new()?;
                grid.SetColumnSpacing(8.0)?;
                let columns = grid.ColumnDefinitions()?;
                columns.Append(&{
                    let def = ColumnDefinition::new()?;
                    def.SetWidth(GridLength {
                        Value: 1.0,
                        GridUnitType: GridUnitType::Star,
                    })?;
                    def
                })?;
                columns.Append(&{
                    let def = ColumnDefinition::new()?;
                    def.SetWidth(GridLength {
                        Value: 50.0,
                        GridUnitType: GridUnitType::Pixel,
                    })?;
                    def
                })?;
                let slider = {
                    let slider = Slider::new()?;
                    slider.SetVerticalAlignment(VerticalAlignment::Center)?;
                    Grid::SetColumn(&slider, 0)?;
                    slider
                };
                let text_box = {
                    let text_box = TextBlock::new()?;
                    text_box.SetVerticalAlignment(VerticalAlignment::Center)?;
                    text_box.SetTextAlignment(TextAlignment::Center)?;
                    //text_box.SetBorderThickness(Thickness::default())?;
                    text_box.SetFontSize(20.0)?;
                    text_box.SetFontWeight(FontWeight { Weight: 400 })?;
                    text_box.SetPadding(Thickness::default())?;
                    text_box.SetText(&HSTRING::from(&format!("{}", slider.Value()?)))?;
                    Grid::SetColumn(&text_box, 1)?;
                    text_box
                };
                slider.ValueChanged(&RangeBaseValueChangedEventHandler::new({
                    let text_box = text_box.clone();
                    move |_, event| {
                        text_box.SetText(&HSTRING::from(format!("{}", event.some()?.NewValue()?)))?;
                        Ok(())
                    }
                }))?;
                slider.PointerWheelChanged(&PointerEventHandler::new(move |sender, args| {
                    let args = args.some()?;
                    args.SetHandled(true)?;
                    let delta = args.GetCurrentPoint(None)?
                        .Properties()?
                        .MouseWheelDelta()? / WHEEL_DELTA as i32;

                    let slider = sender.some()?.cast::<Slider>()?;
                    slider.SetValue2(slider.Value()? + delta as f64)?;
                    Ok(())
                }))?;
                //text_box.TextChanging(&TypedEventHandler::new({
                //    let slider = slider.clone();
                //    move |sender: &Option<TextBox>, _| {
                //        let sender = sender.as_ref().some()?;
                //        let text = sender.Text()?.to_string_lossy();
                //        if let Some(value) = text.parse::<u32>().ok() {
                //            slider.SetValue2(value as f64)?;
                //        }
                //        if !text.is_empty() {
                //            sender.SetText(&HSTRING::from(&format!("{}", slider.Value()?)))?;
                //        }
                //        Ok(())
                //    }
                //}))?;
                let children = grid.Children()?;
                children.Append(&slider)?;
                children.Append(&text_box)?;
                grid
            })?;
            stack_panel
        })?;

        //button.SetContent(&IInspectable::try_from("Hello World")?)?;
        desktop_source.SetContent(&stack_panel)?;

        Ok(Self {
            parent_hwnd: parent,
            child_hwnd: island,
            _desktop_source: desktop_source,
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

pub trait OptionExt<T> {
    fn some(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn some(self) -> Result<T> {
        self.ok_or(Error::from(NO_ERROR))
    }
}