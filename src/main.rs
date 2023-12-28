#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod monitors;
mod utils;
mod ui;

use std::mem::size_of;
use std::process::ExitCode;

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder};
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
use windows_ext::UI::Xaml::Controls::{FontIcon, Slider, TextBlock};
use windows_ext::UI::Xaml::Hosting::{DesktopWindowXamlSource, WindowsXamlManager};
use windows_ext::UI::Xaml::Input::{FocusManager, LosingFocusEventArgs, PointerEventHandler};
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush, SolidColorBrush};
use windows_ext::UI::Xaml::{ElementTheme, TextAlignment, Thickness, VerticalAlignment};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{DeviceEvents, EventLoopBuilder};
use winit::platform::windows::{MonitorHandleExtWindows, WindowBuilderExtWindows};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowBuilder, WindowButtons};
use crate::ui::container::GridSize;

use crate::utils::error::{OptionExt, Result};
use crate::utils::{logger, panic, TrayIconBuilderExt};

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

        let stack_panel = ui::container::StackPanel::vertical()?
            .with_spacing(8.0)?
            .with_padding((20.0, 14.0))?
            .with_child(&Self::create_monitor_entry()?)?;

        // Create a new stack panel for the bottom bar

        let bottom_bar = ui::container::Grid::new()?
            .with_padding(20.0)?
            .with_column_widths([GridSize::Fraction(1.0), GridSize::Auto])?
            .with_background(&SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 70})?)?
            .with_child(&{
                let text_block = TextBlock::new()?;
                text_block.SetText(&HSTRING::from("Adjust Brightness"))?;
                text_block.SetVerticalAlignment(VerticalAlignment::Center)?;
                text_block.SetPadding(Thickness { Left: 20.0, Top: 0.0, Right: 0.0, Bottom: 0.0})?;
                text_block.SetFontSize(15.0)?;
                text_block
            }, 0, 0)?
            .with_child(&ui::container::StackPanel::horizontal()?
                .with_child(&{
                    let icon = FontIcon::new()?;
                    icon.SetGlyph(&HSTRING::from("\u{E713}"))?; // Modern Windows 11 Settings icon
                    icon.SetFontWeight(FontWeight { Weight: 500 })?;
                    icon.SetVerticalAlignment(VerticalAlignment::Center)?;
                    icon
                })?, 0, 1)?;

        let main_grid = ui::container::Grid::new()? // Create a new grid to hold the main stackpanel and the bottom bar
            .with_row_heights([GridSize::Fraction(1.0), GridSize::Auto])?
            .with_background(&{
                let brush = AcrylicBrush::new()?;
                let color = Color { R: 70, G: 70, B: 70, A: 255 };
                brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
                brush.SetFallbackColor(color)?;
                brush.SetTintColor(color)?;
                brush.SetTintOpacity(0.7)?;
                brush
            })?
            .with_theme(ElementTheme::Dark)?
            // Add the main stack panel and the bottom bar to the main grid
            .with_child(&stack_panel, 0, 0)?
            .with_child(&bottom_bar, 1, 0)?;

        desktop_source.SetContent(&main_grid)?;
        Ok(Self {
            hwnd: island,
            _desktop_source: desktop_source
        })
    }

    fn create_monitor_entry() -> Result<ui::container::StackPanel> {
        let slider = {
            let slider = Slider::new()?;
            slider.SetVerticalAlignment(VerticalAlignment::Center)?;
            slider
        };
        let text_box = {
            let text_box = TextBlock::new()?;
            text_box.SetVerticalAlignment(VerticalAlignment::Center)?;
            text_box.SetTextAlignment(TextAlignment::Center)?;
            text_box.SetFontSize(20.0)?;
            text_box.SetFontWeight(FontWeight { Weight: 400 })?;
            text_box.SetPadding(Thickness { Left: 10.0, Top: 0.0, Right: 10.0, Bottom: 0.0 })?;
            text_box.SetText(&HSTRING::from(&format!("{}", slider.Value()?)))?;
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

        Ok(ui::container::StackPanel::vertical()?
            .with_spacing(4.0)?
            .with_child(&ui::container::StackPanel::horizontal()?
                .with_spacing(8.0)?
                .with_child(&{
                    let icon = FontIcon::new()?;
                    //icon.SetFontFamily(&icon_font)?;
                    icon.SetGlyph(&HSTRING::from("\u{E7f4}"))?;
                    icon.SetFontWeight(FontWeight { Weight: 500 })?;
                    icon
                })?
                .with_child(&{
                    let text_block = TextBlock::new()?;
                    text_block.SetText(&HSTRING::from("Monitor 1"))?;
                    text_block.SetFontSize(20.0)?;
                    text_block
                })?)?
            .with_child(&ui::container::Grid::new()?
                .with_column_widths([GridSize::Fraction(1.0), GridSize::Auto])?
                .with_column_spacing(8.0)?
                .with_child(&slider, 0, 0)?
                .with_child(&text_box, 0, 1)?)?)
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
