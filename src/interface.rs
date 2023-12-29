use windows::core::{ComInterface, HSTRING};
use windows::UI::Color;
use windows::UI::Text::FontWeight;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_SHOWWINDOW, WHEEL_DELTA};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use windows_ext::UI::Xaml::Controls::{FontIcon, Slider, TextBlock};
use windows_ext::UI::Xaml::Hosting::DesktopWindowXamlSource;
use windows_ext::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush, SolidColorBrush};
use windows_ext::UI::Xaml::{ElementTheme, TextAlignment, Thickness, VerticalAlignment};
use windows_ext::UI::Xaml::Controls::Primitives::RangeBaseValueChangedEventHandler;
use windows_ext::UI::Xaml::Input::PointerEventHandler;
use windows_ext::Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative;
use crate::ui::container::{Grid, GridSize, StackPanel};
use crate::utils::error::{OptionExt, Result};
use crate::utils::WindowExt;

pub struct XamlGui {
    hwnd: HWND,
    bottom_bar: Grid,
    monitor_controls: StackPanel,
    _desktop_source: DesktopWindowXamlSource
}

impl XamlGui {
    pub fn new(parent: &Window) -> Result<Self> {
        let desktop_source = DesktopWindowXamlSource::new()?;
        let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;
        unsafe {
            interop.AttachToWindow(parent.hwnd())?;
        }
        let island = unsafe { interop.WindowHandle() }?;

        //let icon_font = FontFamily::new(&HSTRING::from("Segoe Fluent Icons"))?;

        let stack_panel = StackPanel::vertical()?
            .with_spacing(8.0)?
            .with_padding((20.0, 14.0))?
            .with_child(&Self::create_monitor_entry()?)?;

        // Create a new stack panel for the bottom bar

        let bottom_bar = Grid::new()?
            .with_padding(20.0)?
            .with_column_widths([GridSize::Fraction(1.0), GridSize::Auto])?
            .with_background(&SolidColorBrush::CreateInstanceWithColor(Color { R: 0, G: 0, B: 0, A: 70})?)?
            .with_child(&{
                let text_block = TextBlock::new()?;
                text_block.SetText(&HSTRING::from("Adjust Brightness"))?;
                text_block.SetVerticalAlignment(VerticalAlignment::Center)?;
                text_block.SetPadding(Thickness { Left: 20.0, Top: 0.0, Right: 0.0, Bottom: 0.0})?;
                text_block.SetFontSize(15.0)?;
                text_block
            }, 0, 0)?
            .with_child(&StackPanel::horizontal()?
                .with_child(&{
                    let icon = FontIcon::new()?;
                    icon.SetGlyph(&HSTRING::from("\u{E713}"))?; // Modern Windows 11 Settings icon
                    icon.SetFontWeight(FontWeight { Weight: 500 })?;
                    icon.SetVerticalAlignment(VerticalAlignment::Center)?;
                    icon
                })?, 0, 1)?;

        let main_grid = Grid::new()? // Create a new grid to hold the main stackpanel and the bottom bar
            .with_row_heights([GridSize::Auto, GridSize::Fraction(1.0), GridSize::Auto])?
            .with_background(&{
                let brush = AcrylicBrush::new()?;
                let color = Color { R: 70, G: 70, B: 70, A: 255 };
                brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;
                brush.SetFallbackColor(color)?;
                brush.SetTintColor(color)?;
                brush.SetTintOpacity(0.7)?;
                brush
            })?
            .with_theme(ElementTheme::Dark)?
            // Add the main stack panel and the bottom bar to the main grid
            .with_child(&stack_panel, 0, 0)?
            .with_child(&bottom_bar, 2, 0)?;

        desktop_source.SetContent(&main_grid)?;
        Ok(Self {
            hwnd: island,
            bottom_bar,
            _desktop_source: desktop_source,
            monitor_controls: stack_panel,
        })
    }

    fn create_monitor_entry() -> Result<StackPanel> {
        let slider = {
            let slider = Slider::new()?;
            slider.SetVerticalAlignment(VerticalAlignment::Center)?;
            slider
        };
        let text_box = {
            let text_box = TextBlock::new()?;
            text_box.SetVerticalAlignment(VerticalAlignment::Center)?;
            text_box.SetTextAlignment(TextAlignment::Center)?;
            text_box.SetFontSize(20.0)?;
            text_box.SetFontWeight(FontWeight { Weight: 400 })?;
            text_box.SetPadding(Thickness { Left: 10.0, Top: 0.0, Right: 10.0, Bottom: 0.0 })?;
            text_box.SetText(&HSTRING::from(&format!("{}", slider.Value()?)))?;
            text_box
        };
        slider.ValueChanged(&RangeBaseValueChangedEventHandler::new({
            let text_box = text_box.clone();
            move |_, event| {
                text_box.SetText(&HSTRING::from(format!("{}", event.some()?.NewValue()?)))?;
                Ok(())
            }
        }))?;
        slider.PointerWheelChanged(&PointerEventHandler::new(move |sender, args| {
            let args = args.some()?;
            args.SetHandled(true)?;
            let delta = args
                .GetCurrentPoint(None)?
                .Properties()?
                .MouseWheelDelta()?
                / WHEEL_DELTA as i32;

            let slider = sender.some()?.cast::<Slider>()?;
            slider.SetValue2(slider.Value()? + delta as f64)?;
            Ok(())
        }))?;

        Ok(StackPanel::vertical()?
            .with_spacing(4.0)?
            .with_child(&StackPanel::horizontal()?
                .with_spacing(8.0)?
                .with_child(&{
                    let icon = FontIcon::new()?;
                    //icon.SetFontFamily(&icon_font)?;
                    icon.SetGlyph(&HSTRING::from("\u{E7f4}"))?;
                    icon.SetFontWeight(FontWeight { Weight: 500 })?;
                    icon
                })?
                .with_child(&{
                    let text_block = TextBlock::new()?;
                    text_block.SetText(&HSTRING::from("Monitor 1"))?;
                    text_block.SetFontSize(20.0)?;
                    text_block
                })?)?
            .with_child(&Grid::new()?
                .with_column_widths([GridSize::Fraction(1.0), GridSize::Auto])?
                .with_column_spacing(8.0)?
                .with_child(&slider, 0, 0)?
                .with_child(&text_box, 0, 1)?)?)
    }

    pub fn get_required_height(&self) -> Result<u32> {
        Ok((self.monitor_controls.get_actual_height()? + self.bottom_bar.get_actual_height()?) as u32)
    }

    pub fn resize(&self, new_size: PhysicalSize<u32>) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND::default(),
                0,
                0,
                new_size.width as _,
                new_size.height as _,
                SWP_SHOWWINDOW
            )?;
        }
        Ok(())
    }

    pub fn focus(&self) {
        unsafe {
            SetFocus(self.hwnd);
        }
    }

}