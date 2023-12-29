
#[macro_export]
macro_rules! new_type {
    ($name:ident, $orig:ty) => {
        #[derive(Clone, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $name($orig);

        unsafe impl windows::core::ComInterface for $name { const IID: windows::core::GUID = <$orig as windows::core::ComInterface>::IID; }

        unsafe impl windows::core::Interface for $name { type Vtable = <$orig as windows::core::Interface>::Vtable; }

        impl windows::core::RuntimeType for $name {
            const SIGNATURE: windows::core::imp::ConstBuffer = <$orig as windows::core::RuntimeType>::SIGNATURE;
        }

        impl crate::ui::NewType for $name {
            type Inner = $orig;

            fn as_inner(&self) -> &Self::Inner {
                &self.0
            }
        }

        impl windows::core::RuntimeName for $name {
            const NAME: &'static str = <$orig as windows::core::RuntimeName>::NAME;
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <$orig as std::fmt::Debug>::fmt(&self.0, f)
            }
        }

        impl windows::core::CanTryInto<windows_ext::UI::Xaml::UIElement> for $name {
            const CAN_INTO: bool = <$orig as windows::core::CanTryInto<windows_ext::UI::Xaml::UIElement>>::CAN_INTO;
        }

        impl windows::core::CanTryInto<windows_ext::UI::Xaml::FrameworkElement> for $name {
            const CAN_INTO: bool = <$orig as windows::core::CanTryInto<windows_ext::UI::Xaml::FrameworkElement>>::CAN_INTO;
        }
    };
}

pub mod container;

pub trait NewType {
    type Inner;

    fn as_inner(&self) -> &Self::Inner;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Padding {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl From<f64> for Padding {
    fn from(value: f64) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

impl From<(f64, f64)> for Padding {
    fn from((horizontal, vertical): (f64, f64)) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }
}

impl From<(f64, f64, f64, f64)> for Padding {
    fn from((l, t, r, b): (f64, f64, f64, f64)) -> Self {
        Self {
            left: l,
            top: t,
            right: r,
            bottom: b,
        }
    }
}

impl From<Padding> for windows_ext::UI::Xaml::Thickness {
    fn from(value: Padding) -> Self {
        Self {
            Left: value.left,
            Top: value.top,
            Right: value.right,
            Bottom: value.bottom,
        }
    }
}