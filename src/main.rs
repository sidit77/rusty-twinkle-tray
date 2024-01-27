#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod utils;
mod ui;
mod interface;
mod backend;
mod config;
mod power;
mod theme;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder, winit::WinitTrayIconBuilderExt};
use log::LevelFilter;
use windows::Foundation::EventHandler;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows_ext::UI::Xaml::Hosting::{WindowsXamlManager};
use windows_ext::UI::Xaml::Input::{FocusManager, LosingFocusEventArgs};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{DeviceEvents, EventLoopBuilder};
use winit::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
use winit::window::{WindowBuilder, WindowButtons, WindowLevel};
use crate::backend::MonitorController;
use crate::config::Config;
use crate::interface::{XamlGui};
use crate::monitors::MonitorPath;
use crate::power::{PowerEvent, PowerStateListener};

use crate::utils::error::{OptionExt};
use crate::utils::{logger, panic};
use crate::utils::extensions::{BorderColor, EventLoopExt, MonitorHandleExt, MutexExt, WindowExt};

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

    //println!("{}", unsafe { GetImmersiveUserColorSetPreference(false, false)});
//
    //let mut colors = (0..)
    //    .into_iter()
    //    .map(|i| unsafe {GetImmersiveColorNamedTypeByIndex(i)
    //        .as_ref()
    //        .copied()});
    //while let Some(name) = colors.next().flatten() {
    //    unsafe {
    //        println!("{}: #{:X}", name.display(), lookup_color(name));
    //    }
    //}
    //println!("\n#{:X}", unsafe { lookup_color(w!("ImmersiveLight"))});
//
    //println!("{} {}", get_theme_setting(w!("ColorPrevalence")), get_theme_setting(w!("SystemUsesLightTheme")));

    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let config = Arc::new(Mutex::new(Config::load()?));

    let event_loop = EventLoopBuilder::with_user_event()
        .build()?;
    event_loop.listen_device_events(DeviceEvents::Never);

    let controller = MonitorController::new(&event_loop, config.clone());

    let _tray = TrayIconBuilder::new()
        .with_tooltip("Change Brightness")
        .with_icon(Icon::from_resource(32512, None)?)
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
        .with_drag_and_drop(false)
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
    let _power_listener = PowerStateListener::new({
        let proxy = controller.create_proxy();
        move |event| match event {
            PowerEvent::MonitorOn => proxy.refresh_brightness_in(Duration::from_secs(10)),
            _ => {}
        }
    })?;
    window.set_border_color(BorderColor::NONE);
    //Drop input focus
    window.set_enable(false);

    let mut last_close = Instant::now();
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
                CustomEvent::Show => if last_close.elapsed() >= Duration::from_millis(250) {
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
                    last_close = Instant::now();
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

