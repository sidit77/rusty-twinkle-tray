#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod utils;
mod window;

use std::process::ExitCode;
use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use std::sync::Once;
use log::LevelFilter;
use windows::core::{PCWSTR, w, ComInterface, HSTRING};
use windows::UI::Color;
use windows::UI::Text::FontWeight;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize, RoUninitialize};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows_ext::UI::Xaml::Controls::{ColumnDefinition, FontIcon, Grid, Orientation, Slider, StackPanel, TextBlock};
use windows_ext::UI::Xaml::Hosting::{DesktopWindowXamlSource, WindowsXamlManager};
use windows_ext::UI::Xaml::{ElementTheme, GridLength, GridUnitType, HorizontalAlignment, TextAlignment, Thickness, VerticalAlignment};
use windows_ext::UI::Xaml::Controls::Primitives::RangeBaseValueChangedEventHandler;
use windows_ext::UI::Xaml::Input::PointerEventHandler;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};
use crate::utils::error::{OptionExt, Result};
use crate::utils::{logger, panic};
use crate::window::{check_for_failure, Event, Window, WindowClass};

static REGISTER_WINDOW_CLASS: Once = Once::new();
const WM_PANIC: u32 = WM_USER + 1;


fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let instance = unsafe { GetModuleHandleW(None)? };
    REGISTER_WINDOW_CLASS.call_once(|| {
        XamlWindow::register(instance)
            .expect("Failed to register window class")
    });

    let window = Window::new::<XamlWindow>(instance)?;

    let mut message = MSG::default();

    unsafe {
        while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() {
            check_for_failure(&message)?;
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        ShowWindow(window.hwnd(), SW_MINIMIZE);
        ShowWindow(window.hwnd(), SW_RESTORE);
    }

    unsafe {
        while GetMessageW(&mut message, None, 0, 0).into() {
            check_for_failure(&message)?;
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }

    unsafe { RoUninitialize() }
    Ok(())
}

struct XamlWindow {
    parent_hwnd: HWND,
    child_hwnd: HWND,
    _desktop_source: DesktopWindowXamlSource
}

impl WindowClass for XamlWindow {
    const NAME: PCWSTR = w!("rusty-twinkle-tray.window");

    fn initialize(parent: HWND) -> Result<Self> {
        let desktop_source = DesktopWindowXamlSource::new()?;
        let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
        unsafe { interop.AttachToWindow(parent)?; }
        let island = unsafe { interop.WindowHandle() }?;
        //let icon_font = FontFamily::new(&HSTRING::from("Segoe Fluent Icons"))?;
        let main_grid = Grid::new()?; // Create a new grid to hold the main stackpanel and the bottom bar
        main_grid.SetBackground(&{
            let brush = AcrylicBrush::new()?;
            let color = Color { R: 70, G: 70, B: 70, A: 255 };
            brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
            brush.SetFallbackColor(color)?;
            brush.SetTintColor(color)?;
            brush.SetTintOpacity(0.7)?;
            brush
        })?;
        main_grid.SetRequestedTheme(ElementTheme::Dark)?;
        main_grid.SetRowSpacing(8.0)?;
        main_grid.SetPadding(Thickness {
            Left: 8.0,
            Top: 8.0,
            Right: 8.0,
            Bottom: 8.0,
        })?;

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
                // text_box.TextChanging(&TypedEventHandler::new({
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
                // }))?;
                let children = grid.Children()?;
                children.Append(&slider)?;
                children.Append(&text_box)?;
                grid
            })?;
            stack_panel
        })?;

        // Create a new stack panel for the bottom bar
        let bottom_bar = StackPanel::new()?;
        bottom_bar.SetOrientation(Orientation::Horizontal)?;
        bottom_bar.SetVerticalAlignment(VerticalAlignment::Bottom)?;
        bottom_bar.SetHorizontalAlignment(HorizontalAlignment::Right)?;
        let bottom_bar_children = bottom_bar.Children()?;
        bottom_bar_children.Append(&{
            let icon = FontIcon::new()?;
            icon.SetGlyph(&HSTRING::from("\u{E713}"))?; // Modern Windows 11 Settings icon
            icon.SetFontWeight(FontWeight { Weight: 500 })?; // Add more padding from margins
            icon
        })?;

        // Add the main stack panel and the bottom bar to the main grid
        let main_grid_children = main_grid.Children()?;
        main_grid_children.Append(&stack_panel)?;
        main_grid_children.Append(&bottom_bar)?;

        //button.SetContent(&IInspectable::try_from("Hello World")?)?;
        desktop_source.SetContent(&main_grid)?;
        Ok(Self {
            parent_hwnd: parent,
            child_hwnd: island,
            _desktop_source: desktop_source,
        })
    }

    fn on_event(&mut self, event: Event) -> Result<()> {

        match event {
            Event::Destroy => unsafe { PostQuitMessage(0); }
            Event::Resize => unsafe {
                let mut rect = RECT::default();
                GetClientRect(self.parent_hwnd, &mut rect)?;
                SetWindowPos(self.child_hwnd, HWND::default(), 0, 0,
                             rect.right - rect.left,
                             rect.bottom - rect.top,
                             SWP_SHOWWINDOW)?;
            }
        }
        Ok(())
    }
}

fn main() -> ExitCode {
    panic::set_hook();
    logger::init(LevelFilter::Trace, LevelFilter::Warn);

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            log::error!("{:?}", err);
            panic::show_msg(format_args!("{}\n at {}", err.error().message(), err.trace()));
            ExitCode::FAILURE
        }
    }
}