use log::{debug, warn};
use loole::Sender;
use windows::core::{h, IInspectable};
use windows::Foundation::Size;
use windows::UI::Color;
use windows_ext::UI::Xaml::Controls::Control;
use windows_ext::UI::Xaml::Controls::Primitives::FlyoutPlacementMode;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush, SolidColorBrush};
use windows_ext::UI::Xaml::{TextAlignment, VerticalAlignment};

use crate::backend::MonitorControllerProxy;
use crate::monitors::MonitorPath;
use crate::theme::ColorSet;
use crate::ui::container::{Grid, GridSize, StackPanel};
use crate::ui::controls::{AppBarButton, Flyout, FontIcon, Slider, TextBlock};
use crate::ui::FontWeight;
use crate::utils::extensions::ChannelExt;
use crate::utils::ordered_map::{OrderedMap, SortKeyExtract};
use crate::windowing::{Window, WindowBuilder, WindowLevel};
use crate::{cloned, hformat, log_assert, CustomEvent, Result};

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
        self.window
            .set_window_pos(Some(WindowLevel::AlwaysOnTop), Some((0, 0)), None, Some(true));
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

    monitor_panel: StackPanel,
    monitor_controls: OrderedMap<MonitorPath, MonitorEntry>
}

impl BrightnessFlyout {
    const WIDTH: f64 = 400.0;

    pub fn new(sender: Sender<CustomEvent>, colors: &ColorSet) -> Result<Self> {
        let settings = AppBarButton::new()?
            .with_icon(&FontIcon::new('\u{E713}')?)?
            .with_label("Settings")?
            //.with_enabled(false)?;
            .with_click_handler(cloned!([sender] move|| {
                sender
                    .send(CustomEvent::OpenSettings)
                    .unwrap_or_else(|_| log::warn!("Failed to send settings event"));
                Ok(())
            }))?;

        let refresh = AppBarButton::new()?
            .with_icon(&FontIcon::new('\u{E72C}')?)?
            .with_label("Refresh")?
            .with_click_handler(cloned!([sender] move || {
                sender
                    .send(CustomEvent::Refresh)
                    .unwrap_or_else(|_| log::warn!("Failed to send refresh event"));
                Ok(())
            }))?;

        // Create a new stack panel for the bottom bar
        let bottom_bar = Grid::new()?
            //.with_padding(20.0)?
            .with_column_widths([GridSize::Fraction(1.0), GridSize::Auto])?
            .with_background(&SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 70 })?)?
            .with_child(
                &TextBlock::with_text("Adjust Brightness")?
                    .with_font_size(15.0)?
                    .with_vertical_alignment(VerticalAlignment::Center)?
                    .with_padding((20.0, 0.0, 0.0, 0.0))?,
                0,
                0
            )?
            .with_child(
                &StackPanel::horizontal()?
                    .with_child(&refresh)?
                    .with_child(&settings)?,
                0,
                1
            )?;

        let monitor_panel = StackPanel::vertical()?
            .with_spacing(20.0)?
            .with_padding((20.0, 14.0))?;

        let container = StackPanel::vertical()?
            .with_theme(colors.theme)?
            .with_width(Self::WIDTH)?
            .with_child(
                &Grid::new()?
                    .with_row_heights([GridSize::Auto, GridSize::Fraction(1.0), GridSize::Auto])?
                    .with_child(&monitor_panel, 0, 0)?
                    .with_child(&bottom_bar, 2, 0)?
            )?;

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

        Ok(Self {
            flyout,
            background,
            container,
            monitor_panel,
            monitor_controls: OrderedMap::new()
        })
    }

    pub fn update_theme(&self, colors: &ColorSet) -> Result<()> {
        self.container.set_theme(colors.theme)?;
        self.background.SetFallbackColor(colors.fallback)?;
        self.background.SetTintColor(colors.tint)?;
        self.background.SetTintOpacity(colors.opacity)?;
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.flyout
            .is_open()
            .map_err(|e| warn!("Failed to check if flyout is open: {}", e))
            .unwrap_or(false)
    }

    pub fn close(&self) {
        self.flyout
            .close()
            .unwrap_or_else(|e| warn!("Failed to close flyout: {}", e));
    }

    pub fn show(&self, proxy_window: &ProxyWindow, x: f32, y: f32) {
        self.flyout
            .show_at(&proxy_window.content, x, y, FlyoutPlacementMode::LeftEdgeAlignedBottom)
            .unwrap_or_else(|e| warn!("Failed to show flyout: {}", e));
    }

    pub fn size(&self) -> Size {
        self.container.measure().unwrap_or_else(|e| {
            debug!("Failed to get measured size of flyout: {}", e);
            // Make a guess
            Size {
                Width: Self::WIDTH as f32,
                Height: 62.0 + 86.0 * self.monitor_controls.len() as f32
            }
        })
    }

    fn repopulate_monitor_list(&self) -> Result<()> {
        self.monitor_panel.clear_children()?;
        for monitor in self.monitor_controls.values() {
            self.monitor_panel.add_child(monitor.ui())?;
        }
        Ok(())
    }

    pub fn register_monitor(&mut self, name: String, path: MonitorPath, position: i32, proxy: MonitorControllerProxy) -> Result<()> {
        log_assert!(self
            .monitor_controls
            .insert(path.clone(), MonitorEntry::create(name, path, position, proxy)?)
            .is_none());
        self.repopulate_monitor_list()?;
        Ok(())
    }

    pub fn unregister_monitor(&mut self, path: &MonitorPath) -> Result<()> {
        log_assert!(self.monitor_controls.remove(path).is_some());
        self.repopulate_monitor_list()?;
        Ok(())
    }

    pub fn clear_monitors(&mut self) -> Result<()> {
        self.monitor_controls.clear();
        self.repopulate_monitor_list()?;
        Ok(())
    }

    pub fn update_brightness(&self, path: MonitorPath, new_brightness: u32) -> Result<()> {
        match self.monitor_controls.get(&path) {
            None => warn!("Monitor is not registered: {:?}", path),
            Some(entry) => entry.set_brightness(new_brightness)?
        }
        Ok(())
    }
}

struct MonitorEntry {
    position: i32,
    ui: StackPanel,
    slider: Slider
}

impl MonitorEntry {
    pub fn create(name: String, path: MonitorPath, position: i32, proxy: MonitorControllerProxy) -> Result<Self> {
        let label = TextBlock::new()?
            .with_vertical_alignment(VerticalAlignment::Center)?
            .with_text_alignment(TextAlignment::Center)?
            .with_font_size(20.0)?
            .with_font_weight(FontWeight::Medium)?;

        let slider = Slider::new()?
            .with_vertical_alignment(VerticalAlignment::Center)?
            .with_value(0.0)?
            .with_mouse_scrollable()?
            .with_value_changed_handler(cloned!([label] move |args| {
                let new = args.NewValue()? as u32;
                label.set_text(hformat!("{}", new))?;
                proxy.set_brightness(path.clone(), new);
                Ok(())
            }))?;

        label.set_text(hformat!("{}", slider.get_value()?))?;

        let ui = StackPanel::vertical()?
            .with_spacing(4.0)?
            .with_child(
                &StackPanel::horizontal()?
                    .with_spacing(8.0)?
                    .with_child(&FontIcon::new('\u{E7f4}')?.with_font_weight(FontWeight::Medium)?)?
                    .with_child(&TextBlock::with_text(&name)?.with_font_size(20.0)?)?
            )?
            .with_child(
                &Grid::new()?
                    .with_column_widths([GridSize::Fraction(1.0), GridSize::Pixel(40.0)])?
                    .with_column_spacing(8.0)?
                    .with_child(&slider, 0, 0)?
                    .with_child(&label, 0, 1)?
            )?;

        Ok(Self { position, ui, slider })
    }

    pub fn set_brightness(&self, value: u32) -> Result<()> {
        self.slider.set_value(value as f64)
    }

    pub fn ui(&self) -> &StackPanel {
        &self.ui
    }
}

impl SortKeyExtract for MonitorEntry {
    type Key = i32;

    fn sort_key(&self) -> i32 {
        self.position
    }
}
