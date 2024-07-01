use std::collections::BTreeMap;
use windows::UI::Color;
use windows_ext::UI::Xaml::Media::{SolidColorBrush};
use crate::{cloned, hformat};
use crate::backend::MonitorControllerProxy;
use crate::monitors::MonitorPath;
use crate::ui::container::{Grid, GridSize, StackPanel};
use crate::ui::controls::{Slider, TextBlock, FontIcon};
use crate::ui::{FontWeight, TextAlignment, VerticalAlignment};
use crate::utils::error::{Result};

pub struct XamlGui {
    monitor_panel: StackPanel,
    main_grid: Grid,
    monitor_controls: BTreeMap<MonitorPath, MonitorEntry>,
}

impl XamlGui {
    pub fn new() -> Result<Self> {

        //let icon_font = FontFamily::new(&HSTRING::from("Segoe Fluent Icons"))?;

        let stack_panel = StackPanel::vertical()?
            .with_spacing(20.0)?
            .with_padding((20.0, 14.0))?;
            //.with_children(monitor_controls.iter().map(MonitorEntry::ui))?;

        let settings = FontIcon::new('\u{E713}')? // Modern Windows 11 Settings icon
            .with_vertical_alignment(VerticalAlignment::Center)?
            .with_font_weight(FontWeight::Medium)?;

        // Create a new stack panel for the bottom bar
        let bottom_bar = Grid::new()?
            .with_padding(20.0)?
            .with_column_widths([GridSize::Fraction(1.0), GridSize::Auto])?
            .with_background(&SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 70})?)?
            .with_child(&TextBlock::new()?
                .with_text("Adjust Brightness")?
                .with_font_size(15.0)?
                .with_vertical_alignment(VerticalAlignment::Center)?
                .with_padding((20.0, 0.0, 0.0, 0.0))?, 0, 0)?
            .with_child(&StackPanel::horizontal()?
                .with_child(&FontIcon::new('\u{E890}')? // New Hide Icon
                    .with_vertical_alignment(VerticalAlignment::Center)?
                    .with_font_weight(FontWeight::Medium)?)?
                .with_child(&settings)?
                .with_spacing(8.0)?, 0, 1)?;

        let main_grid = Grid::new()? // Create a new grid to hold the main stackpanel and the bottom bar
            .with_row_heights([GridSize::Auto, GridSize::Fraction(1.0), GridSize::Auto])?
            // Add the main stack panel and the bottom bar to the main grid
            .with_child(&stack_panel, 0, 0)?
            .with_child(&bottom_bar, 2, 0)?;

        Ok(Self {
            monitor_panel: stack_panel,
            main_grid,
            monitor_controls: BTreeMap::new()
        })
    }

    //pub fn get_required_height(&self) -> Result<u32> {
    //    Ok((self.monitor_panel.get_actual_height()? + self.bottom_bar.get_actual_height()?) as u32)
    //}

    pub fn register_monitor(&mut self, name: String, path: MonitorPath, proxy: MonitorControllerProxy) -> Result<()> {
        assert!(!self.monitor_controls.contains_key(&path));
        let monitor = MonitorEntry::create(name, path.clone(), proxy)?;
        self.monitor_panel.add_child(monitor.ui())?;
        self.monitor_controls.insert(path, monitor);
        Ok(())
    }

    pub fn update_brightness(&self, path: MonitorPath, new_brightness: u32) -> Result<()> {
        match self.monitor_controls.get(&path) {
            None => log::warn!("Monitor is not registered: {:?}", path),
            Some(entry) => entry.set_brightness(new_brightness)?
        }
        Ok(())
    }

    pub fn ui(&self) -> &Grid {
        &self.main_grid
    }

}

struct MonitorEntry {
    ui: StackPanel,
    slider: Slider
}

impl MonitorEntry {

    pub fn create(name: String, path: MonitorPath, proxy: MonitorControllerProxy) -> Result<Self> {
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
            .with_child(&StackPanel::horizontal()?
                .with_spacing(8.0)?
                .with_child(&FontIcon::new('\u{E7f4}')?
                    .with_font_weight(FontWeight::Medium)?)?
                .with_child(&TextBlock::new()?
                    .with_text(name)?
                    .with_font_size(20.0)?)?)?
            .with_child(&Grid::new()?
                .with_column_widths([GridSize::Fraction(1.0), GridSize::Pixel(40.0)])?
                .with_column_spacing(8.0)?
                .with_child(&slider, 0, 0)?
                .with_child(&label, 0, 1)?)?;

        Ok(Self {
            ui,
            slider,
        })
    }

    pub fn set_brightness(&self, value: u32) -> Result<()> {
        self.slider.set_value(value as f64)
    }

    pub fn ui(&self) -> &StackPanel {
        &self.ui
    }

}