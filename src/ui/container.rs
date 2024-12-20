use windows::core::TryIntoParam;
use windows::Foundation::Size;
use windows_ext::UI::Xaml::Controls::{ColumnDefinition, Orientation, RowDefinition};
use windows_ext::UI::Xaml::Media::Brush;
use windows_ext::UI::Xaml::{CornerRadius, FrameworkElement, GridUnitType, UIElement};

use crate::ui::{ElementTheme, NewType, Padding};
use crate::utils::error::Result;

new_type!(StackPanel, windows_ext::UI::Xaml::Controls::StackPanel);
impl StackPanel {
    pub fn vertical() -> Result<Self> {
        let inner = <Self as NewType>::Inner::new()?;
        inner.SetOrientation(Orientation::Vertical)?;
        Ok(Self(inner))
    }

    pub fn horizontal() -> Result<Self> {
        let inner = <Self as NewType>::Inner::new()?;
        inner.SetOrientation(Orientation::Horizontal)?;
        Ok(Self(inner))
    }

    pub fn with_spacing(self, spacing: f64) -> Result<Self> {
        self.0.SetSpacing(spacing)?;
        Ok(self)
    }

    pub fn with_padding<P: Into<Padding>>(self, padding: P) -> Result<Self> {
        self.0.SetPadding(padding.into().into())?;
        Ok(self)
    }

    pub fn with_border_thickness<P: Into<Padding>>(self, thickness: P) -> Result<Self> {
        self.0.SetBorderThickness(thickness.into().into())?;
        Ok(self)
    }

    pub fn with_corner_radius(self, radius: f64) -> Result<Self> {
        self.0.SetCornerRadius(CornerRadius {
            TopLeft: radius,
            TopRight: radius,
            BottomRight: radius,
            BottomLeft: radius
        })?;
        Ok(self)
    }

    pub fn add_child<T: TryIntoParam<UIElement>>(&self, child: T) -> Result<()> {
        self.0.Children()?.Append(child)?;
        Ok(())
    }

    pub fn clear_children(&self) -> Result<()> {
        self.0.Children()?.Clear()?;
        Ok(())
    }

    pub fn with_child<T: TryIntoParam<UIElement>>(self, child: T) -> Result<Self> {
        self.0.Children()?.Append(child)?;
        Ok(self)
    }

    pub fn with_theme(self, theme: ElementTheme) -> Result<Self> {
        self.0.SetRequestedTheme(theme)?;
        Ok(self)
    }

    pub fn with_width(self, width: f64) -> Result<Self> {
        self.0.SetWidth(width)?;
        Ok(self)
    }

    pub fn set_theme(&self, theme: ElementTheme) -> Result<()> {
        self.0.SetRequestedTheme(theme)?;
        Ok(())
    }

    pub fn theme(&self) -> Result<ElementTheme> {
        Ok(self.0.ActualTheme()?)
    }

    pub fn with_background<B: TryIntoParam<Brush>>(self, brush: B) -> Result<Self> {
        self.0.SetBackground(brush)?;
        Ok(self)
    }

    pub fn with_border_brush<B: TryIntoParam<Brush>>(self, brush: B) -> Result<Self> {
        self.0.SetBorderBrush(brush)?;
        Ok(self)
    }

    //pub fn with_children<T: TryIntoParam<UIElement>, I: IntoIterator<Item=T>>(mut self, children: I) -> Result<Self> {
    //    for child in children {
    //        self.add_child(child)?;
    //    }
    //    Ok(self)
    //}

    pub fn get_actual_height(&self) -> Result<f64> {
        Ok(self.0.ActualHeight()?)
    }

    pub fn get_actual_width(&self) -> Result<f64> {
        Ok(self.0.ActualWidth()?)
    }

    pub fn measure(&self) -> Result<Size> {
        self.0.Measure(Size {
            Width: f32::INFINITY,
            Height: f32::INFINITY
        })?;
        let s = self.0.DesiredSize()?;
        Ok(s)
    }
}

pub enum GridSize {
    Auto,
    Pixel(f64),
    Fraction(f64)
}

impl From<GridSize> for windows_ext::UI::Xaml::GridLength {
    fn from(value: GridSize) -> Self {
        Self {
            Value: match value {
                GridSize::Auto => 1.0,
                GridSize::Pixel(v) => v,
                GridSize::Fraction(v) => v
            },
            GridUnitType: match value {
                GridSize::Auto => GridUnitType::Auto,
                GridSize::Pixel(_) => GridUnitType::Pixel,
                GridSize::Fraction(_) => GridUnitType::Star
            }
        }
    }
}

new_type!(Grid, windows_ext::UI::Xaml::Controls::Grid);

impl Grid {
    pub fn new() -> Result<Self> {
        Ok(Self(<Self as NewType>::Inner::new()?))
    }

    pub fn with_column_widths<I: IntoIterator<Item = GridSize>>(self, sizes: I) -> Result<Self> {
        let definitions = self.0.ColumnDefinitions()?;
        definitions.Clear()?;
        for size in sizes {
            definitions.Append(&{
                let def = ColumnDefinition::new()?;
                def.SetWidth(size.into())?;
                def
            })?;
        }
        Ok(self)
    }

    pub fn with_row_heights<I: IntoIterator<Item = GridSize>>(self, sizes: I) -> Result<Self> {
        let definitions = self.0.RowDefinitions()?;
        definitions.Clear()?;
        for size in sizes {
            definitions.Append(&{
                let def = RowDefinition::new()?;
                def.SetHeight(size.into())?;
                def
            })?;
        }
        Ok(self)
    }

    pub fn with_column_spacing(self, spacing: f64) -> Result<Self> {
        self.0.SetColumnSpacing(spacing)?;
        Ok(self)
    }

    pub fn with_padding<P: Into<Padding>>(self, padding: P) -> Result<Self> {
        self.0.SetPadding(padding.into().into())?;
        Ok(self)
    }

    pub fn with_child<T: TryIntoParam<UIElement> + TryIntoParam<FrameworkElement> + Copy>(self, child: T, row: i32, column: i32) -> Result<Self> {
        <Self as NewType>::Inner::SetColumn(child, column)?;
        <Self as NewType>::Inner::SetRow(child, row)?;
        self.0.Children()?.Append(child)?;
        Ok(self)
    }

    pub fn with_background<B: TryIntoParam<Brush>>(self, brush: B) -> Result<Self> {
        self.0.SetBackground(brush)?;
        Ok(self)
    }

    pub fn with_theme(self, theme: ElementTheme) -> Result<Self> {
        self.0.SetRequestedTheme(theme)?;
        Ok(self)
    }

    pub fn set_theme(&self, theme: ElementTheme) -> Result<()> {
        self.0.SetRequestedTheme(theme)?;
        Ok(())
    }

    pub fn get_actual_height(&self) -> Result<f64> {
        Ok(self.0.ActualHeight()?)
    }
    pub fn get_actual_width(&self) -> Result<f64> {
        Ok(self.0.ActualWidth()?)
    }
}
