#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod config;
mod monitors;
pub mod runtime;
mod theme;
mod ui;
mod utils;
mod views;
mod watchers;
mod windowing;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder};
use futures_lite::stream::or;
use futures_lite::{FutureExt, StreamExt};
use log::{debug, info, trace, warn, LevelFilter};
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::RECT;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows::UI::ViewManagement::UISettings;
use windows_ext::UI::Xaml::ElementTheme;
use windows_ext::UI::Xaml::Hosting::WindowsXamlManager;

use crate::backend::MonitorController;
use crate::config::{autostart, Config};
use crate::monitors::MonitorPath;
use crate::runtime::{FutureStream, Timer};
use crate::theme::{ColorSet, SystemSettings};
pub use crate::utils::error::Result;
use crate::utils::extensions::{ChannelExt, MutexExt};
use crate::utils::{logger, panic};
use crate::views::{BrightnessFlyout, ProxyWindow, SettingsWindow};
use crate::watchers::{EventWatcher, PowerEvent};
use crate::windowing::{event_loop, get_primary_work_area, poll_for_click_outside_of_rect};

include!("../assets/ids.rs");

#[derive(Debug, Clone)]
pub enum CustomEvent {
    Quit,
    Show,
    FocusLost,
    ThemeChange,
    ClickedOutside,
    Refresh,
    OpenSettings,
    CloseSettings,
    MonitorAdded { path: MonitorPath, name: String },
    MonitorRemoved { path: MonitorPath },
    BrightnessChanged { path: MonitorPath, value: u32 }
}

fn run() -> Result<()> {
    // For changing the autostart setting with an elevated instance
    if let Some(arg) = std::env::args()
        .skip_while(|arg| arg != "--config-autostart")
        .nth(1)
    {
        match arg.as_str() {
            "enable" => autostart::set_enabled(false, true)?,
            "disable" => autostart::set_enabled(false, false)?,
            _ => panic!("Invalid argument: {}", arg)
        }
        return Ok(());
    }
    let settings_mode = std::env::args().any(|arg| arg == "--settings-mode");

    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };

    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let config = Arc::new(Mutex::new(Config::restore()?));

    let (wnd_sender, wnd_receiver) = loole::unbounded();
    let mut controller = MonitorController::new(wnd_sender.clone(), config.clone());

    let ui_settings = UISettings::new()?;
    let mut colors = SystemSettings::new()
        .map_err(|err| warn!("Failed to read system settings: {err}"))
        .ok()
        .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));

    let tray = TrayIconBuilder::new()
        .with_tooltip("Change Brightness")
        .with_icon(Icon::from_resource(
            if colors.theme == ElementTheme::Light {
                BRIGHTNESS_LIGHT_ICON
            } else {
                BRIGHTNESS_DARK_ICON
            },
            None
        )?)
        .with_menu(Menu::new([MenuItem::button("Quit", CustomEvent::Quit)]))
        .build(cloned!([wnd_sender] move |event| wnd_sender.filter_send_ignore(match event {
            TrayEvent::Tray(ClickType::Left) => Some(CustomEvent::Show),
            TrayEvent::Menu(e) => Some(e),
            _ => None
        })))?;

    let _event_watcher = EventWatcher::new()?
        .on_power_event({
            let proxy = controller.create_proxy();
            move |event| {
                if event == PowerEvent::MonitorOn {
                    proxy.refresh_brightness_in(Duration::from_secs(10));
                }
            }
        })?
        .on_display_change({
            let proxy = controller.create_proxy();
            move || proxy.refresh_monitors()
        })?;

    ui_settings.ColorValuesChanged(&TypedEventHandler::new(cloned!([wnd_sender] move |_: &Option<UISettings>, _| {
        wnd_sender.filter_send_ignore(Some(CustomEvent::ThemeChange));
        Ok(())
    })))?;

    let proxy_window = ProxyWindow::new()?;
    let mut flyout = BrightnessFlyout::new(wnd_sender.clone(), &colors)?;
    let mut settings_window: Option<SettingsWindow> = None;

    if settings_mode {
        wnd_sender.send(CustomEvent::OpenSettings).unwrap();
    }

    let mut last_close = Instant::now();
    event_loop(async {
        let mut click_watcher = FutureStream::new();
        let mut events = wnd_receiver.into_stream();
        while let Some(event) = or(&mut events, &mut click_watcher).next().await {
            match event {
                CustomEvent::Quit => {
                    controller.shutdown();
                    return Ok(());
                }
                CustomEvent::Show => {
                    if flyout.is_open() {
                        trace!("Flyout already open. Closing it instead");
                        flyout.close();
                        continue;
                    }
                    if last_close.elapsed() >= Duration::from_millis(250) {
                        let idpi = 1.0 / proxy_window.dpi();
                        let workspace = get_primary_work_area()?;
                        let gap = 13;
                        let right = workspace.right - gap;
                        let bottom = workspace.bottom - gap;

                        if !proxy_window.activate() {
                            // This happens when opening the flyout while the start menu is open
                            debug!("Failed to set window foreground");
                            let size = flyout.size();
                            click_watcher.set(async move {
                                trace!("Calculated flyout size: {:?}", size);
                                let click = async {
                                    let rect = RECT {
                                        left: right - (size.Width / idpi) as i32,
                                        top: bottom - (size.Height / idpi) as i32,
                                        right,
                                        bottom
                                    };
                                    // We actively poll for clicks outside the flyout for simplicity
                                    // and because low level mouse hooks don't work when the focused application is elevated
                                    poll_for_click_outside_of_rect(Duration::from_millis(100), rect).await;
                                    Some(CustomEvent::ClickedOutside)
                                };
                                let timeout = async {
                                    // Don't waste energy if the flyout is left open
                                    Timer::after(Duration::from_secs(30)).await;
                                    trace!("Canceling click watcher");
                                    None
                                };
                                click.or(timeout).await
                            });
                        }

                        flyout.show(&proxy_window, right as f32 * idpi, bottom as f32 * idpi);
                        controller.refresh_brightness();
                    }
                }
                CustomEvent::FocusLost => {
                    click_watcher.clear();
                    proxy_window.deactivate();
                    config.lock_no_poison().save_if_dirty()?;
                    last_close = Instant::now();
                }
                CustomEvent::ClickedOutside => {
                    flyout.close();
                }
                CustomEvent::ThemeChange => {
                    colors = SystemSettings::new()
                        .map_err(|err| log::warn!("Failed to read system settings: {err}"))
                        .ok()
                        .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));
                    flyout.update_theme(&colors)?;
                    tray.set_icon(Icon::from_resource(
                        if colors.theme == ElementTheme::Light {
                            BRIGHTNESS_LIGHT_ICON
                        } else {
                            BRIGHTNESS_DARK_ICON
                        },
                        None
                    )?);
                    if let Some(window) = settings_window.as_ref() {
                        window
                            .sync_theme()
                            .unwrap_or_else(|e| warn!("Failed to sync theme: {e}"));
                    }
                }
                CustomEvent::Refresh => {
                    log::info!("Restarting backend...");
                    flyout.clear_monitors()?;
                    controller = MonitorController::new(wnd_sender.clone(), config.clone());
                }
                CustomEvent::OpenSettings => {
                    log::info!("Open Settings");
                    if settings_window.is_none() {
                        trace!("Creating settings window");

                        settings_window = Some(SettingsWindow::new(wnd_sender.clone(), config.clone())?);
                    }
                    if let Some(window) = settings_window.as_ref() {
                        window.focus();
                    }
                }
                CustomEvent::CloseSettings => {
                    settings_window = None;
                    config.lock_no_poison().save_if_dirty()?;
                    if settings_mode {
                        return Ok(());
                    }
                }
                CustomEvent::MonitorAdded { path, name } => {
                    let mut lock = config.lock_no_poison();
                    let settings = lock.monitor(&path);
                    let display_name = settings.custom_name.clone().unwrap_or_else(|| name.clone());
                    let position = settings.position.unwrap_or(0);
                    info!("Found monitor: {} [{}]", name, display_name);
                    flyout.register_monitor(display_name, path, position, controller.create_proxy())?;
                }
                CustomEvent::MonitorRemoved { path } => {
                    info!("Monitor removed: {:?}", path);
                    flyout.unregister_monitor(&path)?;
                }
                CustomEvent::BrightnessChanged { path, value } => {
                    flyout.update_brightness(path, value)?;
                }
            }
        }
        Ok(())
    })?;

    controller.shutdown();
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
