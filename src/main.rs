#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod utils;
mod ui;
mod interface;
mod backend;
mod config;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder, winit::WinitTrayIconBuilderExt};
use log::LevelFilter;
use windows::Foundation::EventHandler;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows_ext::UI::Xaml::Hosting::{WindowsXamlManager};
use windows_ext::UI::Xaml::Input::{FocusManager, LosingFocusEventArgs};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{DeviceEvents, EventLoopBuilder};
use winit::platform::windows::{WindowBuilderExtWindows};
use winit::window::{WindowBuilder, WindowButtons, WindowLevel};
use crate::backend::MonitorController;
use crate::config::Config;
use crate::interface::{XamlGui};
use crate::monitors::MonitorPath;

use crate::utils::error::{OptionExt};
use crate::utils::{logger, panic};
use crate::utils::extensions::{EventLoopExt, MonitorHandleExt, MutexExt};

pub use crate::utils::error::Result;

#[derive(Debug, Clone)]
pub enum CustomEvent {
    Quit,
    Show,
    FocusLost,
    RegisterMonitor(String, MonitorPath),
    UpdateBrightness(MonitorPath, u32)
}

fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let config = Arc::new(Mutex::new(Config::load()?));

    let event_loop = EventLoopBuilder::with_user_event().build()?;
    event_loop.listen_device_events(DeviceEvents::Never);

    let controller = MonitorController::new(&event_loop, config.clone());

    let _tray = TrayIconBuilder::new()
        .with_tooltip("Change Brightness")
        .with_icon(Icon::from_rgba(generate_circle_icon_rgba(32), 32, 32)?)
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
        .with_inner_size(PhysicalSize::new(400, 250))
        .with_no_redirection_bitmap(true)
        .with_decorations(false)
        .with_undecorated_shadow(true)
        .with_skip_taskbar(true)
        .with_visible(true)
        .with_resizable(false)
        .with_enabled_buttons(WindowButtons::empty())
        .build(&event_loop)?;

    let mut gui = XamlGui::new(&window)?;

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
        .run_result(|event, target| Ok(match event {
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
                CustomEvent::Show => {
                    controller.refresh_brightness();
                    if let Some(workspace) = target.primary_monitor().and_then(|m| m.get_work_area().ok()) {
                        if let Ok(height) = gui.get_required_height() {
                            let _ = window.request_inner_size(PhysicalSize::new(
                                window.outer_size().width,
                                height));
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
                }
                CustomEvent::RegisterMonitor(name, path) => {
                    log::info!("Found monitor: {}", name);
                    gui.register_monitor(name, path, controller.create_proxy())?
                },
                CustomEvent::UpdateBrightness(path, value) => {
                    gui.update_brightness(path, value)?;
                }
            },
            _ => {}
        }))?;

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
fn generate_circle_icon_rgba(size: usize) -> Vec<u8> {
    let radius = size as f32 / 2.0;
    let center = size as f32 / 2.0;

    let mut rgba_data = vec![0; 4 * size * size];

    for y in 0..size {
        for x in 0..size {
            let offset = 4 * (y * size + x);
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance_squared = dx * dx + dy * dy;

            // Check if the pixel is inside the circle
            if distance_squared <= radius * radius {
                // Create a unique color pattern based on pixel position
                rgba_data[offset] = (x as u8 + y as u8) % 255;  // Red
                rgba_data[offset + 1] = ((2 * x as u8 + y as u8) % 255).wrapping_add(128);  // Green
                rgba_data[offset + 2] = ((x as u8).wrapping_add(2 * (y as u8)) % 255).wrapping_add(64);  // Blue
                rgba_data[offset + 3] = 255;  // Alpha
            }
        }
    }

    rgba_data
}

