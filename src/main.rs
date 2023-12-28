#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod monitors;
mod utils;
//mod window;
//mod tray;
mod framwork;

use std::process::ExitCode;
use std::time::Duration;
use async_io::Timer;
use log::LevelFilter;
use crate::framwork::{block_on, Event, Window};
use crate::utils::error::{Result};
use crate::utils::{logger, panic};
use futures_lite::StreamExt;


async fn run() -> Result<()> {
    println!("Starting");

    let window = Window::new()?;

    window
        .events()
        .take_while(|e| *e != Event::Close)
        .for_each(|e| println!("{:?}", e))
        .await;

    //Timer::interval(Duration::from_millis(500))
    //    .take(10)
    //    .enumerate()
    //    .for_each(|(i, _)| println!("Tick {}"))
    //    .await;
    Ok(())

/*
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let instance = unsafe { GetModuleHandleW(None)? };
    REGISTER_WINDOW_CLASS.call_once(|| {
        XamlWindow::register(instance)
            .expect("Failed to register window class")
    });

    let window = Window::new::<XamlWindow>(instance)?;

    tray(instance, window.hwnd())?;

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
 */
}

/*
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

 */
fn main() -> ExitCode {
    panic::set_hook();
    logger::init(LevelFilter::Trace, LevelFilter::Warn);

    match block_on(run()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            log::error!("{:?}", err);
            panic::show_msg(format_args!("{}\n at {}", err.error().message(), err.trace()));
            ExitCode::FAILURE
        }
    }
}