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
mod windowing;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use betrayer::{ClickType, Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder};
use log::LevelFilter;
use windows::core::{h, IInspectable};
use windows::Foundation::{EventHandler, Point, TypedEventHandler};
use windows::UI::ViewManagement::UISettings;
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_SINGLETHREADED};
use windows_ext::UI::Xaml::Hosting::WindowsXamlManager;
use windows_ext::UI::Xaml::Controls::{Control, Flyout, FlyoutPresenter};
use windows_ext::UI::Xaml::Controls::Primitives::{FlyoutPlacementMode, FlyoutShowOptions};
use windows_ext::UI::Xaml::{ElementTheme, Setter, Style};
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};
use crate::backend::{BackendEvent, MonitorController};
use crate::config::Config;
use crate::interface::{XamlGui};
use crate::power::{PowerEvent, PowerStateListener};
use crate::theme::{ColorSet, SystemSettings};
use crate::ui::container::StackPanel;
use crate::ui::controls::TextBlock;
use crate::ui::NewType;

use crate::utils::{logger, panic};
use crate::utils::extensions::{ChannelExt, MutexExt};

pub use crate::utils::error::Result;
use crate::utils::winrt::{GetTypeName, Reference};
use crate::windowing::{event_loop, get_primary_work_area, ProxyWindow};

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

    //let proxy_window = ProxyWindow::new()?;
    //event_loop();
    //return Ok(());

    let config = Arc::new(Mutex::new(Config::load()?));

    //let event_loop = EventLoopBuilder::with_user_event()
    //    .build()?;
    //event_loop.listen_device_events(DeviceEvents::Never);

    let (wnd_sender, wnd_receiver) = flume::unbounded();
    let controller = MonitorController::new(wnd_sender.clone(), config.clone());


    let ui_settings = UISettings::new()?;
    let mut colors = SystemSettings::new()
        .map_err(|err| log::warn!("Failed to read system settings: {err}"))
        .ok()
        .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));

    let tray = TrayIconBuilder::new()
        .with_tooltip("Change Brightness")
        .with_icon(Icon::from_resource(if colors.theme == ElementTheme::Light { BRIGHTNESS_LIGHT_ICON } else { BRIGHTNESS_DARK_ICON}, None)?)
        .with_menu(Menu::new([MenuItem::button("Quit", CustomEvent::Quit)]))
        .build(cloned!([wnd_sender] move |event| wnd_sender.filter_send_ignore(match event {
            TrayEvent::Tray(ClickType::Left) => Some(CustomEvent::Show),
            TrayEvent::Menu(e) => Some(e),
            _ => None
        })))?;

    //let window = WindowBuilder::new()
    //    .with_title("XAML Window")
    //    //Make the window "invisible" to hide the creation without breaking stuff
    //    .with_position(PhysicalPosition::new(0, 0))
    //    .with_inner_size(PhysicalSize::new(30, 30))
    //    .with_no_redirection_bitmap(true)
    //    .with_decorations(true)
    //    .with_undecorated_shadow(true)
    //    .with_skip_taskbar(true)
    //    .with_visible(false)
    //    .with_resizable(false)
    //    .with_drag_and_drop(false)
    //    .with_enabled_buttons(WindowButtons::empty())
    //    .build(&event_loop)?;

    let proxy_window = ProxyWindow::new()?;

    //FocusManager::LosingFocus(&EventHandler::new({
    //    let proxy = event_loop.create_proxy();
    //    move |_e, arg: &Option<LosingFocusEventArgs>| {
    //        if arg.as_ref().some()?.NewFocusedElement().is_err() {
    //            proxy
    //                .send_event(CustomEvent::FocusLost)
    //                .unwrap_or_else(|err| log::warn!("Failed to forward event: {}", err));
    //        }
    //        Ok(())
    //    }
    //}))?;
    let _power_listener = PowerStateListener::new({
        let proxy = controller.create_proxy();
        move |event| if event == PowerEvent::MonitorOn {
            proxy.refresh_brightness_in(Duration::from_secs(10));
        }
    })?;

    ui_settings.ColorValuesChanged(&TypedEventHandler::new(cloned!([wnd_sender] move |_: &Option<UISettings>, _| {
        wnd_sender.filter_send_ignore(Some(CustomEvent::ThemeChange));
        Ok(())
    })))?;

    let mut gui = XamlGui::new()?;

    let main_content = TextBlock::new()?
        .with_text("Hello World")?;
    proxy_window.set_content(&main_content)?;

    let content = StackPanel::vertical()?
        .with_theme(colors.theme)?
        .apply(|p| p.SetWidth(400.0))?
        //.apply(|p| p.SetHeight(250.0))?
        .with_child(gui.ui())?;


    let background_brush = {
        let brush = AcrylicBrush::new()?;
        brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
        brush.SetFallbackColor(colors.fallback)?;
        brush.SetTintColor(colors.tint)?;
        brush.SetTintOpacity(colors.opacity)?;
        brush.SetTintOpacity(0.1)?;
        brush
    };

    let flyout_style = Style::CreateInstance(&FlyoutPresenter::type_name())?;
    flyout_style.Setters()?.Append(&Setter::CreateInstance(&Control::BackgroundProperty()?, &background_brush)?)?;
    flyout_style.Setters()?.Append(&Setter::CreateInstance(&Control::CornerRadiusProperty()?, &IInspectable::try_from(h!("10.0"))?)?)?;
    flyout_style.Setters()?.Append(&Setter::CreateInstance(&Control::PaddingProperty()?, &IInspectable::try_from(h!("0.0"))?)?)?;

    let flyout = Flyout::new()?;
    flyout.SetContent(&content)?;
    flyout.SetFlyoutPresenterStyle(&flyout_style)?;
    flyout.SetShouldConstrainToRootBounds(false)?;
    flyout.SetAreOpenCloseAnimationsEnabled(true)?;
    flyout.Closed(&EventHandler::new(cloned!([wnd_sender] move |_, _| {
        wnd_sender.filter_send_ignore(Some(CustomEvent::FocusLost));
        Ok(())
    })))?;
    let mut last_close = Instant::now();
    let mut failed_foregound_workaround = false;
    event_loop(async {
        while let Ok(event) = wnd_receiver.recv_async().await {
            match event {
                CustomEvent::Quit => {
                    controller.shutdown();
                    return Ok(())
                },
                CustomEvent::Show => if last_close.elapsed() >= Duration::from_millis(250) {
                    proxy_window.set_visible(true);
                    proxy_window.focus();
                    if !proxy_window.set_foreground() {
                        log::debug!("Failed to set window foreground");
                        failed_foregound_workaround = true;
                    }
                    let workspace = get_primary_work_area()?;
                    let gap = 13;
                    let options = FlyoutShowOptions::new()?;
                    let pt = Point {
                        X: (workspace.right - gap) as f32,
                        Y: (workspace.bottom - gap) as f32,
                    };
                    options.SetPlacement(FlyoutPlacementMode::LeftEdgeAlignedBottom)?;
                    options.SetPosition(&Reference::box_value(pt))?;
                    flyout.ShowAt2(main_content.as_inner(), &options)?;
                    controller.refresh_brightness();
                }
                CustomEvent::FocusLost => {
                    if failed_foregound_workaround {
                        let _ = wnd_sender.send(CustomEvent::Show);
                        failed_foregound_workaround = false;
                        continue;
                    }
                    proxy_window.set_visible(false);
                    config.lock_no_poison().save_if_dirty()?;
                    last_close = Instant::now();
                },
                CustomEvent::ThemeChange => {
                    colors = SystemSettings::new()
                        .map_err(|err| log::warn!("Failed to read system settings: {err}"))
                        .ok()
                        .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));
                    background_brush.SetFallbackColor(colors.fallback)?;
                    background_brush.SetTintColor(colors.tint)?;
                    background_brush.SetOpacity(colors.opacity)?;
                    content.set_theme(colors.theme)?;
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
            }
        }
        Ok(())
    })?;
    /*
    event_loop
        .run_result(|event, target| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    //WindowEvent::Resized(new) => gui.resize(new)?,
                    WindowEvent::Resized(new_size) => {

                    }
                    //WindowEvent::Focused(true) => gui.focus(),
                    _ => {}
                },
                Event::UserEvent(event) => match event {
                    CustomEvent::Quit => {
                        // controller.shutdown();
                        target.exit()
                    },
                    CustomEvent::Show => if last_close.elapsed() >= Duration::from_millis(250) {

                        window.set_visible(true);
                        // window.set_window_level(WindowLevel::AlwaysOnTop);
                        window.focus_window();

                        let options = FlyoutShowOptions::new()?;
                        let pt = Point {
                            X: 1600.0,
                            Y: 990.0,
                        };
                        options.SetPosition(&Reference::box_value(pt))?;
                        flyout.ShowAt2(main_content.as_inner(), &options)?;
                        println!("Show");

                        // controller.refresh_brightness();
                        //unsafe {
                        //    //SetWindowPos(island, HWND::default(), 0, 0, 0, 0, SWP_NOZORDER | SWP_SHOWWINDOW)?;
                        //    SetWindowPos(window.hwnd(), HWND_TOPMOST, 0, 0, 100, 100, SWP_SHOWWINDOW)?;
                        //    SetForegroundWindow(window.hwnd());
                        //}

                        //options.SetPosition(&iref)?;
                        //flyout.ShowAt2(&main_content, &options)?;
                        //if let Some(workspace) = target.primary_monitor().and_then(|m| m.get_work_area().ok()) {
                        //    if let Ok(height) = gui.get_required_height()
                        //        .map_err(|err| log::debug!("Failed to get required height: {err}"))
                        //    {
                        //        let width = window.outer_size()
                        //            .to_logical::<f32>(window.scale_factor())
                        //            .width as u32;
                        //        let _ = window.request_inner_size(LogicalSize::new(
                        //            width, height));
                        //    }
//
                        //    let gap = 14;
                        //    let size = window.outer_size();
//
                        //    window.set_outer_position(PhysicalPosition::new(
                        //        workspace.right - gap - size.width as i32,
                        //        workspace.bottom - gap - size.height as i32
                        //    ));
//
                        //    window.set_visible(true);
                        //    window.set_window_level(WindowLevel::AlwaysOnTop);
                        //    window.focus_window();
                        //} else {
                        //    log::warn!("Can't find work area of primary monitor");
                        //}
                    }
                    CustomEvent::FocusLost => {
                        println!("Closed");
                        window.set_visible(false);
                        //window.set_visible(false);
                        //config.lock_no_poison().save_if_dirty()?;
                        last_close = Instant::now();
                    },
                    CustomEvent::ThemeChange => {
                        colors = SystemSettings::new()
                            .map_err(|err| log::warn!("Failed to read system settings: {err}"))
                            .ok()
                            .map_or_else(ColorSet::dark, |system_settings| ColorSet::system(&system_settings, &ui_settings));
                        //gui.update_theme(&colors)
                        //    .unwrap_or_else(|err| log::warn!("Failed to update gui theme: {err}"));
                        tray.set_icon(Icon::from_resource(if colors.theme == ElementTheme::Light { BRIGHTNESS_LIGHT_ICON } else { BRIGHTNESS_DARK_ICON}, None)?);
                    }
                    CustomEvent::Backend(event) => match event {
                        BackendEvent::RegisterMonitor(name, path) => {
                            log::info!("Found monitor: {}", name);
                            //gui.register_monitor(name, path, controller.create_proxy())?
                        },
                        BackendEvent::UpdateBrightness(path, value) => {
                            //gui.update_brightness(path, value)?;
                        }
                    }

                },
                _ => {}
            }
            Ok(())
        })?;
    */
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

