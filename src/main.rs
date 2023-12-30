#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod utils;
mod ui;
mod interface;

use std::process::ExitCode;

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
use winit::window::{WindowBuilder, WindowButtons};
use crate::interface::{DetectedMonitor, XamlGui};
use crate::monitors::Monitor;

use crate::utils::error::OptionExt;
use crate::utils::{logger, panic};
use crate::utils::extensions::MonitorHandleExt;

pub use crate::utils::error::Result;

#[derive(Debug, Copy, Clone)]
enum CustomEvent {
    Quit,
    Show,
    FocusLost
}

fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let monitors = Monitor::find_all()?;

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

    let gui = XamlGui::new(&window, monitors
        .iter()
        .map(|m| DetectedMonitor {
            name: m.name().to_string(),
            path: m.path().to_path_buf(),
            current_brightness: 0,
        })
        .collect()
    )?;

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
                WindowEvent::Resized(new) => gui.resize(new).unwrap(),
                WindowEvent::Focused(true) => gui.focus(),
                _ => {}
            },
            Event::UserEvent(event) => match event {
                CustomEvent::Quit => target.exit(),
                CustomEvent::Show => {
                    if let Some(workspace) = target.primary_monitor().and_then(|m| m.get_work_area().ok()) {
                        let _ = window.request_inner_size(PhysicalSize::new(
                            window.outer_size().width,
                            gui.get_required_height().unwrap()));

                        let gap = 14;
                        let size = window.outer_size();

                        window.set_outer_position(PhysicalPosition::new(
                            workspace.right - gap - size.width as i32,
                            workspace.bottom - gap - size.height as i32
                        ));

                        window.set_visible(true);
                        window.focus_window();
                    } else {
                        log::warn!("Can't find work area of primary monitor");
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

