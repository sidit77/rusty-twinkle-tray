#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod utils;
mod ui;
mod interface;
mod backend;
mod config;
mod power;
mod theme;
pub mod runtime;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder, winit::WinitTrayIconBuilderExt};
use log::LevelFilter;
use windows::Foundation::{EventHandler, TypedEventHandler};
use windows::UI::ViewManagement::UISettings;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows_ext::UI::Xaml::Hosting::{WindowsXamlManager};
use windows_ext::UI::Xaml::Input::{FocusManager, LosingFocusEventArgs};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{DeviceEvents, EventLoopBuilder};
use winit::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
use winit::window::{WindowBuilder, WindowButtons, WindowLevel};
use windows_ext::UI::Xaml::ElementTheme;
use crate::backend::{BackendEvent, MonitorController};
use crate::config::Config;
use crate::interface::{XamlGui};
use crate::power::{PowerEvent, PowerStateListener};
use crate::theme::{ColorSet, SystemSettings};

use crate::utils::error::{OptionExt};
use crate::utils::{logger, panic};
use crate::utils::extensions::{BorderColor, EventLoopExt, MonitorHandleExt, MutexExt, WindowExt};

pub use crate::utils::error::Result;

include!("../assets/ids.rs");

#[derive(Debug, Clone)]
pub enum CustomEvent {
    Quit,
    Show,
    FocusLost,
    ThemeChange,
    Backend(BackendEvent)
}

fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };

    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let config = Arc::new(Mutex::new(Config::load()?));

    let event_loop = EventLoopBuilder::with_user_event()
        .build()?;
    event_loop.listen_device_events(DeviceEvents::Never);

    let controller = MonitorController::new(&event_loop, config.clone());

    let ui_settings = UISettings::new()?;
    let mut colors = SystemSettings::new()
        .map_err(|err| log::warn!("Failed to read system settings: {err}"))
        .ok()
        .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));

    let tray = TrayIconBuilder::new()
        .with_tooltip("Change Brightness")
        .with_icon(Icon::from_resource(if colors.theme == ElementTheme::Light { BRIGHTNESS_LIGHT_ICON } else { BRIGHTNESS_DARK_ICON}, None)?)
        .with_menu(Menu::new([MenuItem::button("Quit", CustomEvent::Quit)]))
        .build_event_loop(&event_loop, |event| match event {
            TrayEvent::Tray(ClickType::Left) => Some(CustomEvent::Show),
            TrayEvent::Menu(e) => Some(e),
            _ => None
        })?;

    let window = WindowBuilder::new()
        .with_title("XAML Window")
        //Make the window "invisible" to hide the creation without breaking stuff
        .with_position(PhysicalPosition::new(1000000, 0))
        .with_inner_size(LogicalSize::new(400, 250))
        .with_no_redirection_bitmap(true)
        .with_decorations(false)
        .with_undecorated_shadow(true)
        .with_skip_taskbar(true)
        .with_visible(true)
        .with_resizable(false)
        .with_drag_and_drop(false)
        .with_enabled_buttons(WindowButtons::empty())
        .build(&event_loop)?;

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
    let _power_listener = PowerStateListener::new({
        let proxy = controller.create_proxy();
        move |event| if event == PowerEvent::MonitorOn {
            proxy.refresh_brightness_in(Duration::from_secs(10));
        }
    })?;

    ui_settings.ColorValuesChanged(&TypedEventHandler::new({
        let proxy = event_loop.create_proxy();
        move |_: &Option<UISettings>, _| {
            proxy
                .send_event(CustomEvent::ThemeChange)
                .unwrap_or_else(|err| log::warn!("Failed to forward event: {}", err));
            Ok(())
        }
    }))?;

    let mut gui = XamlGui::new(&window, &colors)?;
    window.set_border_color(BorderColor::NONE);
    //Drop input focus
    window.set_enable(false);

    let mut last_close = Instant::now();
    event_loop
        .run_result(|event, target| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new) => gui.resize(new)?,
                    WindowEvent::Focused(true) => gui.focus(),
                    _ => {}
                },
                Event::UserEvent(event) => match event {
                    CustomEvent::Quit => {
                        controller.shutdown();
                        target.exit()
                    },
                    CustomEvent::Show => if last_close.elapsed() >= Duration::from_millis(250) {
                        controller.refresh_brightness();
                        if let Some(workspace) = target.primary_monitor().and_then(|m| m.get_work_area().ok()) {
                            if let Ok(height) = gui.get_required_height()
                                .map_err(|err| log::debug!("Failed to get required height: {err}"))
                            {
                                let width = window.outer_size()
                                    .to_logical::<f32>(window.scale_factor())
                                    .width as u32;
                                let _ = window.request_inner_size(LogicalSize::new(
                                    width, height));
                            }

                            let gap = 14;
                            let size = window.outer_size();

                            window.set_outer_position(PhysicalPosition::new(
                                workspace.right - gap - size.width as i32,
                                workspace.bottom - gap - size.height as i32
                            ));

                            window.set_visible(true);
                            window.set_window_level(WindowLevel::AlwaysOnTop);
                            window.focus_window();
                        } else {
                            log::warn!("Can't find work area of primary monitor");
                        }
                    }
                    CustomEvent::FocusLost => {
                        window.set_visible(false);
                        config.lock_no_poison().save_if_dirty()?;
                        last_close = Instant::now();
                    },
                    CustomEvent::ThemeChange => {
                        colors = SystemSettings::new()
                            .map_err(|err| log::warn!("Failed to read system settings: {err}"))
                            .ok()
                            .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));
                        gui.update_theme(&colors)
                            .unwrap_or_else(|err| log::warn!("Failed to update gui theme: {err}"));
                        tray.set_icon(Icon::from_resource(if colors.theme == ElementTheme::Light { BRIGHTNESS_LIGHT_ICON } else { BRIGHTNESS_DARK_ICON}, None)?);
                    }
                    CustomEvent::Backend(event) => match event {
                        BackendEvent::RegisterMonitor(name, path) => {
                            log::info!("Found monitor: {}", name);
                            gui.register_monitor(name, path, controller.create_proxy())?
                        },
                        BackendEvent::UpdateBrightness(path, value) => {
                            gui.update_brightness(path, value)?;
                        }
                    }

                },
                _ => {}
            }
            Ok(())
        })?;

    config.lock_no_poison().save_if_dirty()?;
    unsafe { RoUninitialize() }
    Ok(())
}

fn main() -> ExitCode {
    panic::set_hook();
    logger::init(LevelFilter::Trace, LevelFilter::Warn);

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            log::error!("{:?}", err);
            panic::show_msg(format_args!("{}\n at {}", err.message(), err.trace()));
            ExitCode::FAILURE
        }
    }
}

