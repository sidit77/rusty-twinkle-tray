#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod config;
mod interface;
mod monitors;
mod power;
pub mod runtime;
mod theme;
mod ui;
mod utils;
mod windowing;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder};
use log::{LevelFilter, trace};
use windows::core::{h, IInspectable};
use windows::Foundation::TypedEventHandler;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows::UI::ViewManagement::UISettings;
use windows_ext::UI::Xaml::Controls::Control;
use windows_ext::UI::Xaml::ElementTheme;
use windows_ext::UI::Xaml::Hosting::WindowsXamlManager;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};

use crate::backend::{BackendEvent, MonitorController};
use crate::config::Config;
use crate::interface::XamlGui;
use crate::power::{PowerEvent, PowerStateListener};
use crate::theme::{ColorSet, SystemSettings};
use crate::ui::container::StackPanel;
use crate::ui::controls::{Flyout, FlyoutPlacementMode, TextBlock};
pub use crate::utils::error::Result;
use crate::utils::extensions::{ChannelExt, MutexExt};
use crate::utils::{logger, panic};
use crate::windowing::{event_loop, get_primary_work_area, WindowBuilder};

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

    let (wnd_sender, wnd_receiver) = flume::unbounded();
    let controller = MonitorController::new(wnd_sender.clone(), config.clone());

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

    let proxy_window = WindowBuilder::default()
        .with_hidden(true)
        .build()?;

    let _power_listener = PowerStateListener::new({
        let proxy = controller.create_proxy();
        move |event| {
            if event == PowerEvent::MonitorOn {
                proxy.refresh_brightness_in(Duration::from_secs(10));
            }
        }
    })?;

    ui_settings.ColorValuesChanged(&TypedEventHandler::new(cloned!([wnd_sender] move |_: &Option<UISettings>, _| {
        wnd_sender.filter_send_ignore(Some(CustomEvent::ThemeChange));
        Ok(())
    })))?;

    let mut gui = XamlGui::new()?;

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
    let mut failed_foregound_workaround = false;
    event_loop(async {
        while let Ok(event) = wnd_receiver.recv_async().await {
            match event {
                CustomEvent::Quit => {
                    controller.shutdown();
                    return Ok(());
                }
                CustomEvent::Show => {
                    //if failed_foregound_workaround {
                    //    println!("Show -> Hide");
                    //    let _ = wnd_sender.try_send(CustomEvent::FocusLost);
                    //    continue;
                    //}
                    if flyout.is_open()? {
                        trace!("Flyout already open. Closing to instead");
                        flyout.close()?;
                        continue;
                    }
                    if last_close.elapsed() >= Duration::from_millis(250) {
                        proxy_window.set_visible(true);
                        proxy_window.focus();
                        if !proxy_window.set_foreground() {
                            log::debug!("Failed to set window foreground");
                            failed_foregound_workaround = true;
                        }
                        let idpi = 1.0 / proxy_window.dpi();
                        let workspace = get_primary_work_area()?;
                        let gap = 13;
                        flyout.show_at(
                            &proxy_content,
                            (workspace.right - gap) as f32 * idpi,
                            (workspace.bottom - gap) as f32 * idpi,
                            FlyoutPlacementMode::LeftEdgeAlignedBottom
                        )?;
                        //println!("Show");
                        controller.refresh_brightness();
                    }
                }
                CustomEvent::FocusLost => {
                    //println!("HIde");
                    if failed_foregound_workaround {
                        //let _ = wnd_sender.send(CustomEvent::Show);
                        failed_foregound_workaround = false;
                        //continue;
                    }
                    proxy_window.set_visible(false);
                    config.lock_no_poison().save_if_dirty()?;
                    last_close = Instant::now();

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
