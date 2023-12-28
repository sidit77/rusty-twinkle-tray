#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod monitors;
mod utils;

use std::mem::size_of;
use std::process::ExitCode;

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIcon, TrayIconBuilder, TrayResult};
use log::LevelFilter;
use windows::core::{ComInterface, HSTRING};
use windows::Foundation::EventHandler;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_SHOWWINDOW, WHEEL_DELTA};
use windows::UI::Color;
use windows::UI::Text::FontWeight;
use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use windows_ext::UI::Xaml::Controls::Primitives::RangeBaseValueChangedEventHandler;
use windows_ext::UI::Xaml::Controls::{ColumnDefinition, FontIcon, Grid, Orientation, Slider, StackPanel, TextBlock};
use windows_ext::UI::Xaml::Hosting::{DesktopWindowXamlSource, WindowsXamlManager};
use windows_ext::UI::Xaml::Input::{FocusManager, LosingFocusEventArgs, PointerEventHandler};
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};
use windows_ext::UI::Xaml::{ElementTheme, GridLength, GridUnitType, HorizontalAlignment, TextAlignment, Thickness, VerticalAlignment};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{DeviceEvents, EventLoop, EventLoopBuilder};
use winit::platform::windows::{MonitorHandleExtWindows, WindowBuilderExtWindows};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowBuilder, WindowButtons};

use crate::utils::error::{OptionExt, Result};
use crate::utils::{logger, panic};

#[derive(Debug, Copy, Clone)]
enum CustomEvent {
    Quit,
    Show,
    FocusLost
}

fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let event_loop = EventLoopBuilder::with_user_event().build().unwrap();

    event_loop.listen_device_events(DeviceEvents::Never);

    let _tray = TrayIconBuilder::new()
        .with_tooltip("Change Brightness")
        .with_icon(Icon::from_rgba(vec![255u8; 4 * 32 * 32], 32, 32).unwrap())
        .with_menu(Menu::new([MenuItem::button("Quit", CustomEvent::Quit)]))
        .build_event_loop(&event_loop, |event| match event {
            TrayEvent::Tray(ClickType::Left) => Some(CustomEvent::Show),
            TrayEvent::Menu(e) => Some(e),
            _ => None
        })
        .unwrap();

    let window = WindowBuilder::new()
        .with_title("XAML Window")
        //Make the window "invisible" to hide the creation without breaking stuff
        .with_position(PhysicalPosition::new(1000000, 0))
        .with_inner_size(PhysicalSize::new(400, 250))
        .with_no_redirection_bitmap(true)
        .with_decorations(false)
        .with_undecorated_shadow(true)
        .with_skip_taskbar(true)
        .with_visible(true)
        .with_resizable(false)
        .with_enabled_buttons(WindowButtons::empty())
        .build(&event_loop)
        .unwrap();

    let gui = XamlGui::new(&window)?;

    FocusManager::LosingFocus(&EventHandler::new({
        let proxy = event_loop.create_proxy();
        move |_e, arg: &Option<LosingFocusEventArgs>| {
            if arg.as_ref().some()?.NewFocusedElement().is_err() {
                proxy
                    .send_event(CustomEvent::FocusLost)
                    .unwrap_or_else(|err| log::warn!("Failed to forward event: {}", err));
            }
            Ok(())
        }
    }))?;
    window.set_visible(true);

    event_loop
        .run(|event, target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new) => {
                    gui.resize(new).unwrap();
                }
                WindowEvent::Focused(true) => unsafe {
                    SetFocus(gui.hwnd);
                },
                _ => {}
            },
            Event::UserEvent(event) => match event {
                CustomEvent::Quit => target.exit(),
                CustomEvent::Show => {
                    if let Some(monitor) = target.primary_monitor() {
                        let workspace = {
                            let mut mi = MONITORINFO {
                                cbSize: size_of::<MONITORINFO>() as u32,
                                ..Default::default()
                            };
                            unsafe {
                                GetMonitorInfoW(HMONITOR(monitor.hmonitor()), &mut mi)
                                    .ok()
                                    .unwrap()
                            };
                            mi.rcWork
                        };

                        let gap = 14;
                        let size = window.outer_size();

                        window.set_outer_position(PhysicalPosition::new(
                            workspace.right - gap - size.width as i32,
                            workspace.bottom - gap - size.height as i32
                        ));
                        window.set_visible(true);
                        window.focus_window();
                    } else {
                        log::warn!("Can't find primary monitor");
                    }
                }
                CustomEvent::FocusLost => {
                    window.set_visible(false);
                }
            },
            _ => {}
        })
        .unwrap();

    unsafe { RoUninitialize() }
    Ok(())
}

struct XamlGui {
    hwnd: HWND,
    _desktop_source: DesktopWindowXamlSource
}

impl XamlGui {
    pub fn new(parent: &Window) -> Result<Self> {
        let desktop_source = DesktopWindowXamlSource::new()?;
        let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
        let parent = match parent.window_handle().unwrap().as_raw() {
            RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get()),
            _ => unimplemented!()
        };
        unsafe {
            interop.AttachToWindow(parent)?;
        }
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
            Bottom: 8.0
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
            Bottom: 8.0
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
                        GridUnitType: GridUnitType::Star
                    })?;
                    def
                })?;
                columns.Append(&{
                    let def = ColumnDefinition::new()?;
                    def.SetWidth(GridLength {
                        Value: 50.0,
                        GridUnitType: GridUnitType::Pixel
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
                    let delta = args
                        .GetCurrentPoint(None)?
                        .Properties()?
                        .MouseWheelDelta()?
                        / WHEEL_DELTA as i32;

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
        bottom_bar.SetHorizontalAlignment(HorizontalAlignment::Stretch)?;
        bottom_bar.SetSpacing(230.0)?;
        let bottom_bar_children = bottom_bar.Children()?;

        bottom_bar_children.Append(&{
            let text_block = TextBlock::new()?;
            text_block.SetText(&HSTRING::from("Adjust Brightness"))?;
            text_block.SetHorizontalAlignment(HorizontalAlignment::Left)?;
            text_block.SetFontSize(15.0)?;
            text_block
        })?;

        bottom_bar_children.Append(&{
            let icon = FontIcon::new()?;
            icon.SetGlyph(&HSTRING::from("\u{E713}"))?; // Modern Windows 11 Settings icon
            icon.SetFontWeight(FontWeight { Weight: 500 })?;
            icon.SetHorizontalAlignment(HorizontalAlignment::Right)?;
            icon.SetMargin(Thickness {
                Left: 0.0,
                Top: 0.0,
                Right: 200.0, // Add right padding
                Bottom: 0.0
            })?;
            icon
        })?;

        // Add the main stack panel and the bottom bar to the main grid
        let main_grid_children = main_grid.Children()?;
        main_grid_children.Append(&stack_panel)?;
        main_grid_children.Append(&bottom_bar)?;

        desktop_source.SetContent(&main_grid)?;
        Ok(Self {
            hwnd: island,
            _desktop_source: desktop_source
        })
    }

    pub fn resize(&self, new_size: PhysicalSize<u32>) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND::default(),
                0,
                0,
                new_size.width as _,
                new_size.height as _,
                SWP_SHOWWINDOW
            )?;
        }
        Ok(())
    }
}

trait TrayIconBuilderExt<T> {
    fn build_event_loop<E, F>(self, event_loop: &EventLoop<E>, map: F) -> TrayResult<TrayIcon<T>>
    where
        F: Fn(TrayEvent<T>) -> Option<E> + Send + 'static,
        E: Send;
}

impl<T: Clone + Send + 'static> TrayIconBuilderExt<T> for TrayIconBuilder<T> {
    fn build_event_loop<E, F>(self, event_loop: &EventLoop<E>, map: F) -> TrayResult<TrayIcon<T>>
    where
        F: Fn(TrayEvent<T>) -> Option<E> + Send + 'static,
        E: Send
    {
        let proxy = event_loop.create_proxy();
        self.build(move |event| {
            if let Some(event) = map(event) {
                proxy
                    .send_event(event)
                    .unwrap_or_else(|err| log::warn!("Failed to forward event: {}", err));
            }
        })
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
