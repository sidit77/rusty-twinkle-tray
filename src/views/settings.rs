use std::sync::{Arc, Mutex};
use log::warn;
use loole::Sender;
use windows::core::ComInterface;
use windows::UI::Color;
use windows_ext::IXamlSourceTransparency;
use windows_ext::UI::Xaml::{ElementTheme, VerticalAlignment, Window as XamlWindow};
use windows_ext::UI::Xaml::Media::SolidColorBrush;
use crate::windowing::{Window, WindowBuilder};
use crate::{cloned, CustomEvent, Result, APP_ICON};
use crate::config::{autostart, Config};
use crate::ui::container::StackPanel;
use crate::ui::controls::{TextBlock, ToggleSwitch};
use crate::ui::{FontWeight};
use crate::utils::extensions::{FunctionalExt, MutexExt};

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
    background_brush: SolidColorBrush,
}

impl SettingsWindow {
    pub fn new(sender: Sender<CustomEvent>, config: Arc<Mutex<Config>>) -> Result<Self> {
        let mut mica = TRANSPARENT_BACKGROUND.with(|t| *t);

        //mica = false;

        let window = WindowBuilder::default()
            .with_size(800, 800)
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
            background_brush: SolidColorBrush::new()?,
        };
        result.build_gui(config)?;
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
            ElementTheme::Light => (Color { R: 251, G: 251, B: 251, A: 255 }, false),
            _ => unreachable!(),
        };

        self.background_brush.SetColor(color)?;
        if self.mica {
            self.window.enable_immersive_dark_mode(dark)?;
        }
        Ok(())
    }

    fn build_gui(&mut self, config: Arc<Mutex<Config>>) -> Result<()> {
        const TOGGLE_WIDTH: f64 = 100.0;

        let border_brush = SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 30 })?;
         let general = StackPanel::vertical()?
            .apply_if(self.mica, |p| p
                .with_win11_style(&self.background_brush, &border_brush))?
            .with_padding(10.0)?
            .with_spacing(7.0)?
            .with_child(&TextBlock::with_text("General")?
                .with_font_size(24.0)?
                .with_font_weight(FontWeight::SemiLight)?)?
            .with_child(&StackPanel::horizontal()?
                .with_child(&ToggleSwitch::new()?
                    .with_width(TOGGLE_WIDTH)?
                    .with_state(autostart::is_enabled())?
                    .with_toggled_handler(|state| {
                        autostart::set_enabled(state)
                            .unwrap_or_else(|e| warn!("Failed to set autostart: {e}"));
                        Ok(())
                    })?)?
                .with_child(&TextBlock::with_text("Automatically run on startup")?
                    .with_vertical_alignment(VerticalAlignment::Center)?)?)?
             .with_child(&StackPanel::horizontal()?
                 .with_child(&ToggleSwitch::new()?
                     .with_width(TOGGLE_WIDTH)?
                     .with_state(config.lock_no_poison().restore_from_config)?
                     .with_toggled_handler(cloned!([config] move |state| {
                         let mut config = config.lock_no_poison();
                         config.restore_from_config = state;
                         config.dirty = true;
                         Ok(())
                     }))?)?
                 .with_child(&TextBlock::with_text("Automatically restore saved brightness")?
                     .with_vertical_alignment(VerticalAlignment::Center)?)?)?;

        let main = StackPanel::vertical()?
            .apply_if(!self.mica, |p| p
                .with_background(&self.background_brush))?
            .with_padding(10.0)?
            .with_child(&general)?;

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