use log::{debug, warn};
use loole::Sender;
use windows::core::{h, IInspectable};
use windows::Foundation::Size;
use windows_ext::UI::Xaml::Controls::Control;
use windows_ext::UI::Xaml::Controls::Primitives::FlyoutPlacementMode;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush};
use crate::interface::XamlGui;
use crate::ui::container::StackPanel;
use crate::ui::controls::{Flyout, TextBlock};
use crate::windowing::{Window, WindowBuilder, WindowLevel};
use crate::{cloned, CustomEvent, Result};
use crate::backend::MonitorControllerProxy;
use crate::monitors::MonitorPath;
use crate::theme::ColorSet;
use crate::utils::extensions::ChannelExt;

pub struct ProxyWindow {
    window: Window,
    content: TextBlock
}

impl ProxyWindow {

    pub fn new() -> Result<Self> {
        let window = WindowBuilder::default()
            .with_position(0, 0)
            .with_size(10, 10)
            .with_hidden(true)
            .build()?;
        let content = TextBlock::with_text("This should be invisible!")?;
        window.set_content(&content)?;
        Ok(Self { window, content })
    }

    pub fn activate(&self) -> bool {
        self.window.set_window_pos(Some(WindowLevel::AlwaysOnTop), Some((0, 0)), None, Some(true));
        self.window.focus();
        self.window.set_foreground()
    }

    pub fn deactivate(&self) {
        self.window.set_window_pos(None, None, None, Some(false))
    }

    pub fn dpi(&self) -> f32 {
        self.window.dpi()
    }

}

pub struct BrightnessFlyout {
    flyout: Flyout,
    background: AcrylicBrush,
    container: StackPanel,

    gui: XamlGui
}

impl BrightnessFlyout {

    const WIDTH: f64 = 400.0;


    pub fn new(sender: Sender<CustomEvent>, colors: &ColorSet) -> Result<Self> {
        let gui = XamlGui::new(sender.clone())?;

        let container = StackPanel::vertical()?
            .with_theme(colors.theme)?
            .with_width(Self::WIDTH)?
            .with_child(gui.ui())?;

        let background = {
            let brush = AcrylicBrush::new()?;
            brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
            brush.SetFallbackColor(colors.fallback)?;
            brush.SetTintColor(colors.tint)?;
            brush.SetTintOpacity(colors.opacity)?;
            brush
        };

        let flyout = Flyout::new(&container)?
            .with_style(|style| {
                style
                    .with_setter(&Control::BackgroundProperty()?, &background)?
                    .with_setter(&Control::CornerRadiusProperty()?, &IInspectable::try_from(h!("10.0"))?)?
                    .with_setter(&Control::PaddingProperty()?, &IInspectable::try_from(h!("0.0"))?)
            })?
            .with_closed_handler(cloned!([sender] move || {
                sender.filter_send_ignore(Some(CustomEvent::FocusLost));
                Ok(())
            }))?;

        Ok(Self { flyout, background, container, gui })
    }

    pub fn update_theme(&self, colors: &ColorSet) -> Result<()> {
        self.container.set_theme(colors.theme)?;
        self.background.SetFallbackColor(colors.fallback)?;
        self.background.SetTintColor(colors.tint)?;
        self.background.SetTintOpacity(colors.opacity)?;
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.flyout.is_open()
            .map_err(|e| warn!("Failed to check if flyout is open: {}", e))
            .unwrap_or(false)
    }

    pub fn close(&self) {
        self.flyout.close()
            .unwrap_or_else(|e| warn!("Failed to close flyout: {}", e));
    }

    pub fn show(&self, proxy_window: &ProxyWindow, x: f32, y: f32) {
        self.flyout.show_at(
            &proxy_window.content,
            x, y,
            FlyoutPlacementMode::LeftEdgeAlignedBottom
        ).unwrap_or_else(|e| warn!("Failed to show flyout: {}", e));
    }

    pub fn size(&self) -> Size {
        self.container.measure()
            .unwrap_or_else(|e| {
                debug!("Failed to get measured size of flyout: {}", e);
                // Make a guess
                Size { Width: Self::WIDTH as f32, Height: 62.0 + 86.0 * self.gui.number_of_monitors() as f32 }
            })
    }



    pub fn register_monitor(&mut self, name: String, path: MonitorPath, proxy: MonitorControllerProxy) {
        self.gui.register_monitor(name, path, proxy)
            .unwrap_or_else(|e| warn!("Failed to register monitor: {}", e));
    }

    pub fn unregister_monitor(&mut self, path: &MonitorPath) {
        self.gui.unregister_monitor(path)
            .unwrap_or_else(|e| warn!("Failed to unregister monitor: {}", e));
    }

    pub fn clear_monitors(&mut self) {
        self.gui.clear_monitors()
            .unwrap_or_else(|e| warn!("Failed to clear monitors: {}", e));
    }

    pub fn update_brightness(&self, path: MonitorPath, new_brightness: u32) {
        self.gui.update_brightness(path, new_brightness)
            .unwrap_or_else(|e| warn!("Failed to update slider: {}", e));
    }

}