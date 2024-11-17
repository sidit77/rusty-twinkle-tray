use windows::core::{ComInterface, TryIntoParam, HSTRING};
use windows::Foundation::{EventHandler, IReference, Point, PropertyValue};
use windows::Win32::UI::WindowsAndMessaging::WHEEL_DELTA;
use windows_ext::UI::Xaml::Controls::Primitives::{FlyoutShowOptions, RangeBaseValueChangedEventArgs, RangeBaseValueChangedEventHandler};
use windows_ext::UI::Xaml::Controls::{FlyoutPresenter, IconElement};
use windows_ext::UI::Xaml::Input::PointerEventHandler;
use windows_ext::UI::Xaml::{DependencyObject, RoutedEventHandler, UIElement};

use super::{FontWeight, Padding, TextAlignment, VerticalAlignment};
use crate::ui::style::Style;
use crate::ui::NewType;
use crate::utils::error::{WinOptionExt, TracedResultEx};
use crate::Result;

new_type!(Slider, windows_ext::UI::Xaml::Controls::Slider);

impl Slider {
    pub fn new() -> Result<Self> {
        let slider = <Self as NewType>::Inner::new()?;
        Ok(Self(slider))
    }

    pub fn with_value(self, value: f64) -> Result<Self> {
        self.0.SetValue2(value)?;
        Ok(self)
    }

    pub fn with_vertical_alignment(self, alignment: VerticalAlignment) -> Result<Self> {
        self.0.SetVerticalAlignment(alignment)?;
        Ok(self)
    }

    pub fn with_mouse_scrollable(self) -> Result<Self> {
        self.0
            .PointerWheelChanged(&PointerEventHandler::new(move |sender, args| {
                let args = args.some()?;
                args.SetHandled(true)?;
                let delta = args
                    .GetCurrentPoint(None)?
                    .Properties()?
                    .MouseWheelDelta()?
                    / WHEEL_DELTA as i32;

                let slider = sender.some()?.cast::<Self>()?;
                slider
                    .set_value(slider.get_value().to_win_result()? + delta as f64)
                    .to_win_result()?;
                Ok(())
            }))?;
        Ok(self)
    }

    pub fn with_value_changed_handler<F>(self, mut handler: F) -> Result<Self>
    where
        F: FnMut(/*Option<&::windows_core::IInspectable>, */ &RangeBaseValueChangedEventArgs) -> Result<()> + Send + 'static
    {
        self.0
            .ValueChanged(&RangeBaseValueChangedEventHandler::new(move |_sender, args| {
                handler(args.some()?).to_win_result()
            }))?;
        Ok(self)
    }

    pub fn get_value(&self) -> Result<f64> {
        Ok(self.0.Value()?)
    }

    pub fn set_value(&self, value: f64) -> Result<()> {
        Ok(self.0.SetValue2(value)?)
    }
}

new_type!(TextBlock, windows_ext::UI::Xaml::Controls::TextBlock);

impl TextBlock {
    pub fn new() -> Result<Self> {
        Ok(Self(<Self as NewType>::Inner::new()?))
    }

    pub fn with_text<T: Into<HSTRING>>(text: T) -> Result<Self> {
        let block = Self::new()?;
        block.0.SetText(&text.into())?;
        Ok(block)
    }

    pub fn with_vertical_alignment(self, alignment: VerticalAlignment) -> Result<Self> {
        self.0.SetVerticalAlignment(alignment)?;
        Ok(self)
    }

    pub fn with_text_alignment(self, alignment: TextAlignment) -> Result<Self> {
        self.0.SetTextAlignment(alignment)?;
        Ok(self)
    }

    pub fn with_padding<P: Into<Padding>>(self, padding: P) -> Result<Self> {
        self.0.SetPadding(padding.into().into())?;
        Ok(self)
    }

    pub fn with_font_size(self, size: f64) -> Result<Self> {
        self.0.SetFontSize(size)?;
        Ok(self)
    }

    pub fn with_font_weight<W: Into<FontWeight>>(self, weight: W) -> Result<Self> {
        self.0.SetFontWeight(weight.into().into())?;
        Ok(self)
    }

    pub fn set_text<T: Into<HSTRING>>(&self, text: T) -> Result<()> {
        Ok(self.0.SetText(&text.into())?)
    }
}

new_type!(FontIcon, windows_ext::UI::Xaml::Controls::FontIcon);
impl windows::core::CanTryInto<IconElement> for FontIcon {
    const CAN_INTO: bool = <windows_ext::UI::Xaml::Controls::FontIcon as windows::core::CanTryInto<IconElement>>::CAN_INTO;
}

impl FontIcon {
    pub fn new(icon: char) -> Result<Self> {
        let mut buffer = [0u16; 2];
        let font_icon = <Self as NewType>::Inner::new()?;
        font_icon.SetGlyph(&HSTRING::from_wide(icon.encode_utf16(&mut buffer))?)?;
        Ok(Self(font_icon))
    }

    //pub fn with_font_size(self, size: f64) -> Result<Self> {
    //    self.0.SetFontSize(size)?;
    //    Ok(self)
    //}

    pub fn with_font_weight<W: Into<FontWeight>>(self, weight: W) -> Result<Self> {
        self.0.SetFontWeight(weight.into().into())?;
        Ok(self)
    }

    pub fn with_vertical_alignment(self, alignment: VerticalAlignment) -> Result<Self> {
        self.0.SetVerticalAlignment(alignment)?;
        Ok(self)
    }
}

new_type!(AppBarButton, windows_ext::UI::Xaml::Controls::AppBarButton);

impl AppBarButton {
    pub fn new() -> Result<Self> {
        let button = <Self as NewType>::Inner::new()?;
        Ok(Self(button))
    }

    pub fn with_label<T: Into<HSTRING>>(self, text: T) -> Result<Self> {
        self.0.SetLabel(&text.into())?;
        Ok(self)
    }

    pub fn with_icon<T: TryIntoParam<IconElement>>(self, icon: T) -> Result<Self> {
        self.0.SetIcon(icon)?;
        Ok(self)
    }

    pub fn with_enabled(self, enabled: bool) -> Result<Self> {
        self.0.SetIsEnabled(enabled)?;
        Ok(self)
    }

    pub fn with_click_handler<F>(self, mut handler: F) -> Result<Self>
    where
        F: FnMut(/*Option<&::windows_core::IInspectable>,  &RoutedEventArgs*/) -> Result<()> + Send + 'static
    {
        self.0
            .Click(&RoutedEventHandler::new(move |_sender, _args| handler().to_win_result()))?;
        Ok(self)
    }
}

new_type!(ToggleSwitch, windows_ext::UI::Xaml::Controls::ToggleSwitch);

impl ToggleSwitch {
    pub fn new() -> Result<Self> {
        let switch = <Self as NewType>::Inner::new()?;
        Ok(Self(switch))
    }

    pub fn with_state(self, on: bool) -> Result<Self> {
        self.0.SetIsOn(on)?;
        Ok(self)
    }

    pub fn with_toggled_handler<F>(self, mut handler: F) -> Result<Self>
    where
        F: FnMut(bool) -> Result<()> + Send + 'static
    {
        self.0.Toggled(&RoutedEventHandler::new(move |sender, _| {
            let state = sender.some()?.cast::<Self>()?.0.IsOn()?;
            handler(state).to_win_result()
        }))?;
        Ok(self)
    }
}

pub use windows_ext::UI::Xaml::Controls::Primitives::FlyoutPlacementMode;
new_type!(Flyout, windows_ext::UI::Xaml::Controls::Flyout, no_ui);

impl Flyout {
    pub fn new<T: TryIntoParam<UIElement>>(content: T) -> Result<Self> {
        let flyout = <Self as NewType>::Inner::new()?;
        flyout.SetContent(content)?;
        flyout.SetShouldConstrainToRootBounds(false)?;
        flyout.SetAreOpenCloseAnimationsEnabled(true)?;
        Ok(Self(flyout))
    }

    pub fn with_closed_handler<F>(self, mut handler: F) -> Result<Self>
    where
        F: FnMut() -> Result<()> + Send + 'static
    {
        self.0
            .Closed(&EventHandler::new(move |_, _| handler().to_win_result()))?;
        Ok(self)
    }

    pub fn with_style<F: FnOnce(Style) -> Result<Style>>(self, f: F) -> Result<Self> {
        let style = f(Style::new::<FlyoutPresenter>()?)?;
        self.0.SetFlyoutPresenterStyle(style.as_inner())?;
        Ok(self)
    }

    pub fn show_at<E>(&self, base: E, x: f32, y: f32, mode: FlyoutPlacementMode) -> Result<()>
    where
        E: TryIntoParam<DependencyObject>
    {
        let options = FlyoutShowOptions::new()?;
        let pt = Point { X: x, Y: y };
        options.SetPosition(&PropertyValue::CreatePoint(pt)?.cast::<IReference<Point>>()?)?;
        options.SetPlacement(mode)?;

        self.0.ShowAt2(base, &options)?;
        Ok(())
    }

    pub fn is_open(&self) -> Result<bool> {
        Ok(self.0.IsOpen()?)
    }

    pub fn close(&self) -> Result<()> {
        Ok(self.0.Hide()?)
    }
}
