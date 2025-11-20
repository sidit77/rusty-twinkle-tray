use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use log::warn;
use loole::Sender;
use windows::core::{ComInterface, IInspectable, HSTRING};
use windows::System::VirtualKey;
use windows::UI::Color;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT};
use windows_ext::IXamlSourceTransparency;
use windows_ext::UI::Xaml::Media::SolidColorBrush;
use windows_ext::UI::Xaml::{ElementTheme, FocusState, RoutedEventHandler, TextAlignment, Thickness, VerticalAlignment, Window as XamlWindow};
use windows_ext::UI::Xaml::Controls::{Button, ClickMode, TextBox};
use windows_ext::UI::Xaml::Input::KeyEventHandler;
use crate::config::{autostart, Config};
use crate::ui::container::StackPanel;
use crate::ui::controls::{TextBlock, ToggleSwitch};
use crate::ui::FontWeight;
use crate::utils::elevation::relaunch_as_elevated;
use crate::utils::extensions::{ChannelExt, FunctionalExt, MutexExt};
use crate::windowing::{Window, WindowBuilder};
use crate::{cloned, CustomEvent, Result, APP_ICON};
use crate::windowing::hotkey::Modifier;
use std::fmt::Write;
use std::ops::Deref;
use crate::utils::error::TracedError;

thread_local! {
    static TRANSPARENT_BACKGROUND: bool = XamlWindow::Current()
                            .and_then(|w| w.cast::<IXamlSourceTransparency>())
                            .and_then(|t| t.SetIsBackgroundTransparent(true))
                            .map_err(|e| warn!("Failed to make XAML island background transparent: {e}"))
                            .is_ok();
}

pub struct SettingsWindow {
    window: Window,
    mica: bool,
    content: Option<StackPanel>,
    background_brush: SolidColorBrush
}

impl SettingsWindow {
    pub fn new(sender: Sender<CustomEvent>, config: Arc<Mutex<Config>>) -> Result<Self> {
        let mut mica = TRANSPARENT_BACKGROUND.with(|t| *t);

        //mica = false;

        let window = WindowBuilder::default()
            .with_size(900, 800)
            .with_title("Rusty Twinkle Tray Settings")
            .with_icon_resource(APP_ICON)
            .with_close_handler(cloned!([sender] move || sender
                                .send(CustomEvent::CloseSettings)
                                .unwrap_or_default()))
            .build()?;
        if mica {
            mica &= window
                .apply_mica_backdrop()
                .map_err(|e| warn!("Failed to set DWM system backdrop attribute: {e}"))
                .is_ok();
        }
        if mica {
            window
                .make_titlebar_transparent()
                .unwrap_or_else(|e| warn!("Failed to set DWM caption color attribute: {e}"));
        }

        let mut result = Self {
            window,
            mica,
            content: None,
            background_brush: SolidColorBrush::new()?
        };
        result.build_gui(sender, config)?;
        result
            .sync_theme()
            .unwrap_or_else(|e| warn!("Failed to sync theme: {e}"));
        result.window.set_visible(true);

        Ok(result)
    }

    pub fn focus(&self) {
        self.window.focus();
    }

    pub fn sync_theme(&self) -> Result<()> {
        let theme = self
            .content
            .as_ref()
            .and_then(|c| c.theme().ok())
            .filter(|t| *t != ElementTheme::Default)
            .unwrap_or(ElementTheme::Light);

        let (color, dark) = match theme {
            ElementTheme::Dark => (Color { R: 45, G: 45, B: 45, A: 255 }, true),
            ElementTheme::Light => (
                Color {
                    R: 251,
                    G: 251,
                    B: 251,
                    A: 255
                },
                false
            ),
            _ => unreachable!()
        };

        self.background_brush.SetColor(color)?;
        if self.mica {
            self.window.enable_immersive_dark_mode(dark)?;
        }
        Ok(())
    }

    fn build_gui(&mut self, sender: Sender<CustomEvent>, config: Arc<Mutex<Config>>) -> Result<()> {
        const TOGGLE_WIDTH: f64 = 100.0;

        let border_brush = SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 30 })?;

        let section = |title| {
            StackPanel::vertical()?
                .apply_if(self.mica, |p| p.with_win11_style(&self.background_brush, &border_brush))?
                .with_padding(10.0)?
                .with_spacing(7.0)?
                .with_child(
                    &TextBlock::with_text(title)?
                        .with_font_size(24.0)?
                        .with_font_weight(FontWeight::SemiLight)?
                )
        };

        let auto_start_priority = config.lock_no_poison().use_prioritized_autostart;
        let auto_start_enabled = autostart::is_enabled(!auto_start_priority);

        let autostart_priority_toggle = ToggleSwitch::new()?
            .with_width(TOGGLE_WIDTH)?
            .with_state(auto_start_priority)?
            .with_enabled(!auto_start_enabled)?
            .with_toggled_handler(cloned!([config] move |ts | {
                let mut config = config.lock_no_poison();
                config.use_prioritized_autostart = ts.get_state()?;
                config.dirty = true;
                Ok(())
            }))?;

        let hwnd = self.window.hwnd();
        let auto_start_toggle = ToggleSwitch::new()?
            .with_width(TOGGLE_WIDTH)?
            .with_state(auto_start_enabled)?
            .with_toggled_handler(cloned!([config, autostart_priority_toggle] move |ts| {
                let user = !config.lock_no_poison().use_prioritized_autostart;
                match user {
                    true => autostart::set_enabled(true, ts.get_state()?)
                        .unwrap_or_else(|e| warn!("Failed to set autostart: {e}")),
                    false => relaunch_as_elevated(hwnd, match ts.get_state()? {
                        true => "--config-autostart enable",
                        false => "--config-autostart disable"
                    }).unwrap_or_else(|e| warn!("Failed to relaunch as elevated: {e}"))
                }
                let enabled = autostart::is_enabled(user);
                ts.set_state(enabled)?;
                autostart_priority_toggle.set_enabled(!enabled)?;
                Ok(())
            }))?;

        let enable_icon_scroll = ToggleSwitch::new()?
            .with_width(TOGGLE_WIDTH)?
            .with_state(config.lock_no_poison().icon_scoll_enabled)?
            .with_toggled_handler(cloned!([config, sender] move |ts | {
                let mut config = config.lock_no_poison();
                config.icon_scoll_enabled = ts.get_state()?;
                config.dirty = true;
                sender.send_ignore(CustomEvent::ReinitializeControls);
                Ok(())
            }))?;

        let make_hotkey_editor = || {
            let hotkey = TextBox::new()?;
            hotkey.SetIsReadOnly(true)?;
            hotkey.SetText(&HSTRING::from("CTRL + SHIFT + ALT"))?;
            hotkey.SetTextAlignment(TextAlignment::Center)?;
            hotkey.SetWidth(200.0)?;
            hotkey.SetMargin(Thickness {
                Left: 30.0,
                Top: 0.0,
                Right: 20.0,
                Bottom: 0.0,
            })?;
            let hk = hotkey.clone();
            hotkey.PreviewKeyDown(&KeyEventHandler::new(move |sender, args| {
                let args = args.unwrap();
                args.SetHandled(true)?;
                let key = args.Key()?;
                let modifier_keys = [
                    VirtualKey::LeftControl,
                    VirtualKey::RightControl,
                    VirtualKey::Control,
                    VirtualKey::LeftShift,
                    VirtualKey::RightShift,
                    VirtualKey::Shift,
                    VirtualKey::LeftMenu,
                    VirtualKey::RightMenu,
                    VirtualKey::Menu,
                    VirtualKey::LeftWindows,
                    VirtualKey::RightWindows
                ];
                if modifier_keys.contains(&key) {
                    return Ok(());
                }
                let modifiers = get_modifier_state();
                println!("{:?} {:?}", modifiers, key);
                let mut text = String::new();
                for m in modifiers {
                    let _ = write!(text, "{:?} + ", m);
                }
                let _ = write!(text, "{}", key.0);
                hk.SetText(&HSTRING::from(text.to_uppercase()))?;
                Ok(())
            }))?;
            Ok::<_, TracedError>(hotkey)
        };

        let hotkey_increase = make_hotkey_editor()?;
        let hotkey_decrease = make_hotkey_editor()?;
        //let hotkey_decrease = TextBlock::with_text("ALT")?;

        let hotkey_toggle = ToggleSwitch::new()?
            .with_width(TOGGLE_WIDTH)?
            .with_state(true)?
            .with_toggled_handler(cloned!([config, sender, hotkey_increase, hotkey_decrease] move |ts| {
                hotkey_increase.SetIsEnabled(ts.get_state()?)?;
                hotkey_decrease.SetIsEnabled(ts.get_state()?)?;
                sender.send_ignore(CustomEvent::ReinitializeControls);
                Ok(())
            }))?;

        let general = section("General")?
            .with_child(
                &StackPanel::horizontal()?
                    .with_child(&auto_start_toggle)?
                    .with_child(&TextBlock::with_text("Automatically run on startup")?.with_vertical_alignment(VerticalAlignment::Center)?)?
            )?
            .with_child(
                &StackPanel::horizontal()?
                    .with_child(
                        &ToggleSwitch::new()?
                            .with_width(TOGGLE_WIDTH)?
                            .with_state(config.lock_no_poison().restore_from_config)?
                            .with_toggled_handler(cloned!([config] move |state| {
                                let mut config = config.lock_no_poison();
                                config.restore_from_config = state.get_state()?;
                                config.dirty = true;
                                Ok(())
                            }))?
                    )?
                    .with_child(
                        &TextBlock::with_text("Automatically restore saved brightness")?.with_vertical_alignment(VerticalAlignment::Center)?
                    )?
            )?;

        let controls = section("Controls")?
            .with_child(
            &StackPanel::horizontal()?
                .with_child(&enable_icon_scroll)?
                .with_child(&TextBlock::with_text("Adjust the brightness of all displays by scrolling over the tray icon")?
                    .with_vertical_alignment(VerticalAlignment::Center)?)?
            )?
            .with_child(
                &StackPanel::vertical()?
                    .with_spacing(5.0)?
                    .with_child(
                        &StackPanel::horizontal()?
                            .with_child(&hotkey_toggle)?
                            .with_child(&TextBlock::with_text("Adjust the brightness of all displays by pressing the following hotkeys:")?
                                .with_vertical_alignment(VerticalAlignment::Center)?)?
                    )?
                    .with_child(
                        &StackPanel::horizontal()?
                            .with_child(&hotkey_increase)?
                            .with_child(&TextBlock::with_text("Increase brightness")?
                                .with_vertical_alignment(VerticalAlignment::Center)?)?
                    )?
                    .with_child(
                        &StackPanel::horizontal()?
                            .with_child(&hotkey_decrease)?
                            .with_child(&TextBlock::with_text("Decrease brightness")?
                                .with_vertical_alignment(VerticalAlignment::Center)?)?
                    )?
            )?;

        let advanced = section("Advanced")?.with_child(
            &StackPanel::horizontal()?
                .with_child(&autostart_priority_toggle)?
                .with_child(
                    &TextBlock::with_text("Use higher autostart priority (requires admin permissions)")?
                        .with_vertical_alignment(VerticalAlignment::Center)?
                )?
        )?;

        let main = StackPanel::vertical()?
            .apply_if(!self.mica, |p| p.with_background(&self.background_brush))?
            .with_padding(10.0)?
            .with_spacing(7.0)?
            .with_child(&general)?
            .with_child(&controls)?
            .with_child(&advanced)?;

        self.window.set_content(&main)?;
        self.content = Some(main);
        Ok(())
    }
}

trait StackPanelExt: Sized {
    fn with_win11_style(self, background: &SolidColorBrush, border: &SolidColorBrush) -> Result<Self>;
}

impl StackPanelExt for StackPanel {
    fn with_win11_style(self, background: &SolidColorBrush, border: &SolidColorBrush) -> Result<Self> {
        self.with_background(background)?
            .with_border_thickness(1.0)?
            .with_border_brush(border)?
            .with_corner_radius(5.0)
    }
}


fn get_modifier_state() -> HashSet<Modifier> {
    fn is_key_down(key: VIRTUAL_KEY) -> bool {
        unsafe { GetKeyState(key.0 as i32) & (1 << 15) != 0 }
    }

    const MODIFIER_KEYS: &[(Modifier, &[VIRTUAL_KEY])] = &[
        (Modifier::Shift, &[VK_SHIFT]),
        (Modifier::Ctrl, &[VK_CONTROL]),
        (Modifier::Alt, &[VK_MENU]),
        (Modifier::Win, &[VK_LWIN, VK_RWIN]),
    ];

    MODIFIER_KEYS
        .into_iter()
        .copied()
        .filter_map(|(m, keys)| keys
            .iter()
            .copied()
            .any(is_key_down)
            .then_some(m))
        .collect()
}
