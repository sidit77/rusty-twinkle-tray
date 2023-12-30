use windows::core::ComInterface;
use windows::Win32::UI::WindowsAndMessaging::WHEEL_DELTA;
use windows_ext::UI::Xaml::Controls::Primitives::{RangeBaseValueChangedEventArgs, RangeBaseValueChangedEventHandler};
use windows_ext::UI::Xaml::Input::PointerEventHandler;
use super::VerticalAlignment;
use crate::Result;
use crate::ui::NewType;
use crate::utils::error::{OptionExt, ResultEx};

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
        self.0.PointerWheelChanged(&PointerEventHandler::new(move |sender, args| {
            let args = args.some()?;
            args.SetHandled(true)?;
            let delta = args
                .GetCurrentPoint(None)?
                .Properties()?
                .MouseWheelDelta()?
                / WHEEL_DELTA as i32;

            let slider = sender.some()?.cast::<Self>()?;
            slider.set_value(slider.get_value().to_win_result()? + delta as f64).to_win_result()?;
            Ok(())
        }))?;
        Ok(self)
    }

    pub fn with_value_changed_handler<F>(self, mut handler: F) -> Result<Self>
        where F: FnMut( /*Option<&::windows_core::IInspectable>, */&RangeBaseValueChangedEventArgs) -> Result<()> + Send + 'static
    {
        self.0.ValueChanged(&RangeBaseValueChangedEventHandler::new(move | _sender, args| {
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