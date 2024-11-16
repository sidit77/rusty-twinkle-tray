#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod config;
mod interface;
mod monitors;
pub mod runtime;
mod theme;
mod ui;
mod utils;
mod windowing;
mod watchers;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder};
use futures_lite::stream::or;
use futures_lite::{FutureExt, StreamExt};
use log::{trace, LevelFilter};
use windows::core::{h, IInspectable};
use windows::Foundation::{Size, TypedEventHandler};
use windows::Win32::Foundation::RECT;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows::UI::ViewManagement::UISettings;
use windows_ext::UI::Xaml::Controls::Control;
use windows_ext::UI::Xaml::ElementTheme;
use windows_ext::UI::Xaml::Hosting::WindowsXamlManager;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};

use crate::backend::{BackendEvent, MonitorController};
use crate::config::Config;
use crate::interface::XamlGui;
use crate::runtime::{FutureStream, Timer};
use crate::theme::{ColorSet, SystemSettings};
use crate::ui::container::StackPanel;
use crate::ui::controls::{Flyout, FlyoutPlacementMode, TextBlock};
pub use crate::utils::error::Result;
use crate::utils::extensions::{ChannelExt, MutexExt};
use crate::utils::{logger, panic};
use crate::watchers::{EventWatcher, PowerEvent};
use crate::windowing::{event_loop, get_primary_work_area, poll_for_click_outside_of_rect, WindowBuilder, WindowLevel};

include!("../assets/ids.rs");

#[derive(Debug, Clone)]
pub enum CustomEvent {
    Quit,
    Show,
    FocusLost,
    ThemeChange,
    Backend(BackendEvent),
    ClickedOutside,
    Refresh
}

fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };

    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    let config = Arc::new(Mutex::new(Config::load()?));

    let (wnd_sender, wnd_receiver) = flume::unbounded();
    let mut controller = MonitorController::new(wnd_sender.clone(), config.clone());

    let ui_settings = UISettings::new()?;
    let mut colors = SystemSettings::new()
        .map_err(|err| log::warn!("Failed to read system settings: {err}"))
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
        .on_display_change(|| println!("Display change"))?;

    /*
    unsafe {
        unsafe extern "system" fn callback(hnotify: HCMNOTIFICATION, context: *const c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32 {
            match action {
                CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL => println!("CM Device Arrival"),
                CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL => println!("CM Device Removal"),
                CM_NOTIFY_ACTION_DEVICEQUERYREMOVE => println!("CM Device Query Remove"),
                CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED => println!("CM Device Query Remove Failed"),
                CM_NOTIFY_ACTION_DEVICEREMOVEPENDING => println!("CM Device Remove Pending"),
                CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE => println!("CM Device Remove Complete"),
                CM_NOTIFY_ACTION_DEVICECUSTOMEVENT => println!("CM Device Custom Event"),
                CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED => println!("CM Device Instance Enumerated"),
                CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED => println!("CM Device Instance Started"),
                CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED => println!("CM Device Instance Removed"),
                _ => println!("CM Unknown Action")
            }
            ERROR_SUCCESS.0
        }

        let mut handle = std::mem::zeroed();
        assert_eq!(CM_Register_Notification(
            &CM_NOTIFY_FILTER {
                cbSize: std::mem::size_of::<CM_NOTIFY_FILTER>() as u32,
                Flags: CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES,
                FilterType: CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE,
                Reserved: 0,
                u: zeroed(),
            },
            None,
            Some(callback),
            &mut handle
        ), CR_SUCCESS);
    }
     */
    ui_settings.ColorValuesChanged(&TypedEventHandler::new(cloned!([wnd_sender] move |_: &Option<UISettings>, _| {
        wnd_sender.filter_send_ignore(Some(CustomEvent::ThemeChange));
        Ok(())
    })))?;

    let mut gui = XamlGui::new(wnd_sender.clone())?;

    let proxy_window = WindowBuilder::default().with_hidden(true).build()?;
    let proxy_content = TextBlock::new()?.with_text("Hello World")?;
    proxy_window.set_content(&proxy_content)?;

    let content = StackPanel::vertical()?
        .with_theme(colors.theme)?
        .with_width(400.0)?
        .with_child(gui.ui())?;

    let background_brush = {
        let brush = AcrylicBrush::new()?;
        brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
        brush.SetFallbackColor(colors.fallback)?;
        brush.SetTintColor(colors.tint)?;
        brush.SetTintOpacity(colors.opacity)?;
        brush
    };

    let flyout = Flyout::new(&content)?
        .with_style(|style| {
            style
                .with_setter(&Control::BackgroundProperty()?, &background_brush)?
                .with_setter(&Control::CornerRadiusProperty()?, &IInspectable::try_from(h!("10.0"))?)?
                .with_setter(&Control::PaddingProperty()?, &IInspectable::try_from(h!("0.0"))?)
        })?
        .with_closed_handler(cloned!([wnd_sender] move || {
            wnd_sender.filter_send_ignore(Some(CustomEvent::FocusLost));
            Ok(())
        }))?;

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
                    if flyout.is_open()? {
                        trace!("Flyout already open. Closing it instead");
                        flyout.close()?;
                        continue;
                    }
                    if last_close.elapsed() >= Duration::from_millis(250) {
                        let idpi = 1.0 / proxy_window.dpi();
                        let workspace = get_primary_work_area()?;
                        let gap = 13;
                        let right = workspace.right - gap;
                        let bottom = workspace.bottom - gap;

                        proxy_window.set_window_pos(Some(WindowLevel::AlwaysOnTop), Some((0, 0)), None, Some(true));
                        proxy_window.focus();
                        if !proxy_window.set_foreground() {
                            // This happens when opening the flyout while the start menu is open
                            log::debug!("Failed to set window foreground");
                            let size = content.measure().unwrap_or_else(|err| {
                                log::warn!("Failed to measure content: {err}");
                                // Make a guess
                                Size {
                                    Width: 400.0,
                                    Height: 62.0 + 86.0 * gui.number_of_monitors() as f32
                                }
                            });
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

                        flyout.show_at(
                            &proxy_content,
                            right as f32 * idpi,
                            bottom as f32 * idpi,
                            FlyoutPlacementMode::LeftEdgeAlignedBottom
                        )?;
                        controller.refresh_brightness();
                    }
                }
                CustomEvent::FocusLost => {
                    click_watcher.clear();
                    proxy_window.set_window_pos(None, None, None, Some(false));
                    config.lock_no_poison().save_if_dirty()?;
                    last_close = Instant::now();
                }
                CustomEvent::ClickedOutside => {
                    flyout.close()?;
                }
                CustomEvent::ThemeChange => {
                    colors = SystemSettings::new()
                        .map_err(|err| log::warn!("Failed to read system settings: {err}"))
                        .ok()
                        .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));
                    background_brush.SetFallbackColor(colors.fallback)?;
                    background_brush.SetTintColor(colors.tint)?;
                    background_brush.SetOpacity(colors.opacity)?;
                    content.set_theme(colors.theme)?;
                    tray.set_icon(Icon::from_resource(
                        if colors.theme == ElementTheme::Light {
                            BRIGHTNESS_LIGHT_ICON
                        } else {
                            BRIGHTNESS_DARK_ICON
                        },
                        None
                    )?);
                }
                CustomEvent::Refresh => {
                    log::info!("Restarting backend...");
                    gui.clear_monitors()?;
                    controller = MonitorController::new(wnd_sender.clone(), config.clone());
                }
                CustomEvent::Backend(event) => match event {
                    BackendEvent::RegisterMonitor(name, path) => {
                        log::info!("Found monitor: {}", name);
                        gui.register_monitor(name, path, controller.create_proxy())?
                    }
                    BackendEvent::UpdateBrightness(path, value) => {
                        gui.update_brightness(path, value)?;
                    }
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
