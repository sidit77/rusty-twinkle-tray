use log::warn;
use loole::Sender;
use windows::core::ComInterface;
use windows::UI::Color;
use windows::Win32::Foundation::COLORREF;
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMWA_CAPTION_COLOR, DWMWA_COLOR_DEFAULT, DWMWA_COLOR_NONE, DWMWA_SYSTEMBACKDROP_TYPE, DWM_SYSTEMBACKDROP_TYPE};
use windows_ext::IXamlSourceTransparency;
use windows_ext::UI::Xaml::{UIElement, Window as XamlWindow};
use windows_ext::UI::Xaml::Media::SolidColorBrush;
use crate::windowing::{Window, WindowBuilder};
use crate::{cloned, CustomEvent, Result, APP_ICON};
use crate::ui::container::StackPanel;
use crate::ui::controls::{TextBlock, ToggleSwitch};
use crate::ui::{FontWeight};
use crate::utils::extensions::FunctionalExt;

thread_local! {
    static TRANSPARENT_BACKGROUND: bool = XamlWindow::Current()
                            .and_then(|w| w.cast::<IXamlSourceTransparency>())
                            .and_then(|t| t.SetIsBackgroundTransparent(true))
                            .map_err(|e| warn!("Failed to make XAML island background transparent: {e}"))
                            .is_ok();
}

pub struct SettingsWindow {
    window: Window
}

impl SettingsWindow {
    pub fn new(sender: Sender<CustomEvent>) -> Result<Self> {
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
        unsafe {
            if mica {
                mica &= DwmSetWindowAttribute(window.hwnd, DWMWA_SYSTEMBACKDROP_TYPE, &DWMSBT_MAINWINDOW as *const _ as _, size_of::<DWM_SYSTEMBACKDROP_TYPE>() as u32)
                    .map_err(|e| warn!("Failed to set DWM system backdrop attribute: {e}"))
                    .is_ok();
            }
            if mica {
                DwmSetWindowAttribute(window.hwnd, DWMWA_CAPTION_COLOR, &DWMWA_COLOR_NONE as *const _ as _, size_of::<COLORREF>() as u32)
                    .unwrap_or_else(|e| warn!("Failed to set DWM caption color attribute: {e}"));
            }

            //DwmSetWindowAttribute(window.hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, &TRUE as *const _ as _, size_of::<BOOL>() as u32).unwrap();
        }

        window.set_content(&Self::build_gui(mica)?)?;
        window.set_visible(true);

        Ok(Self {
            window
        })
    }

    pub fn focus(&self) {
        self.window.focus();
    }

    fn build_gui(mica: bool) -> Result<impl windows::core::CanTryInto<UIElement>> {
        //let background_brush = {
        //    let brush = SolidColorBrush::CreateInstanceWithColor(Color { A: 120, R: 0, G: 0, B: 0, })?;
        //    brush
        //};

        let section_background_brush = SolidColorBrush::CreateInstanceWithColor(Color { R: 255, G: 255, B: 255, A: 255 })?;
        let border_brush = SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 30 })?;

        let general = StackPanel::vertical()?
            .apply_if(mica, |p| p
                .with_win11_style(&section_background_brush, &border_brush))?
            .with_padding(10.0)?
            .with_spacing(7.0)?
            .with_child(&TextBlock::with_text("General")?
                .with_font_size(24.0)?
                .with_font_weight(FontWeight::SemiLight)?)?
            .with_child(&StackPanel::vertical()?
                .with_child(&TextBlock::with_text("Automatically run on startup")?)?
                .with_child(&ToggleSwitch::new()?
                    .with_state(true)?
                    .with_toggled_handler(|state| Ok(println!("autostart: {state}")))?)?)?;

        let main = StackPanel::vertical()?
            .apply_if(!mica, |p| p
                .with_background(&section_background_brush))?
            .with_padding(10.0)?
            .with_child(&general)?;

        Ok(main)
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