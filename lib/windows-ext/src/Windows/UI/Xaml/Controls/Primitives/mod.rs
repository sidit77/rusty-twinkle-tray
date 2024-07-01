#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IAppBarButtonTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarButtonTemplateSettings {
    type Vtable = IAppBarButtonTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBarButtonTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xcbc9b39d_0c95_4951_bff2_13963691c366,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarButtonTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IAppBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarTemplateSettings {
    type Vtable = IAppBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBarTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbcc2a863_eb35_423c_8389_d7827be3bf67,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ClipRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub CompactVerticalDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CompactRootMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub MinimalVerticalDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub MinimalRootMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub HiddenVerticalDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub HiddenRootMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IAppBarTemplateSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarTemplateSettings2 {
    type Vtable = IAppBarTemplateSettings2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBarTemplateSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xcbe66259_0399_5bcc_b925_4d5f5c9a4568,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NegativeCompactVerticalDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub NegativeMinimalVerticalDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub NegativeHiddenVerticalDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IAppBarToggleButtonTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarToggleButtonTemplateSettings {
    type Vtable = IAppBarToggleButtonTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBarToggleButtonTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xaaf99c48_d8f4_40d9_9fa3_3a64f0fec5d8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarToggleButtonTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IButtonBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IButtonBase {
    type Vtable = IButtonBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IButtonBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfa002c1a_494e_46cf_91d4_e14a8d798674,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ClickMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ClickMode,
    ) -> ::windows_core::HRESULT,
    pub SetClickMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ClickMode,
    ) -> ::windows_core::HRESULT,
    pub IsPointerOver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CommandParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCommandParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Click: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveClick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IButtonBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IButtonBaseFactory {
    type Vtable = IButtonBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IButtonBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x389b7c71_5220_42b2_9992_2690c1a6702f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IButtonBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IButtonBaseStatics {
    type Vtable = IButtonBaseStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IButtonBaseStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x67ef17e1_fe37_474f_9e97_3b5e0b30f2df,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ClickModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsPointerOverProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsPressedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CommandParameterProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICalendarPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarPanel {
    type Vtable = ICalendarPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICalendarPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfcd55a2d_02d3_4ee6_9a90_9df3ead00994,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICalendarViewTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarViewTemplateSettings {
    type Vtable = ICalendarViewTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICalendarViewTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x56c71483_64e1_477c_8a0b_cb2f3334b9b0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarViewTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinViewWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub HeaderText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WeekDay7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HasMoreContentAfter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HasMoreContentBefore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HasMoreViews: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ClipRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICarouselPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICarouselPanel {
    type Vtable = ICarouselPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICarouselPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdeab78b2_373b_4151_8785_e544d0d9362b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICarouselPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanVerticallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanVerticallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CanHorizontallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanHorizontallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ScrollOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetScrollOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: f64,
    ) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: f64,
    ) -> ::windows_core::HRESULT,
    pub MakeVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        visual: *mut ::core::ffi::c_void,
        rectangle: super::super::super::super::Foundation::Rect,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICarouselPanelFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICarouselPanelFactory {
    type Vtable = ICarouselPanelFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICarouselPanelFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc1109404_9ae1_440e_a0dd_bbb6e2293cbe,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICarouselPanelFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IColorPickerSlider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPickerSlider {
    type Vtable = IColorPickerSlider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorPickerSlider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x94394d83_e0df_4c5f_bbcd_8155f4020440,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSlider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorChannel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ColorPickerHsvChannel,
    ) -> ::windows_core::HRESULT,
    pub SetColorChannel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ColorPickerHsvChannel,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IColorPickerSliderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPickerSliderFactory {
    type Vtable = IColorPickerSliderFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorPickerSliderFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x06d879a2_8c07_4b1e_a940_9fbce8f49639,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSliderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IColorPickerSliderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPickerSliderStatics {
    type Vtable = IColorPickerSliderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorPickerSliderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x22eafc6a_9fe3_4eee_8734_a1398ec4413a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSliderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorChannelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IColorSpectrum(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorSpectrum {
    type Vtable = IColorSpectrum_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorSpectrum {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xce46f271_f509_4f98_8288_e4942fb385df,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrum_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Color,
    ) -> ::windows_core::HRESULT,
    HsvColor: usize,
    SetHsvColor: usize,
    pub MinHue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMinHue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub MaxHue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMaxHue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub MinSaturation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMinSaturation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub MaxSaturation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMaxSaturation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub MinValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMinValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMaxValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ColorSpectrumShape,
    ) -> ::windows_core::HRESULT,
    pub SetShape: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ColorSpectrumShape,
    ) -> ::windows_core::HRESULT,
    pub Components: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ColorSpectrumComponents,
    ) -> ::windows_core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ColorSpectrumComponents,
    ) -> ::windows_core::HRESULT,
    pub ColorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveColorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IColorSpectrumFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorSpectrumFactory {
    type Vtable = IColorSpectrumFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorSpectrumFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x90c7e61e_904d_42ab_b44f_e68dbf0cdee9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrumFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IColorSpectrumStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorSpectrumStatics {
    type Vtable = IColorSpectrumStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorSpectrumStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x906bee7c_2cee_4e90_968b_f0a5bd691b4a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrumStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HsvColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinHueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaxHueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinSaturationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaxSaturationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinValueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaxValueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShapeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ComponentsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IComboBoxTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComboBoxTemplateSettings {
    type Vtable = IComboBoxTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IComboBoxTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x83285c4e_17f6_4aa3_b61b_e87c718604ea,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DropDownOpenedHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub DropDownClosedHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub DropDownOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SelectedItemDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AnimationDirection,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IComboBoxTemplateSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComboBoxTemplateSettings2 {
    type Vtable = IComboBoxTemplateSettings2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IComboBoxTemplateSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x00e90cd7_68be_449d_b5a7_76e26f703e9b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DropDownContentMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarFlyoutCommandBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarFlyoutCommandBar {
    type Vtable = ICommandBarFlyoutCommandBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandBarFlyoutCommandBar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x14146e7c_38c4_55c4_b706_ce18f6061e7e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FlyoutTemplateSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarFlyoutCommandBarFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarFlyoutCommandBarFactory {
    type Vtable = ICommandBarFlyoutCommandBarFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandBarFlyoutCommandBarFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf8236f9f_5559_5697_8e6f_20d70ca17dd0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarFlyoutCommandBarTemplateSettings {
    type Vtable = ICommandBarFlyoutCommandBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface
for ICommandBarFlyoutCommandBarTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x47642c44_26ff_5d14_9cfc_77dc64f3a447,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpenAnimationStartPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OpenAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CloseAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CurrentWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandedWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub WidthExpansionDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub WidthExpansionAnimationStartPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub WidthExpansionAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub WidthExpansionMoreButtonAnimationStartPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub WidthExpansionMoreButtonAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandUpOverflowVerticalPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandDownOverflowVerticalPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandUpAnimationStartPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandUpAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandUpAnimationHoldPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandDownAnimationStartPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandDownAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExpandDownAnimationHoldPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ContentClipRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentClipRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings {
    type Vtable = ICommandBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandBarTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x61c8f92c_05aa_414a_a2ae_482c5a46c08e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContentHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentClipRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentMaxHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub NegativeOverflowContentHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarTemplateSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings2 {
    type Vtable = ICommandBarTemplateSettings2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandBarTemplateSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfbb24f93_c2e2_4177_a2b6_3cd705073cf6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OverflowContentMaxWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarTemplateSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings3 {
    type Vtable = ICommandBarTemplateSettings3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandBarTemplateSettings3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3bd71eba_3403_4bfe_842d_2ce8c511d245,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EffectiveOverflowButtonVisibility: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Visibility,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ICommandBarTemplateSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings4 {
    type Vtable = ICommandBarTemplateSettings4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandBarTemplateSettings4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf2562dd3_aa58_59c5_853b_828a19d1214e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OverflowContentCompactYTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentMinimalYTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OverflowContentHiddenYTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragCompletedEventArgs {
    type Vtable = IDragCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb04f29a1_bd16_48f6_a511_9c2763641331,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HorizontalChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Canceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragCompletedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragCompletedEventArgsFactory {
    type Vtable = IDragCompletedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragCompletedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x36a7d99d_148c_495f_a0fc_afc871d62f33,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragCompletedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        horizontalchange: f64,
        verticalchange: f64,
        canceled: bool,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragDeltaEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragDeltaEventArgs {
    type Vtable = IDragDeltaEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragDeltaEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2c2dd73c_2806_49fc_aae9_6d792b572b6a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDeltaEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HorizontalChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragDeltaEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragDeltaEventArgsFactory {
    type Vtable = IDragDeltaEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragDeltaEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x46e7a1ef_ae15_44a6_8a04_95b0bf9ab876,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDeltaEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithHorizontalChangeAndVerticalChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        horizontalchange: f64,
        verticalchange: f64,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragStartedEventArgs {
    type Vtable = IDragStartedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragStartedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9f915dd0_a124_4366_bd85_2408214aeed4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragStartedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragStartedEventArgsFactory {
    type Vtable = IDragStartedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragStartedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5eefe579_c706_4781_a308_c9e7f4c6a1d7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithHorizontalOffsetAndVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        horizontaloffset: f64,
        verticaloffset: f64,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase {
    type Vtable = IFlyoutBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x723eea0b_d12e_430d_a9f0_9bb32bbf9913,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Placement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FlyoutPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FlyoutPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Opening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveOpening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ShowAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        placementtarget: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBase2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase2 {
    type Vtable = IFlyoutBase2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBase2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf82b435e_65b3_41c6_a9e2_77b67bc4c00c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Target: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub LightDismissOverlayMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::LightDismissOverlayMode,
    ) -> ::windows_core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::LightDismissOverlayMode,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusWhenDisabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::ElementSoundMode,
    ) -> ::windows_core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::ElementSoundMode,
    ) -> ::windows_core::HRESULT,
    pub Closing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveClosing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBase3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase3 {
    type Vtable = IFlyoutBase3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBase3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa89c9712_48e0_4240_95b9_0dfd0826a8d3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OverlayInputPassThroughElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetOverlayInputPassThroughElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBase4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase4 {
    type Vtable = IFlyoutBase4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBase4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe3897d69_a37f_4828_9b70_0ef67c03b5f8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryInvokeKeyboardAccelerator: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBase5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase5 {
    type Vtable = IFlyoutBase5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBase5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xad3ec0c7_12bb_5a73_b78e_105192ca73d6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FlyoutShowMode,
    ) -> ::windows_core::HRESULT,
    pub SetShowMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FlyoutShowMode,
    ) -> ::windows_core::HRESULT,
    pub InputDevicePrefersPrimaryCommands: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub AreOpenCloseAnimationsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreOpenCloseAnimationsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ShowAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        placementtarget: *mut ::core::ffi::c_void,
        showoptions: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBase6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase6 {
    type Vtable = IFlyoutBase6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBase6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5399de8c_06cc_5b52_b65a_ff9322d1c940,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldConstrainToRootBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldConstrainToRootBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsConstrainedToRootBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseClosingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseClosingEventArgs {
    type Vtable = IFlyoutBaseClosingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseClosingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd075852d_b09a_4fd1_b005_db2ba01206fb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseClosingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseFactory {
    type Vtable = IFlyoutBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1c3363d7_fca7_407e_920e_70e15e9f0bf1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseOverrides {
    type Vtable = IFlyoutBaseOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x101dec86_6f4d_45a4_9d0e_3ece6f16977e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreatePresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseOverrides4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseOverrides4 {
    type Vtable = IFlyoutBaseOverrides4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseOverrides4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa6bfd04d_5ff3_4418_add8_4042a88d2da5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnProcessKeyboardAccelerators: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics {
    type Vtable = IFlyoutBaseStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe2d795e3_85c0_4de2_bac1_5294ca011a78,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlacementProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AttachedFlyoutProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAttachedFlyout: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAttachedFlyout: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShowAttachedFlyout: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        flyoutowner: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics2 {
    type Vtable = IFlyoutBaseStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa8e913fe_2d60_4307_aad9_56b450121b58,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusWhenDisabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics3 {
    type Vtable = IFlyoutBaseStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7ba92e4f_dd16_4be4_99db_bd9d4406c0f8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OverlayInputPassThroughElementProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics5 {
    type Vtable = IFlyoutBaseStatics5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x69edb25c_992a_542b_bcff_2f7f855523bd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShowModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InputDevicePrefersPrimaryCommandsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AreOpenCloseAnimationsEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutBaseStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics6 {
    type Vtable = IFlyoutBaseStatics6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutBaseStatics6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x96d49254_c91b_5246_8b39_afc2a2c50cf8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldConstrainToRootBoundsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutShowOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutShowOptions {
    type Vtable = IFlyoutShowOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutShowOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x57d693ad_0c74_54dd_b110_1ee43fabadd9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutShowOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ExclusionRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetExclusionRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShowMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FlyoutShowMode,
    ) -> ::windows_core::HRESULT,
    pub SetShowMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FlyoutShowMode,
    ) -> ::windows_core::HRESULT,
    pub Placement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FlyoutPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FlyoutPlacementMode,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFlyoutShowOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutShowOptionsFactory {
    type Vtable = IFlyoutShowOptionsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFlyoutShowOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xce596f61_2eb4_5b4e_af69_f9af42320eee,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutShowOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IGeneratorPositionHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneratorPositionHelper {
    type Vtable = IGeneratorPositionHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeneratorPositionHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xcd40318d_7745_40d9_ab9d_abbda4a7ffea,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneratorPositionHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IGeneratorPositionHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneratorPositionHelperStatics {
    type Vtable = IGeneratorPositionHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeneratorPositionHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xad4095cd_60ec_4588_8d60_39d29097a4df,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneratorPositionHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromIndexAndOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: i32,
        offset: i32,
        result__: *mut GeneratorPosition,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IGridViewItemPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemPresenter {
    type Vtable = IGridViewItemPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridViewItemPresenter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x214f9010_56e2_4821_8a1c_2305709af94b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectionCheckMarkVisualEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionCheckMarkVisualEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CheckHintBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckHintBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckSelectingBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckSelectingBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDragBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDragForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFocusBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PlaceholderBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPlaceholderBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedPointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedBorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub DisabledOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDisabledOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub DragOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDragOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ReorderHintOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetReorderHintOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    GridViewItemPresenterHorizontalContentAlignment: usize,
    SetGridViewItemPresenterHorizontalContentAlignment: usize,
    GridViewItemPresenterVerticalContentAlignment: usize,
    SetGridViewItemPresenterVerticalContentAlignment: usize,
    GridViewItemPresenterPadding: usize,
    SetGridViewItemPresenterPadding: usize,
    pub PointerOverBackgroundMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetPointerOverBackgroundMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub ContentMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetContentMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IGridViewItemPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemPresenterFactory {
    type Vtable = IGridViewItemPresenterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridViewItemPresenterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x53c12178_63bb_4a65_a3f1_ab114cfc6ffe,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IGridViewItemPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemPresenterStatics {
    type Vtable = IGridViewItemPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridViewItemPresenterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe958f8c4_277e_4a72_a01e_9e1688980178,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectionCheckMarkVisualEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckHintBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckSelectingBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PlaceholderBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBorderThicknessProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisabledOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ReorderHintOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    GridViewItemPresenterHorizontalContentAlignmentProperty: usize,
    GridViewItemPresenterVerticalContentAlignmentProperty: usize,
    GridViewItemPresenterPaddingProperty: usize,
    pub PointerOverBackgroundMarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ContentMarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IGridViewItemTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemTemplateSettings {
    type Vtable = IGridViewItemTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridViewItemTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9e30baaf_165d_4267_a45e_1a43a75706ac,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DragItemsCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IItemsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItemsChangedEventArgs {
    type Vtable = IItemsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IItemsChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe8b45568_7d10_421e_be29_81839a91de20,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GeneratorPosition,
    ) -> ::windows_core::HRESULT,
    pub OldPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GeneratorPosition,
    ) -> ::windows_core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ItemUICount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IJumpListItemBackgroundConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemBackgroundConverter {
    type Vtable = IJumpListItemBackgroundConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IJumpListItemBackgroundConverter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x81177858_d224_410c_b16c_c5b6bb6188b2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Disabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDisabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IJumpListItemBackgroundConverterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemBackgroundConverterStatics {
    type Vtable = IJumpListItemBackgroundConverterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IJumpListItemBackgroundConverterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x20e7c3dd_6f27_4808_b0be_83e0e9b5cc45,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IJumpListItemForegroundConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemForegroundConverter {
    type Vtable = IJumpListItemForegroundConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IJumpListItemForegroundConverter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1590ed38_c504_4796_a63a_5bfc9eefaae8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Disabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDisabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IJumpListItemForegroundConverterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemForegroundConverterStatics {
    type Vtable = IJumpListItemForegroundConverterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IJumpListItemForegroundConverterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x474e7352_210c_4673_ac6a_413f0e2c7750,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILayoutInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILayoutInformation {
    type Vtable = ILayoutInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILayoutInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb5384c9b_c8cf_41b3_bf16_18c8420e72c9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILayoutInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILayoutInformationStatics {
    type Vtable = ILayoutInformationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILayoutInformationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xcf06cf99_58e9_4682_8326_50caab65ed7c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetLayoutExceptionElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dispatcher: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetLayoutSlot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILayoutInformationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILayoutInformationStatics2 {
    type Vtable = ILayoutInformationStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILayoutInformationStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x760315b5_6d4e_4939_ac61_639863cea36b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAvailableSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter {
    type Vtable = IListViewItemPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfc8946bd_a3a2_4969_8174_25b5d3c28033,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectionCheckMarkVisualEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionCheckMarkVisualEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CheckHintBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckHintBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckSelectingBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckSelectingBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDragBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDragForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFocusBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PlaceholderBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPlaceholderBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedPointerOverBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedBorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub DisabledOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDisabledOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub DragOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDragOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ReorderHintOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetReorderHintOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    ListViewItemPresenterHorizontalContentAlignment: usize,
    SetListViewItemPresenterHorizontalContentAlignment: usize,
    ListViewItemPresenterVerticalContentAlignment: usize,
    SetListViewItemPresenterVerticalContentAlignment: usize,
    ListViewItemPresenterPadding: usize,
    SetListViewItemPresenterPadding: usize,
    pub PointerOverBackgroundMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetPointerOverBackgroundMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub ContentMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetContentMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter2 {
    type Vtable = IListViewItemPresenter2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf5dc5496_e122_4c57_a625_ac4b08fb2d4c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedPressedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedPressedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PressedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPressedBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusSecondaryBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFocusSecondaryBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ListViewItemPresenterCheckMode,
    ) -> ::windows_core::HRESULT,
    pub SetCheckMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ListViewItemPresenterCheckMode,
    ) -> ::windows_core::HRESULT,
    pub PointerOverForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPointerOverForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenter3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter3 {
    type Vtable = IListViewItemPresenter3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenter3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x36620013_0390_4e30_ad97_8744404f7010,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RevealBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRevealBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RevealBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRevealBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RevealBorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetRevealBorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub RevealBackgroundShowsAboveContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetRevealBackgroundShowsAboveContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenter4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter4 {
    type Vtable = IListViewItemPresenter4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenter4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xda600ac1_adea_5940_a18f_57582f96d99a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedDisabledBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedDisabledBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPointerOverBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxPointerOverBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxSelectedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedPointerOverBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxSelectedPointerOverBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxSelectedPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxSelectedDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPressedBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxPressedBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxDisabledBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxDisabledBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxCornerRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::CornerRadius,
    ) -> ::windows_core::HRESULT,
    pub SetCheckBoxCornerRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::CornerRadius,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorCornerRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::CornerRadius,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorCornerRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::CornerRadius,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorVisualEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorVisualEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ListViewItemPresenterSelectionIndicatorMode,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ListViewItemPresenterSelectionIndicatorMode,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorPointerOverBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorPointerOverBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorPressedBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorDisabledBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPressedBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedPressedBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedDisabledBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedDisabledBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedInnerBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedInnerBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPointerOverBorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterFactory {
    type Vtable = IListViewItemPresenterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe0777cfd_f7e4_4a67_9ac0_a994fcacd020,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics {
    type Vtable = IListViewItemPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6504a55a_15dd_42fb_aa5d_2d8ce2e9c294,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectionCheckMarkVisualEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckHintBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckSelectingBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PlaceholderBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBorderThicknessProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisabledOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ReorderHintOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    ListViewItemPresenterHorizontalContentAlignmentProperty: usize,
    ListViewItemPresenterVerticalContentAlignmentProperty: usize,
    ListViewItemPresenterPaddingProperty: usize,
    pub PointerOverBackgroundMarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ContentMarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenterStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics2 {
    type Vtable = IListViewItemPresenterStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenterStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4cb3b945_d24d_42a3_9e83_a86d0618bf1b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedPressedBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PressedBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusSecondaryBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenterStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics3 {
    type Vtable = IListViewItemPresenterStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenterStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc3d3d11e_fa26_4ce7_a4ed_ff948f01b7c0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RevealBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RevealBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RevealBorderThicknessProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RevealBackgroundShowsAboveContentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemPresenterStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics4 {
    type Vtable = IListViewItemPresenterStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemPresenterStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3917159e_74a1_5e7e_a743_e45be9fb919b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedDisabledBackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckPressedBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckDisabledBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPointerOverBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPressedBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxDisabledBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedPointerOverBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedPressedBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedDisabledBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPointerOverBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxPressedBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxDisabledBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CheckBoxCornerRadiusProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorCornerRadiusProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorVisualEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorPointerOverBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorPressedBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionIndicatorDisabledBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedPressedBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedDisabledBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedInnerBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerOverBorderBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IListViewItemTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemTemplateSettings {
    type Vtable = IListViewItemTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IListViewItemTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x67af84bf_8279_4686_9326_cd189f27575d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DragItemsCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILoopingSelector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelector {
    type Vtable = ILoopingSelector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoopingSelector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4c9a3e04_4827_49d9_8806_093957b0fd21,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldLoop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldLoop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Items: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetItemWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub ItemHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetItemHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub ItemTemplate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveSelectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILoopingSelectorItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelectorItem {
    type Vtable = ILoopingSelectorItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoopingSelectorItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc69714b9_27c6_4433_9d7c_0dbfb2f4344f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILoopingSelectorPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelectorPanel {
    type Vtable = ILoopingSelectorPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoopingSelectorPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x40a9ba70_1011_4778_87f7_6bfd20d6377d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ILoopingSelectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelectorStatics {
    type Vtable = ILoopingSelectorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoopingSelectorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x03e8bafa_8c7d_4fc5_b92a_f049fb933cc5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldLoopProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IMenuFlyoutItemTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMenuFlyoutItemTemplateSettings {
    type Vtable = IMenuFlyoutItemTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMenuFlyoutItemTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x56ad1809_3a16_4147_81cb_d0b35c834e0f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutItemTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IMenuFlyoutPresenterTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMenuFlyoutPresenterTemplateSettings {
    type Vtable = IMenuFlyoutPresenterTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMenuFlyoutPresenterTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd68fd00d_629d_4349_ac51_b877c80983b8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutPresenterTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FlyoutContentMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct INavigationViewItemPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationViewItemPresenter {
    type Vtable = INavigationViewItemPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INavigationViewItemPresenter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9956d3fc_4693_59cb_b6bf_37249058be96,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Icon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct INavigationViewItemPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationViewItemPresenterFactory {
    type Vtable = INavigationViewItemPresenterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INavigationViewItemPresenterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbb062c50_4a36_52e7_9459_e89d02f3fc42,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct INavigationViewItemPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationViewItemPresenterStatics {
    type Vtable = INavigationViewItemPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INavigationViewItemPresenterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x52814604_cfc1_5ad5_a3aa_fa355be6bd76,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IconProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IOrientedVirtualizingPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientedVirtualizingPanel {
    type Vtable = IOrientedVirtualizingPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOrientedVirtualizingPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf077b577_39bd_46ee_bdd7_0826beed71b8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanVerticallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanVerticallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CanHorizontallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanHorizontallyScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ScrollOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetScrollOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PageRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: f64,
    ) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: f64,
    ) -> ::windows_core::HRESULT,
    pub MakeVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        visual: *mut ::core::ffi::c_void,
        rectangle: super::super::super::super::Foundation::Rect,
        result__: *mut super::super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IOrientedVirtualizingPanelFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientedVirtualizingPanelFactory {
    type Vtable = IOrientedVirtualizingPanelFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOrientedVirtualizingPanelFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7b8eaeaf_f92f_439d_9ebf_e9919f56c94d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanelFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPickerFlyoutBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBase {
    type Vtable = IPickerFlyoutBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPickerFlyoutBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe33574ea_1076_44d1_9383_dc24ac5cff2a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPickerFlyoutBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBaseFactory {
    type Vtable = IPickerFlyoutBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPickerFlyoutBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7ec27a53_9502_4beb_b342_00566c8f16b0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPickerFlyoutBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBaseOverrides {
    type Vtable = IPickerFlyoutBaseOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPickerFlyoutBaseOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5bfc4f4a_4822_47b4_a958_77c20ba120d3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnConfirmed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShouldShowConfirmationButtons: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPickerFlyoutBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBaseStatics {
    type Vtable = IPickerFlyoutBaseStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPickerFlyoutBaseStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5a4d0ac5_89ae_40e5_a7f1_bb702355adf3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TitleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPivotHeaderItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotHeaderItem {
    type Vtable = IPivotHeaderItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPivotHeaderItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x594572c2_82aa_410b_9e55_fd8e2c98862d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPivotHeaderItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotHeaderItemFactory {
    type Vtable = IPivotHeaderItemFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPivotHeaderItemFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x14308b37_185b_4117_bc77_dda2eb261b99,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPivotHeaderPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotHeaderPanel {
    type Vtable = IPivotHeaderPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPivotHeaderPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x21484ebc_9241_4203_bd37_6c08fb096612,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPivotPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotPanel {
    type Vtable = IPivotPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPivotPanel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xad4ebe80_22a9_4ca3_9212_2773b6359ff3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotPanel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup {
    type Vtable = IPopup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x62418240_e6d3_4705_a1dc_39156456ee29,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Child: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    ChildTransitions: usize,
    SetChildTransitions: usize,
    pub IsLightDismissEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsLightDismissEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopup2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup2 {
    type Vtable = IPopup2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopup2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x376a8c4c_aac0_4b20_966a_0b9364feb4b5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LightDismissOverlayMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::LightDismissOverlayMode,
    ) -> ::windows_core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::LightDismissOverlayMode,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopup3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup3 {
    type Vtable = IPopup3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopup3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf9c46915_a65c_5f68_9f54_310a1b51095f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldConstrainToRootBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldConstrainToRootBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsConstrainedToRootBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopup4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup4 {
    type Vtable = IPopup4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopup4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1870b836_df2f_5fc6_a5f2_748ed6ce7321,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlacementTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPlacementTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DesiredPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PopupPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: PopupPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub ActualPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PopupPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub ActualPlacementChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveActualPlacementChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics {
    type Vtable = IPopupStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopupStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5ae3bf1a_6e34_40d6_8a7f_ca822aaf59e3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChildProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChildTransitionsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsLightDismissEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopupStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics2 {
    type Vtable = IPopupStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopupStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2b9ae9ec_55ef_43b6_b459_12e40ffa4302,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopupStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics3 {
    type Vtable = IPopupStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopupStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x00789589_c580_558f_945a_7d02ee004d3e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldConstrainToRootBoundsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IPopupStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics4 {
    type Vtable = IPopupStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPopupStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd1a42c06_8bfa_5164_8554_48bfe6bd4cc6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlacementTargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DesiredPlacementProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IProgressBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProgressBarTemplateSettings {
    type Vtable = IProgressBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProgressBarTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3fe2ea2a_e3f2_4c2b_9488_918d77d2bbe4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EllipseDiameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub EllipseOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub EllipseAnimationWellPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub EllipseAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ContainerAnimationStartPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ContainerAnimationEndPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub IndicatorLengthDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IProgressRingTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProgressRingTemplateSettings {
    type Vtable = IProgressRingTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProgressRingTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb9b675ec_c723_42e6_83e9_9826272bdc0e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressRingTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EllipseDiameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub EllipseOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub MaxSideLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRangeBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBase {
    type Vtable = IRangeBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfa002c1a_494e_46cf_91d4_e14a8d798675,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Minimum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMinimum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Maximum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMaximum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetSmallChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLargeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ValueChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveValueChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRangeBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseFactory {
    type Vtable = IRangeBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x389b7c71_5220_42b2_9992_2690c1a67030,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRangeBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseOverrides {
    type Vtable = IRangeBaseOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeBaseOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4291af39_7f0b_4bc2_99c4_06e7062682d8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnMinimumChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        oldminimum: f64,
        newminimum: f64,
    ) -> ::windows_core::HRESULT,
    pub OnMaximumChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        oldmaximum: f64,
        newmaximum: f64,
    ) -> ::windows_core::HRESULT,
    pub OnValueChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        oldvalue: f64,
        newvalue: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRangeBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseStatics {
    type Vtable = IRangeBaseStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeBaseStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x67ef17e1_fe37_474f_9e97_3b5e0b30f2e0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinimumProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaximumProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SmallChangeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LargeChangeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRangeBaseValueChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseValueChangedEventArgs {
    type Vtable = IRangeBaseValueChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeBaseValueChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa1921777_d5c1_4f9c_a7b0_0401b7e6dc5c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseValueChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRepeatButton(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepeatButton {
    type Vtable = IRepeatButton_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRepeatButton {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x02200df9_021a_484a_a93b_0f31020314e5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButton_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Delay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IRepeatButtonStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepeatButtonStatics {
    type Vtable = IRepeatButtonStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRepeatButtonStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3914ac4e_f462_4f73_8197_e8846639c682,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButtonStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DelayProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IntervalProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IScrollBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollBar {
    type Vtable = IScrollBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollBar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf57ae6ca_d1a6_4b90_a4e9_54df1ba8d2ec,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Orientation,
    ) -> ::windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Orientation,
    ) -> ::windows_core::HRESULT,
    pub ViewportSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetViewportSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub IndicatorMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ScrollingIndicatorMode,
    ) -> ::windows_core::HRESULT,
    pub SetIndicatorMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ScrollingIndicatorMode,
    ) -> ::windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IScrollBarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollBarStatics {
    type Vtable = IScrollBarStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollBarStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x45eaf38d_b814_48cf_97f2_539eb16dfd4d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBarStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OrientationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ViewportSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IndicatorModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IScrollEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollEventArgs {
    type Vtable = IScrollEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc57e5168_3afe_448d_b752_2f364c75d743,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NewValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ScrollEventType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ScrollEventType,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IScrollSnapPointsInfo(::windows_core::IUnknown);
impl IScrollSnapPointsInfo {}
::windows_core::imp::interface_hierarchy!(
    IScrollSnapPointsInfo, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IScrollSnapPointsInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{1b5d1336-e61b-4d51-be41-fd8ddc55c58c}",
    );
}
unsafe impl ::windows_core::Interface for IScrollSnapPointsInfo {
    type Vtable = IScrollSnapPointsInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollSnapPointsInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1b5d1336_e61b_4d51_be41_fd8ddc55c58c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollSnapPointsInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AreHorizontalSnapPointsRegular: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub AreVerticalSnapPointsRegular: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HorizontalSnapPointsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveHorizontalSnapPointsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub VerticalSnapPointsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveVerticalSnapPointsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub GetIrregularSnapPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        orientation: super::Orientation,
        alignment: SnapPointsAlignment,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetRegularSnapPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        orientation: super::Orientation,
        alignment: SnapPointsAlignment,
        offset: *mut f32,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISelector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelector {
    type Vtable = ISelector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe30eb3a5_b36b_42dc_8527_cd25136c083c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedValuePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedValuePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsSynchronizedWithCurrentItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetIsSynchronizedWithCurrentItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveSelectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISelectorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorFactory {
    type Vtable = ISelectorFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc9be2995_d136_4600_b187_8ad56079b48a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISelectorItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorItem {
    type Vtable = ISelectorItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectorItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x541c8d6c_0283_4581_b945_2a64c28a0646,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISelectorItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorItemFactory {
    type Vtable = ISelectorItemFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectorItemFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb9363945_c86a_4b1e_9440_1879378d5313,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISelectorItemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorItemStatics {
    type Vtable = ISelectorItemStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectorItemStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2a353ab8_cbe9_4303_92e7_c8906e218392,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSelectedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISelectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorStatics {
    type Vtable = ISelectorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x13300b06_bd10_4e09_bff7_71efb8bbb42b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedValueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectedValuePathProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsSynchronizedWithCurrentItemProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIsSelectionActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISettingsFlyoutTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISettingsFlyoutTemplateSettings {
    type Vtable = ISettingsFlyoutTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISettingsFlyoutTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbcf14c10_cea7_43f1_9d68_57605ded69d4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsFlyoutTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HeaderBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HeaderForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BorderBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BorderThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub IconSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    ContentTransitions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISplitViewTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplitViewTemplateSettings {
    type Vtable = ISplitViewTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISplitViewTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc16ab5a7_4996_4443_b199_6b6b89124eab,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitViewTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpenPaneLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub NegativeOpenPaneLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OpenPaneLengthMinusCompactLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub NegativeOpenPaneLengthMinusCompactLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub OpenPaneGridLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::GridLength,
    ) -> ::windows_core::HRESULT,
    pub CompactPaneGridLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::GridLength,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IThumb(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThumb {
    type Vtable = IThumb_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThumb {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe8b2b281_0d6a_45cf_b333_2402b037f099,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumb_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsDragging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DragStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DragDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DragCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub CancelDrag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IThumbStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThumbStatics {
    type Vtable = IThumbStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThumbStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x955024eb_36f3_4672_a186_baaf626ac4ad,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsDraggingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ITickBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITickBar {
    type Vtable = ITickBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITickBar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x994683fa_f1f6_487d_a5ac_c15921bfa995,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITickBar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Fill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ITickBarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITickBarStatics {
    type Vtable = ITickBarStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITickBarStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2c6d7e40_799d_4a54_be09_1fefc61d018e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITickBarStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IToggleButton(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButton {
    type Vtable = IToggleButton_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToggleButton {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x589877fb_0fc7_4036_9d8b_127dfa75c16d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButton_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsChecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsThreeState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsThreeState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Checked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveChecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Unchecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveUnchecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Indeterminate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveIndeterminate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IToggleButtonFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButtonFactory {
    type Vtable = IToggleButtonFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToggleButtonFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd56aa2fc_fc7f_449c_9855_7a1055d668a8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IToggleButtonOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButtonOverrides {
    type Vtable = IToggleButtonOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToggleButtonOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd20e4c28_f18b_491a_9a45_f1a04a9369a4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnToggle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IToggleButtonStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButtonStatics {
    type Vtable = IToggleButtonStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToggleButtonStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xaf1eab12_0128_4f67_9c5a_82320c445d19,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCheckedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsThreeStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IToggleSwitchTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleSwitchTemplateSettings {
    type Vtable = IToggleSwitchTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToggleSwitchTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x02b7bdcd_628a_4363_86e0_51d6e2e89e58,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleSwitchTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KnobCurrentToOnOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub KnobCurrentToOffOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub KnobOnToOffOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub KnobOffToOnOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CurtainCurrentToOnOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CurtainCurrentToOffOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CurtainOnToOffOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub CurtainOffToOnOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IToolTipTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToolTipTemplateSettings {
    type Vtable = IToolTipTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToolTipTemplateSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd4388247_0ec4_4506_affd_afac2225b48c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTipTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct AppBarButtonTemplateSettings(::windows_core::IUnknown);
impl AppBarButtonTemplateSettings {}
impl ::windows_core::RuntimeType for AppBarButtonTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarButtonTemplateSettings;{cbc9b39d-0c95-4951-bff2-13963691c366})",
    );
}
unsafe impl ::windows_core::Interface for AppBarButtonTemplateSettings {
    type Vtable = IAppBarButtonTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBarButtonTemplateSettings {
    const IID: ::windows_core::GUID = <IAppBarButtonTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBarButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarButtonTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    AppBarButtonTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for AppBarButtonTemplateSettings {}
unsafe impl ::core::marker::Send for AppBarButtonTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarButtonTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct AppBarTemplateSettings(::windows_core::IUnknown);
impl AppBarTemplateSettings {}
impl ::windows_core::RuntimeType for AppBarTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarTemplateSettings;{bcc2a863-eb35-423c-8389-d7827be3bf67})",
    );
}
unsafe impl ::windows_core::Interface for AppBarTemplateSettings {
    type Vtable = IAppBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBarTemplateSettings {
    const IID: ::windows_core::GUID = <IAppBarTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    AppBarTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for AppBarTemplateSettings {}
unsafe impl ::core::marker::Send for AppBarTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct AppBarToggleButtonTemplateSettings(::windows_core::IUnknown);
impl AppBarToggleButtonTemplateSettings {}
impl ::windows_core::RuntimeType for AppBarToggleButtonTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarToggleButtonTemplateSettings;{aaf99c48-d8f4-40d9-9fa3-3a64f0fec5d8})",
    );
}
unsafe impl ::windows_core::Interface for AppBarToggleButtonTemplateSettings {
    type Vtable = IAppBarToggleButtonTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBarToggleButtonTemplateSettings {
    const IID: ::windows_core::GUID = <IAppBarToggleButtonTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBarToggleButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarToggleButtonTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    AppBarToggleButtonTemplateSettings, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for AppBarToggleButtonTemplateSettings {}
unsafe impl ::core::marker::Send for AppBarToggleButtonTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarToggleButtonTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ButtonBase(::windows_core::IUnknown);
impl ButtonBase {}
impl ::windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ButtonBase;{fa002c1a-494e-46cf-91d4-e14a8d798674})",
    );
}
unsafe impl ::windows_core::Interface for ButtonBase {
    type Vtable = IButtonBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ButtonBase {
    const IID: ::windows_core::GUID = <IButtonBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ButtonBase";
}
::windows_core::imp::interface_hierarchy!(
    ButtonBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentControl> for ButtonBase {}
impl ::windows_core::CanTryInto<super::Control> for ButtonBase {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for ButtonBase {}
impl ::windows_core::CanTryInto<super::super::UIElement> for ButtonBase {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for ButtonBase {}
unsafe impl ::core::marker::Send for ButtonBase {}
unsafe impl ::core::marker::Sync for ButtonBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CalendarPanel(::windows_core::IUnknown);
impl CalendarPanel {}
impl ::windows_core::RuntimeType for CalendarPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.CalendarPanel;{fcd55a2d-02d3-4ee6-9a90-9df3ead00994})",
    );
}
unsafe impl ::windows_core::Interface for CalendarPanel {
    type Vtable = ICalendarPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CalendarPanel {
    const IID: ::windows_core::GUID = <ICalendarPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CalendarPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CalendarPanel";
}
::windows_core::imp::interface_hierarchy!(
    CalendarPanel, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Panel> for CalendarPanel {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for CalendarPanel {}
impl ::windows_core::CanTryInto<super::super::UIElement> for CalendarPanel {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for CalendarPanel {}
unsafe impl ::core::marker::Send for CalendarPanel {}
unsafe impl ::core::marker::Sync for CalendarPanel {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CalendarViewTemplateSettings(::windows_core::IUnknown);
impl CalendarViewTemplateSettings {}
impl ::windows_core::RuntimeType for CalendarViewTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.CalendarViewTemplateSettings;{56c71483-64e1-477c-8a0b-cb2f3334b9b0})",
    );
}
unsafe impl ::windows_core::Interface for CalendarViewTemplateSettings {
    type Vtable = ICalendarViewTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CalendarViewTemplateSettings {
    const IID: ::windows_core::GUID = <ICalendarViewTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CalendarViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CalendarViewTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    CalendarViewTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for CalendarViewTemplateSettings {}
unsafe impl ::core::marker::Send for CalendarViewTemplateSettings {}
unsafe impl ::core::marker::Sync for CalendarViewTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CarouselPanel(::windows_core::IUnknown);
impl CarouselPanel {}
impl ::windows_core::RuntimeType for CarouselPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.CarouselPanel;{deab78b2-373b-4151-8785-e544d0d9362b})",
    );
}
unsafe impl ::windows_core::Interface for CarouselPanel {
    type Vtable = ICarouselPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CarouselPanel {
    const IID: ::windows_core::GUID = <ICarouselPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CarouselPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CarouselPanel";
}
::windows_core::imp::interface_hierarchy!(
    CarouselPanel, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IScrollSnapPointsInfo> for CarouselPanel {}
impl ::windows_core::CanTryInto<super::VirtualizingPanel> for CarouselPanel {}
impl ::windows_core::CanTryInto<super::Panel> for CarouselPanel {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for CarouselPanel {}
impl ::windows_core::CanTryInto<super::super::UIElement> for CarouselPanel {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for CarouselPanel {}
unsafe impl ::core::marker::Send for CarouselPanel {}
unsafe impl ::core::marker::Sync for CarouselPanel {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ColorPickerSlider(::windows_core::IUnknown);
impl ColorPickerSlider {}
impl ::windows_core::RuntimeType for ColorPickerSlider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ColorPickerSlider;{94394d83-e0df-4c5f-bbcd-8155f4020440})",
    );
}
unsafe impl ::windows_core::Interface for ColorPickerSlider {
    type Vtable = IColorPickerSlider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ColorPickerSlider {
    const IID: ::windows_core::GUID = <IColorPickerSlider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ColorPickerSlider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ColorPickerSlider";
}
::windows_core::imp::interface_hierarchy!(
    ColorPickerSlider, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Slider> for ColorPickerSlider {}
impl ::windows_core::CanTryInto<RangeBase> for ColorPickerSlider {}
impl ::windows_core::CanTryInto<super::Control> for ColorPickerSlider {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for ColorPickerSlider {}
impl ::windows_core::CanTryInto<super::super::UIElement> for ColorPickerSlider {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for ColorPickerSlider {}
unsafe impl ::core::marker::Send for ColorPickerSlider {}
unsafe impl ::core::marker::Sync for ColorPickerSlider {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ColorSpectrum(::windows_core::IUnknown);
impl ColorSpectrum {}
impl ::windows_core::RuntimeType for ColorSpectrum {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ColorSpectrum;{ce46f271-f509-4f98-8288-e4942fb385df})",
    );
}
unsafe impl ::windows_core::Interface for ColorSpectrum {
    type Vtable = IColorSpectrum_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ColorSpectrum {
    const IID: ::windows_core::GUID = <IColorSpectrum as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ColorSpectrum {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ColorSpectrum";
}
::windows_core::imp::interface_hierarchy!(
    ColorSpectrum, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Control> for ColorSpectrum {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for ColorSpectrum {}
impl ::windows_core::CanTryInto<super::super::UIElement> for ColorSpectrum {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for ColorSpectrum {}
unsafe impl ::core::marker::Send for ColorSpectrum {}
unsafe impl ::core::marker::Sync for ColorSpectrum {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ComboBoxTemplateSettings(::windows_core::IUnknown);
impl ComboBoxTemplateSettings {}
impl ::windows_core::RuntimeType for ComboBoxTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ComboBoxTemplateSettings;{83285c4e-17f6-4aa3-b61b-e87c718604ea})",
    );
}
unsafe impl ::windows_core::Interface for ComboBoxTemplateSettings {
    type Vtable = IComboBoxTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ComboBoxTemplateSettings {
    const IID: ::windows_core::GUID = <IComboBoxTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ComboBoxTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ComboBoxTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    ComboBoxTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ComboBoxTemplateSettings {}
unsafe impl ::core::marker::Send for ComboBoxTemplateSettings {}
unsafe impl ::core::marker::Sync for ComboBoxTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CommandBarFlyoutCommandBar(::windows_core::IUnknown);
impl CommandBarFlyoutCommandBar {}
impl ::windows_core::RuntimeType for CommandBarFlyoutCommandBar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar;{14146e7c-38c4-55c4-b706-ce18f6061e7e})",
    );
}
unsafe impl ::windows_core::Interface for CommandBarFlyoutCommandBar {
    type Vtable = ICommandBarFlyoutCommandBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CommandBarFlyoutCommandBar {
    const IID: ::windows_core::GUID = <ICommandBarFlyoutCommandBar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CommandBarFlyoutCommandBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar";
}
::windows_core::imp::interface_hierarchy!(
    CommandBarFlyoutCommandBar, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::CommandBar> for CommandBarFlyoutCommandBar {}
impl ::windows_core::CanTryInto<super::AppBar> for CommandBarFlyoutCommandBar {}
impl ::windows_core::CanTryInto<super::ContentControl> for CommandBarFlyoutCommandBar {}
impl ::windows_core::CanTryInto<super::Control> for CommandBarFlyoutCommandBar {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement>
for CommandBarFlyoutCommandBar {}
impl ::windows_core::CanTryInto<super::super::UIElement> for CommandBarFlyoutCommandBar {}
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for CommandBarFlyoutCommandBar {}
unsafe impl ::core::marker::Send for CommandBarFlyoutCommandBar {}
unsafe impl ::core::marker::Sync for CommandBarFlyoutCommandBar {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CommandBarFlyoutCommandBarTemplateSettings(::windows_core::IUnknown);
impl CommandBarFlyoutCommandBarTemplateSettings {}
impl ::windows_core::RuntimeType for CommandBarFlyoutCommandBarTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarTemplateSettings;{47642c44-26ff-5d14-9cfc-77dc64f3a447})",
    );
}
unsafe impl ::windows_core::Interface for CommandBarFlyoutCommandBarTemplateSettings {
    type Vtable = ICommandBarFlyoutCommandBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CommandBarFlyoutCommandBarTemplateSettings {
    const IID: ::windows_core::GUID = <ICommandBarFlyoutCommandBarTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CommandBarFlyoutCommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    CommandBarFlyoutCommandBarTemplateSettings, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for CommandBarFlyoutCommandBarTemplateSettings {}
unsafe impl ::core::marker::Send for CommandBarFlyoutCommandBarTemplateSettings {}
unsafe impl ::core::marker::Sync for CommandBarFlyoutCommandBarTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CommandBarTemplateSettings(::windows_core::IUnknown);
impl CommandBarTemplateSettings {}
impl ::windows_core::RuntimeType for CommandBarTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarTemplateSettings;{61c8f92c-05aa-414a-a2ae-482c5a46c08e})",
    );
}
unsafe impl ::windows_core::Interface for CommandBarTemplateSettings {
    type Vtable = ICommandBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CommandBarTemplateSettings {
    const IID: ::windows_core::GUID = <ICommandBarTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    CommandBarTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for CommandBarTemplateSettings {}
unsafe impl ::core::marker::Send for CommandBarTemplateSettings {}
unsafe impl ::core::marker::Sync for CommandBarTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragCompletedEventArgs(::windows_core::IUnknown);
impl DragCompletedEventArgs {}
impl ::windows_core::RuntimeType for DragCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.DragCompletedEventArgs;{b04f29a1-bd16-48f6-a511-9c2763641331})",
    );
}
unsafe impl ::windows_core::Interface for DragCompletedEventArgs {
    type Vtable = IDragCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragCompletedEventArgs {
    const IID: ::windows_core::GUID = <IDragCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DragCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::RoutedEventArgs>
for DragCompletedEventArgs {}
unsafe impl ::core::marker::Send for DragCompletedEventArgs {}
unsafe impl ::core::marker::Sync for DragCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragDeltaEventArgs(::windows_core::IUnknown);
impl DragDeltaEventArgs {}
impl ::windows_core::RuntimeType for DragDeltaEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.DragDeltaEventArgs;{2c2dd73c-2806-49fc-aae9-6d792b572b6a})",
    );
}
unsafe impl ::windows_core::Interface for DragDeltaEventArgs {
    type Vtable = IDragDeltaEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragDeltaEventArgs {
    const IID: ::windows_core::GUID = <IDragDeltaEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragDeltaEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragDeltaEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DragDeltaEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::RoutedEventArgs> for DragDeltaEventArgs {}
unsafe impl ::core::marker::Send for DragDeltaEventArgs {}
unsafe impl ::core::marker::Sync for DragDeltaEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragStartedEventArgs(::windows_core::IUnknown);
impl DragStartedEventArgs {}
impl ::windows_core::RuntimeType for DragStartedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.DragStartedEventArgs;{9f915dd0-a124-4366-bd85-2408214aeed4})",
    );
}
unsafe impl ::windows_core::Interface for DragStartedEventArgs {
    type Vtable = IDragStartedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragStartedEventArgs {
    const IID: ::windows_core::GUID = <IDragStartedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragStartedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DragStartedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::RoutedEventArgs> for DragStartedEventArgs {}
unsafe impl ::core::marker::Send for DragStartedEventArgs {}
unsafe impl ::core::marker::Sync for DragStartedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FlyoutBase(::windows_core::IUnknown);
impl FlyoutBase {
    pub fn GetValue<P0>(
        &self,
        dp: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .GetValue)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetValue)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                    value.into_param().abi(),
                )
                .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .ClearValue)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                )
                .ok()
        }
    }
    pub fn ReadLocalValue<P0>(
        &self,
        dp: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ReadLocalValue)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .GetAnimationBaseValue)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn Dispatcher(
        &self,
    ) -> ::windows_core::Result<super::super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject2,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .RegisterPropertyChangedCallback)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                    callback.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IDependencyObject2,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .UnregisterPropertyChangedCallback)(
                    ::windows_core::Interface::as_raw(this),
                    dp.into_param().abi(),
                    token,
                )
                .ok()
        }
    }
    pub fn Placement(&self) -> ::windows_core::Result<FlyoutPlacementMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Placement)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetPlacement(
        &self,
        value: FlyoutPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetPlacement)(::windows_core::Interface::as_raw(this), value)
                .ok()
        }
    }
    pub fn Opened<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<
        super::super::super::super::Foundation::EventRegistrationToken,
    >
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Foundation::EventHandler<
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Opened)(
                    ::windows_core::Interface::as_raw(this),
                    handler.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn RemoveOpened(
        &self,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .RemoveOpened)(::windows_core::Interface::as_raw(this), token)
                .ok()
        }
    }
    pub fn Closed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<
        super::super::super::super::Foundation::EventRegistrationToken,
    >
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Foundation::EventHandler<
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Closed)(
                    ::windows_core::Interface::as_raw(this),
                    handler.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn RemoveClosed(
        &self,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .RemoveClosed)(::windows_core::Interface::as_raw(this), token)
                .ok()
        }
    }
    pub fn Opening<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<
        super::super::super::super::Foundation::EventRegistrationToken,
    >
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Foundation::EventHandler<
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Opening)(
                    ::windows_core::Interface::as_raw(this),
                    handler.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn RemoveOpening(
        &self,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .RemoveOpening)(::windows_core::Interface::as_raw(this), token)
                .ok()
        }
    }
    pub fn ShowAt<P0>(&self, placementtarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::FrameworkElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .ShowAt)(
                    ::windows_core::Interface::as_raw(this),
                    placementtarget.try_into_param()?.abi(),
                )
                .ok()
        }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .Hide)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Target(&self) -> ::windows_core::Result<super::super::FrameworkElement> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Target)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AllowFocusOnInteraction)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetAllowFocusOnInteraction)(
                    ::windows_core::Interface::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn LightDismissOverlayMode(
        &self,
    ) -> ::windows_core::Result<super::LightDismissOverlayMode> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .LightDismissOverlayMode)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetLightDismissOverlayMode(
        &self,
        value: super::LightDismissOverlayMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetLightDismissOverlayMode)(
                    ::windows_core::Interface::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AllowFocusWhenDisabled)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetAllowFocusWhenDisabled)(
                    ::windows_core::Interface::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn ElementSoundMode(
        &self,
    ) -> ::windows_core::Result<super::super::ElementSoundMode> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ElementSoundMode)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: super::super::ElementSoundMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetElementSoundMode)(::windows_core::Interface::as_raw(this), value)
                .ok()
        }
    }
    pub fn Closing<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<
        super::super::super::super::Foundation::EventRegistrationToken,
    >
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Foundation::TypedEventHandler<
                FlyoutBase,
                FlyoutBaseClosingEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Closing)(
                    ::windows_core::Interface::as_raw(this),
                    handler.into_param().abi(),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn RemoveClosing(
        &self,
        token: super::super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .RemoveClosing)(::windows_core::Interface::as_raw(this), token)
                .ok()
        }
    }
    pub fn OverlayInputPassThroughElement(
        &self,
    ) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .OverlayInputPassThroughElement)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetOverlayInputPassThroughElement<P0>(
        &self,
        value: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetOverlayInputPassThroughElement)(
                    ::windows_core::Interface::as_raw(this),
                    value.try_into_param()?.abi(),
                )
                .ok()
        }
    }
    pub fn TryInvokeKeyboardAccelerator<P0>(
        &self,
        args: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<
            super::super::Input::ProcessKeyboardAcceleratorEventArgs,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .TryInvokeKeyboardAccelerator)(
                    ::windows_core::Interface::as_raw(this),
                    args.into_param().abi(),
                )
                .ok()
        }
    }
    pub fn ShowMode(&self) -> ::windows_core::Result<FlyoutShowMode> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ShowMode)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetShowMode)(::windows_core::Interface::as_raw(this), value)
                .ok()
        }
    }
    pub fn InputDevicePrefersPrimaryCommands(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .InputDevicePrefersPrimaryCommands)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn AreOpenCloseAnimationsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AreOpenCloseAnimationsEnabled)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetAreOpenCloseAnimationsEnabled(
        &self,
        value: bool,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetAreOpenCloseAnimationsEnabled)(
                    ::windows_core::Interface::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn IsOpen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .IsOpen)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn ShowAt2<P0, P1>(
        &self,
        placementtarget: P0,
        showoptions: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::DependencyObject>,
        P1: ::windows_core::TryIntoParam<FlyoutShowOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .ShowAt)(
                    ::windows_core::Interface::as_raw(this),
                    placementtarget.try_into_param()?.abi(),
                    showoptions.try_into_param()?.abi(),
                )
                .ok()
        }
    }
    pub fn ShouldConstrainToRootBounds(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ShouldConstrainToRootBounds)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn SetShouldConstrainToRootBounds(
        &self,
        value: bool,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetShouldConstrainToRootBounds)(
                    ::windows_core::Interface::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn IsConstrainedToRootBounds(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .IsConstrainedToRootBounds)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .XamlRoot)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetXamlRoot)(
                    ::windows_core::Interface::as_raw(this),
                    value.into_param().abi(),
                )
                .ok()
        }
    }
    pub fn CreatePresenter(&self) -> ::windows_core::Result<super::Control> {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBaseOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .CreatePresenter)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn OnProcessKeyboardAccelerators<P0>(
        &self,
        args: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<
            super::super::Input::ProcessKeyboardAcceleratorEventArgs,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IFlyoutBaseOverrides4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .OnProcessKeyboardAccelerators)(
                    ::windows_core::Interface::as_raw(this),
                    args.into_param().abi(),
                )
                .ok()
        }
    }
    pub fn PlacementProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .PlacementProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn AttachedFlyoutProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AttachedFlyoutProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn GetAttachedFlyout<P0>(element: P0) -> ::windows_core::Result<FlyoutBase>
    where
        P0: ::windows_core::TryIntoParam<super::super::FrameworkElement>,
    {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .GetAttachedFlyout)(
                    ::windows_core::Interface::as_raw(this),
                    element.try_into_param()?.abi(),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn SetAttachedFlyout<P0, P1>(
        element: P0,
        value: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::FrameworkElement>,
        P1: ::windows_core::TryIntoParam<FlyoutBase>,
    {
        Self::IFlyoutBaseStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this)
                .SetAttachedFlyout)(
                    ::windows_core::Interface::as_raw(this),
                    element.try_into_param()?.abi(),
                    value.try_into_param()?.abi(),
                )
                .ok()
        })
    }
    pub fn ShowAttachedFlyout<P0>(flyoutowner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::FrameworkElement>,
    {
        Self::IFlyoutBaseStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this)
                .ShowAttachedFlyout)(
                    ::windows_core::Interface::as_raw(this),
                    flyoutowner.try_into_param()?.abi(),
                )
                .ok()
        })
    }
    pub fn AllowFocusOnInteractionProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AllowFocusOnInteractionProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn LightDismissOverlayModeProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .LightDismissOverlayModeProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn AllowFocusWhenDisabledProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AllowFocusWhenDisabledProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn ElementSoundModeProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ElementSoundModeProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn OverlayInputPassThroughElementProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .OverlayInputPassThroughElementProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn TargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .TargetProperty)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        })
    }
    pub fn ShowModeProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ShowModeProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn InputDevicePrefersPrimaryCommandsProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .InputDevicePrefersPrimaryCommandsProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn AreOpenCloseAnimationsEnabledProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .AreOpenCloseAnimationsEnabledProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    pub fn IsOpenProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .IsOpenProperty)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        })
    }
    pub fn ShouldConstrainToRootBoundsProperty() -> ::windows_core::Result<
        super::super::DependencyProperty,
    > {
        Self::IFlyoutBaseStatics6(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ShouldConstrainToRootBoundsProperty)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFlyoutBaseStatics<
        R,
        F: FnOnce(&IFlyoutBaseStatics) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FlyoutBase,
            IFlyoutBaseStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFlyoutBaseStatics2<
        R,
        F: FnOnce(&IFlyoutBaseStatics2) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FlyoutBase,
            IFlyoutBaseStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFlyoutBaseStatics3<
        R,
        F: FnOnce(&IFlyoutBaseStatics3) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FlyoutBase,
            IFlyoutBaseStatics3,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFlyoutBaseStatics5<
        R,
        F: FnOnce(&IFlyoutBaseStatics5) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FlyoutBase,
            IFlyoutBaseStatics5,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFlyoutBaseStatics6<
        R,
        F: FnOnce(&IFlyoutBaseStatics6) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FlyoutBase,
            IFlyoutBaseStatics6,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FlyoutBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutBase;{723eea0b-d12e-430d-a9f0-9bb32bbf9913})",
    );
}
unsafe impl ::windows_core::Interface for FlyoutBase {
    type Vtable = IFlyoutBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FlyoutBase {
    const IID: ::windows_core::GUID = <IFlyoutBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutBase";
}
::windows_core::imp::interface_hierarchy!(
    FlyoutBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject> for FlyoutBase {}
unsafe impl ::core::marker::Send for FlyoutBase {}
unsafe impl ::core::marker::Sync for FlyoutBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FlyoutBaseClosingEventArgs(::windows_core::IUnknown);
impl FlyoutBaseClosingEventArgs {}
impl ::windows_core::RuntimeType for FlyoutBaseClosingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutBaseClosingEventArgs;{d075852d-b09a-4fd1-b005-db2ba01206fb})",
    );
}
unsafe impl ::windows_core::Interface for FlyoutBaseClosingEventArgs {
    type Vtable = IFlyoutBaseClosingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FlyoutBaseClosingEventArgs {
    const IID: ::windows_core::GUID = <IFlyoutBaseClosingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FlyoutBaseClosingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutBaseClosingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    FlyoutBaseClosingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FlyoutBaseClosingEventArgs {}
unsafe impl ::core::marker::Sync for FlyoutBaseClosingEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FlyoutShowOptions(::windows_core::IUnknown);
impl FlyoutShowOptions {
    pub fn Position(
        &self,
    ) -> ::windows_core::Result<
        super::super::super::super::Foundation::IReference<
            super::super::super::super::Foundation::Point,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Position)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetPosition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::super::super::Foundation::IReference<
                super::super::super::super::Foundation::Point,
            >,
        >,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetPosition)(
                    ::windows_core::Interface::as_raw(this),
                    value.try_into_param()?.abi(),
                )
                .ok()
        }
    }
    pub fn ExclusionRect(
        &self,
    ) -> ::windows_core::Result<
        super::super::super::super::Foundation::IReference<
            super::super::super::super::Foundation::Rect,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ExclusionRect)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetExclusionRect<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::super::super::Foundation::IReference<
                super::super::super::super::Foundation::Rect,
            >,
        >,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetExclusionRect)(
                    ::windows_core::Interface::as_raw(this),
                    value.try_into_param()?.abi(),
                )
                .ok()
        }
    }
    pub fn ShowMode(&self) -> ::windows_core::Result<FlyoutShowMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .ShowMode)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetShowMode)(::windows_core::Interface::as_raw(this), value)
                .ok()
        }
    }
    pub fn Placement(&self) -> ::windows_core::Result<FlyoutPlacementMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Placement)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn SetPlacement(
        &self,
        value: FlyoutPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetPlacement)(::windows_core::Interface::as_raw(this), value)
                .ok()
        }
    }
    pub fn new() -> ::windows_core::Result<FlyoutShowOptions> {
        Self::IFlyoutShowOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .CreateInstance)(
                    ::windows_core::Interface::as_raw(this),
                    ::core::ptr::null_mut(),
                    &mut ::core::option::Option::<::windows::core::IInspectable>::None
                        as *mut _ as _,
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFlyoutShowOptionsFactory<
        R,
        F: FnOnce(&IFlyoutShowOptionsFactory) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FlyoutShowOptions,
            IFlyoutShowOptionsFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FlyoutShowOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutShowOptions;{57d693ad-0c74-54dd-b110-1ee43fabadd9})",
    );
}
unsafe impl ::windows_core::Interface for FlyoutShowOptions {
    type Vtable = IFlyoutShowOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FlyoutShowOptions {
    const IID: ::windows_core::GUID = <IFlyoutShowOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FlyoutShowOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutShowOptions";
}
::windows_core::imp::interface_hierarchy!(
    FlyoutShowOptions, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FlyoutShowOptions {}
unsafe impl ::core::marker::Sync for FlyoutShowOptions {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct GeneratorPositionHelper(::windows_core::IUnknown);
impl GeneratorPositionHelper {}
impl ::windows_core::RuntimeType for GeneratorPositionHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.GeneratorPositionHelper;{cd40318d-7745-40d9-ab9d-abbda4a7ffea})",
    );
}
unsafe impl ::windows_core::Interface for GeneratorPositionHelper {
    type Vtable = IGeneratorPositionHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeneratorPositionHelper {
    const IID: ::windows_core::GUID = <IGeneratorPositionHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeneratorPositionHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GeneratorPositionHelper";
}
::windows_core::imp::interface_hierarchy!(
    GeneratorPositionHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for GeneratorPositionHelper {}
unsafe impl ::core::marker::Sync for GeneratorPositionHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct GridViewItemPresenter(::windows_core::IUnknown);
impl GridViewItemPresenter {}
impl ::windows_core::RuntimeType for GridViewItemPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.GridViewItemPresenter;{214f9010-56e2-4821-8a1c-2305709af94b})",
    );
}
unsafe impl ::windows_core::Interface for GridViewItemPresenter {
    type Vtable = IGridViewItemPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GridViewItemPresenter {
    const IID: ::windows_core::GUID = <IGridViewItemPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GridViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GridViewItemPresenter";
}
::windows_core::imp::interface_hierarchy!(
    GridViewItemPresenter, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentPresenter> for GridViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement>
for GridViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::UIElement> for GridViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for GridViewItemPresenter {}
unsafe impl ::core::marker::Send for GridViewItemPresenter {}
unsafe impl ::core::marker::Sync for GridViewItemPresenter {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct GridViewItemTemplateSettings(::windows_core::IUnknown);
impl GridViewItemTemplateSettings {}
impl ::windows_core::RuntimeType for GridViewItemTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.GridViewItemTemplateSettings;{9e30baaf-165d-4267-a45e-1a43a75706ac})",
    );
}
unsafe impl ::windows_core::Interface for GridViewItemTemplateSettings {
    type Vtable = IGridViewItemTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GridViewItemTemplateSettings {
    const IID: ::windows_core::GUID = <IGridViewItemTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GridViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GridViewItemTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    GridViewItemTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for GridViewItemTemplateSettings {}
unsafe impl ::core::marker::Send for GridViewItemTemplateSettings {}
unsafe impl ::core::marker::Sync for GridViewItemTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ItemsChangedEventArgs(::windows_core::IUnknown);
impl ItemsChangedEventArgs {}
impl ::windows_core::RuntimeType for ItemsChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ItemsChangedEventArgs;{e8b45568-7d10-421e-be29-81839a91de20})",
    );
}
unsafe impl ::windows_core::Interface for ItemsChangedEventArgs {
    type Vtable = IItemsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ItemsChangedEventArgs {
    const IID: ::windows_core::GUID = <IItemsChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ItemsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ItemsChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ItemsChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ItemsChangedEventArgs {}
unsafe impl ::core::marker::Sync for ItemsChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct JumpListItemBackgroundConverter(::windows_core::IUnknown);
impl JumpListItemBackgroundConverter {}
impl ::windows_core::RuntimeType for JumpListItemBackgroundConverter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.JumpListItemBackgroundConverter;{81177858-d224-410c-b16c-c5b6bb6188b2})",
    );
}
unsafe impl ::windows_core::Interface for JumpListItemBackgroundConverter {
    type Vtable = IJumpListItemBackgroundConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for JumpListItemBackgroundConverter {
    const IID: ::windows_core::GUID = <IJumpListItemBackgroundConverter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for JumpListItemBackgroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.JumpListItemBackgroundConverter";
}
::windows_core::imp::interface_hierarchy!(
    JumpListItemBackgroundConverter, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for JumpListItemBackgroundConverter {}
unsafe impl ::core::marker::Send for JumpListItemBackgroundConverter {}
unsafe impl ::core::marker::Sync for JumpListItemBackgroundConverter {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct JumpListItemForegroundConverter(::windows_core::IUnknown);
impl JumpListItemForegroundConverter {}
impl ::windows_core::RuntimeType for JumpListItemForegroundConverter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.JumpListItemForegroundConverter;{1590ed38-c504-4796-a63a-5bfc9eefaae8})",
    );
}
unsafe impl ::windows_core::Interface for JumpListItemForegroundConverter {
    type Vtable = IJumpListItemForegroundConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for JumpListItemForegroundConverter {
    const IID: ::windows_core::GUID = <IJumpListItemForegroundConverter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for JumpListItemForegroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.JumpListItemForegroundConverter";
}
::windows_core::imp::interface_hierarchy!(
    JumpListItemForegroundConverter, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for JumpListItemForegroundConverter {}
unsafe impl ::core::marker::Send for JumpListItemForegroundConverter {}
unsafe impl ::core::marker::Sync for JumpListItemForegroundConverter {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct LayoutInformation(::windows_core::IUnknown);
impl LayoutInformation {}
impl ::windows_core::RuntimeType for LayoutInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.LayoutInformation;{b5384c9b-c8cf-41b3-bf16-18c8420e72c9})",
    );
}
unsafe impl ::windows_core::Interface for LayoutInformation {
    type Vtable = ILayoutInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LayoutInformation {
    const IID: ::windows_core::GUID = <ILayoutInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LayoutInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LayoutInformation";
}
::windows_core::imp::interface_hierarchy!(
    LayoutInformation, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for LayoutInformation {}
unsafe impl ::core::marker::Sync for LayoutInformation {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ListViewItemPresenter(::windows_core::IUnknown);
impl ListViewItemPresenter {}
impl ::windows_core::RuntimeType for ListViewItemPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenter;{fc8946bd-a3a2-4969-8174-25b5d3c28033})",
    );
}
unsafe impl ::windows_core::Interface for ListViewItemPresenter {
    type Vtable = IListViewItemPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ListViewItemPresenter {
    const IID: ::windows_core::GUID = <IListViewItemPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ListViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenter";
}
::windows_core::imp::interface_hierarchy!(
    ListViewItemPresenter, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentPresenter> for ListViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement>
for ListViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::UIElement> for ListViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ListViewItemPresenter {}
unsafe impl ::core::marker::Send for ListViewItemPresenter {}
unsafe impl ::core::marker::Sync for ListViewItemPresenter {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ListViewItemTemplateSettings(::windows_core::IUnknown);
impl ListViewItemTemplateSettings {}
impl ::windows_core::RuntimeType for ListViewItemTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ListViewItemTemplateSettings;{67af84bf-8279-4686-9326-cd189f27575d})",
    );
}
unsafe impl ::windows_core::Interface for ListViewItemTemplateSettings {
    type Vtable = IListViewItemTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ListViewItemTemplateSettings {
    const IID: ::windows_core::GUID = <IListViewItemTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ListViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ListViewItemTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    ListViewItemTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ListViewItemTemplateSettings {}
unsafe impl ::core::marker::Send for ListViewItemTemplateSettings {}
unsafe impl ::core::marker::Sync for ListViewItemTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct LoopingSelector(::windows_core::IUnknown);
impl LoopingSelector {}
impl ::windows_core::RuntimeType for LoopingSelector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelector;{4c9a3e04-4827-49d9-8806-093957b0fd21})",
    );
}
unsafe impl ::windows_core::Interface for LoopingSelector {
    type Vtable = ILoopingSelector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoopingSelector {
    const IID: ::windows_core::GUID = <ILoopingSelector as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoopingSelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelector";
}
::windows_core::imp::interface_hierarchy!(
    LoopingSelector, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Control> for LoopingSelector {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for LoopingSelector {}
impl ::windows_core::CanTryInto<super::super::UIElement> for LoopingSelector {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for LoopingSelector {}
unsafe impl ::core::marker::Send for LoopingSelector {}
unsafe impl ::core::marker::Sync for LoopingSelector {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct LoopingSelectorItem(::windows_core::IUnknown);
impl LoopingSelectorItem {}
impl ::windows_core::RuntimeType for LoopingSelectorItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelectorItem;{c69714b9-27c6-4433-9d7c-0dbfb2f4344f})",
    );
}
unsafe impl ::windows_core::Interface for LoopingSelectorItem {
    type Vtable = ILoopingSelectorItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoopingSelectorItem {
    const IID: ::windows_core::GUID = <ILoopingSelectorItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoopingSelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelectorItem";
}
::windows_core::imp::interface_hierarchy!(
    LoopingSelectorItem, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentControl> for LoopingSelectorItem {}
impl ::windows_core::CanTryInto<super::Control> for LoopingSelectorItem {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for LoopingSelectorItem {}
impl ::windows_core::CanTryInto<super::super::UIElement> for LoopingSelectorItem {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for LoopingSelectorItem {}
unsafe impl ::core::marker::Send for LoopingSelectorItem {}
unsafe impl ::core::marker::Sync for LoopingSelectorItem {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct LoopingSelectorPanel(::windows_core::IUnknown);
impl LoopingSelectorPanel {}
impl ::windows_core::RuntimeType for LoopingSelectorPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelectorPanel;{40a9ba70-1011-4778-87f7-6bfd20d6377d})",
    );
}
unsafe impl ::windows_core::Interface for LoopingSelectorPanel {
    type Vtable = ILoopingSelectorPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoopingSelectorPanel {
    const IID: ::windows_core::GUID = <ILoopingSelectorPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoopingSelectorPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelectorPanel";
}
::windows_core::imp::interface_hierarchy!(
    LoopingSelectorPanel, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IScrollSnapPointsInfo> for LoopingSelectorPanel {}
impl ::windows_core::CanTryInto<super::Canvas> for LoopingSelectorPanel {}
impl ::windows_core::CanTryInto<super::Panel> for LoopingSelectorPanel {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement>
for LoopingSelectorPanel {}
impl ::windows_core::CanTryInto<super::super::UIElement> for LoopingSelectorPanel {}
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for LoopingSelectorPanel {}
unsafe impl ::core::marker::Send for LoopingSelectorPanel {}
unsafe impl ::core::marker::Sync for LoopingSelectorPanel {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct MenuFlyoutItemTemplateSettings(::windows_core::IUnknown);
impl MenuFlyoutItemTemplateSettings {}
impl ::windows_core::RuntimeType for MenuFlyoutItemTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.MenuFlyoutItemTemplateSettings;{56ad1809-3a16-4147-81cb-d0b35c834e0f})",
    );
}
unsafe impl ::windows_core::Interface for MenuFlyoutItemTemplateSettings {
    type Vtable = IMenuFlyoutItemTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MenuFlyoutItemTemplateSettings {
    const IID: ::windows_core::GUID = <IMenuFlyoutItemTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MenuFlyoutItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.MenuFlyoutItemTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    MenuFlyoutItemTemplateSettings, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for MenuFlyoutItemTemplateSettings {}
unsafe impl ::core::marker::Send for MenuFlyoutItemTemplateSettings {}
unsafe impl ::core::marker::Sync for MenuFlyoutItemTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct MenuFlyoutPresenterTemplateSettings(::windows_core::IUnknown);
impl MenuFlyoutPresenterTemplateSettings {}
impl ::windows_core::RuntimeType for MenuFlyoutPresenterTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.MenuFlyoutPresenterTemplateSettings;{d68fd00d-629d-4349-ac51-b877c80983b8})",
    );
}
unsafe impl ::windows_core::Interface for MenuFlyoutPresenterTemplateSettings {
    type Vtable = IMenuFlyoutPresenterTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MenuFlyoutPresenterTemplateSettings {
    const IID: ::windows_core::GUID = <IMenuFlyoutPresenterTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MenuFlyoutPresenterTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.MenuFlyoutPresenterTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    MenuFlyoutPresenterTemplateSettings, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for MenuFlyoutPresenterTemplateSettings {}
unsafe impl ::core::marker::Send for MenuFlyoutPresenterTemplateSettings {}
unsafe impl ::core::marker::Sync for MenuFlyoutPresenterTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct NavigationViewItemPresenter(::windows_core::IUnknown);
impl NavigationViewItemPresenter {}
impl ::windows_core::RuntimeType for NavigationViewItemPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter;{9956d3fc-4693-59cb-b6bf-37249058be96})",
    );
}
unsafe impl ::windows_core::Interface for NavigationViewItemPresenter {
    type Vtable = INavigationViewItemPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigationViewItemPresenter {
    const IID: ::windows_core::GUID = <INavigationViewItemPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NavigationViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter";
}
::windows_core::imp::interface_hierarchy!(
    NavigationViewItemPresenter, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentControl> for NavigationViewItemPresenter {}
impl ::windows_core::CanTryInto<super::Control> for NavigationViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement>
for NavigationViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::UIElement>
for NavigationViewItemPresenter {}
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for NavigationViewItemPresenter {}
unsafe impl ::core::marker::Send for NavigationViewItemPresenter {}
unsafe impl ::core::marker::Sync for NavigationViewItemPresenter {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct OrientedVirtualizingPanel(::windows_core::IUnknown);
impl OrientedVirtualizingPanel {}
impl ::windows_core::RuntimeType for OrientedVirtualizingPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.OrientedVirtualizingPanel;{f077b577-39bd-46ee-bdd7-0826beed71b8})",
    );
}
unsafe impl ::windows_core::Interface for OrientedVirtualizingPanel {
    type Vtable = IOrientedVirtualizingPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OrientedVirtualizingPanel {
    const IID: ::windows_core::GUID = <IOrientedVirtualizingPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OrientedVirtualizingPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.OrientedVirtualizingPanel";
}
::windows_core::imp::interface_hierarchy!(
    OrientedVirtualizingPanel, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IInsertionPanel> for OrientedVirtualizingPanel {}
impl ::windows_core::CanTryInto<IScrollSnapPointsInfo> for OrientedVirtualizingPanel {}
impl ::windows_core::CanTryInto<super::VirtualizingPanel> for OrientedVirtualizingPanel {}
impl ::windows_core::CanTryInto<super::Panel> for OrientedVirtualizingPanel {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement>
for OrientedVirtualizingPanel {}
impl ::windows_core::CanTryInto<super::super::UIElement> for OrientedVirtualizingPanel {}
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for OrientedVirtualizingPanel {}
unsafe impl ::core::marker::Send for OrientedVirtualizingPanel {}
unsafe impl ::core::marker::Sync for OrientedVirtualizingPanel {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PickerFlyoutBase(::windows_core::IUnknown);
impl PickerFlyoutBase {}
impl ::windows_core::RuntimeType for PickerFlyoutBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.PickerFlyoutBase;{e33574ea-1076-44d1-9383-dc24ac5cff2a})",
    );
}
unsafe impl ::windows_core::Interface for PickerFlyoutBase {
    type Vtable = IPickerFlyoutBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PickerFlyoutBase {
    const IID: ::windows_core::GUID = <IPickerFlyoutBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PickerFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PickerFlyoutBase";
}
::windows_core::imp::interface_hierarchy!(
    PickerFlyoutBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<FlyoutBase> for PickerFlyoutBase {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for PickerFlyoutBase {}
unsafe impl ::core::marker::Send for PickerFlyoutBase {}
unsafe impl ::core::marker::Sync for PickerFlyoutBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PivotHeaderItem(::windows_core::IUnknown);
impl PivotHeaderItem {}
impl ::windows_core::RuntimeType for PivotHeaderItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.PivotHeaderItem;{594572c2-82aa-410b-9e55-fd8e2c98862d})",
    );
}
unsafe impl ::windows_core::Interface for PivotHeaderItem {
    type Vtable = IPivotHeaderItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PivotHeaderItem {
    const IID: ::windows_core::GUID = <IPivotHeaderItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PivotHeaderItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotHeaderItem";
}
::windows_core::imp::interface_hierarchy!(
    PivotHeaderItem, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentControl> for PivotHeaderItem {}
impl ::windows_core::CanTryInto<super::Control> for PivotHeaderItem {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for PivotHeaderItem {}
impl ::windows_core::CanTryInto<super::super::UIElement> for PivotHeaderItem {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for PivotHeaderItem {}
unsafe impl ::core::marker::Send for PivotHeaderItem {}
unsafe impl ::core::marker::Sync for PivotHeaderItem {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PivotHeaderPanel(::windows_core::IUnknown);
impl PivotHeaderPanel {}
impl ::windows_core::RuntimeType for PivotHeaderPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.PivotHeaderPanel;{21484ebc-9241-4203-bd37-6c08fb096612})",
    );
}
unsafe impl ::windows_core::Interface for PivotHeaderPanel {
    type Vtable = IPivotHeaderPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PivotHeaderPanel {
    const IID: ::windows_core::GUID = <IPivotHeaderPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PivotHeaderPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotHeaderPanel";
}
::windows_core::imp::interface_hierarchy!(
    PivotHeaderPanel, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Canvas> for PivotHeaderPanel {}
impl ::windows_core::CanTryInto<super::Panel> for PivotHeaderPanel {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for PivotHeaderPanel {}
impl ::windows_core::CanTryInto<super::super::UIElement> for PivotHeaderPanel {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for PivotHeaderPanel {}
unsafe impl ::core::marker::Send for PivotHeaderPanel {}
unsafe impl ::core::marker::Sync for PivotHeaderPanel {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PivotPanel(::windows_core::IUnknown);
impl PivotPanel {}
impl ::windows_core::RuntimeType for PivotPanel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.PivotPanel;{ad4ebe80-22a9-4ca3-9212-2773b6359ff3})",
    );
}
unsafe impl ::windows_core::Interface for PivotPanel {
    type Vtable = IPivotPanel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PivotPanel {
    const IID: ::windows_core::GUID = <IPivotPanel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PivotPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotPanel";
}
::windows_core::imp::interface_hierarchy!(
    PivotPanel, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IScrollSnapPointsInfo> for PivotPanel {}
impl ::windows_core::CanTryInto<super::Panel> for PivotPanel {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for PivotPanel {}
impl ::windows_core::CanTryInto<super::super::UIElement> for PivotPanel {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for PivotPanel {}
unsafe impl ::core::marker::Send for PivotPanel {}
unsafe impl ::core::marker::Sync for PivotPanel {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Popup(::windows_core::IUnknown);
impl Popup {}
impl ::windows_core::RuntimeType for Popup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.Popup;{62418240-e6d3-4705-a1dc-39156456ee29})",
    );
}
unsafe impl ::windows_core::Interface for Popup {
    type Vtable = IPopup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Popup {
    const IID: ::windows_core::GUID = <IPopup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Popup {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Popup";
}
::windows_core::imp::interface_hierarchy!(
    Popup, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for Popup {}
impl ::windows_core::CanTryInto<super::super::UIElement> for Popup {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for Popup {}
unsafe impl ::core::marker::Send for Popup {}
unsafe impl ::core::marker::Sync for Popup {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ProgressBarTemplateSettings(::windows_core::IUnknown);
impl ProgressBarTemplateSettings {}
impl ::windows_core::RuntimeType for ProgressBarTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ProgressBarTemplateSettings;{3fe2ea2a-e3f2-4c2b-9488-918d77d2bbe4})",
    );
}
unsafe impl ::windows_core::Interface for ProgressBarTemplateSettings {
    type Vtable = IProgressBarTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProgressBarTemplateSettings {
    const IID: ::windows_core::GUID = <IProgressBarTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProgressBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ProgressBarTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    ProgressBarTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ProgressBarTemplateSettings {}
unsafe impl ::core::marker::Send for ProgressBarTemplateSettings {}
unsafe impl ::core::marker::Sync for ProgressBarTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ProgressRingTemplateSettings(::windows_core::IUnknown);
impl ProgressRingTemplateSettings {}
impl ::windows_core::RuntimeType for ProgressRingTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ProgressRingTemplateSettings;{b9b675ec-c723-42e6-83e9-9826272bdc0e})",
    );
}
unsafe impl ::windows_core::Interface for ProgressRingTemplateSettings {
    type Vtable = IProgressRingTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProgressRingTemplateSettings {
    const IID: ::windows_core::GUID = <IProgressRingTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProgressRingTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ProgressRingTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    ProgressRingTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ProgressRingTemplateSettings {}
unsafe impl ::core::marker::Send for ProgressRingTemplateSettings {}
unsafe impl ::core::marker::Sync for ProgressRingTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RangeBase(::windows_core::IUnknown);
impl RangeBase {}
impl ::windows_core::RuntimeType for RangeBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.RangeBase;{fa002c1a-494e-46cf-91d4-e14a8d798675})",
    );
}
unsafe impl ::windows_core::Interface for RangeBase {
    type Vtable = IRangeBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RangeBase {
    const IID: ::windows_core::GUID = <IRangeBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBase";
}
::windows_core::imp::interface_hierarchy!(
    RangeBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Control> for RangeBase {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for RangeBase {}
impl ::windows_core::CanTryInto<super::super::UIElement> for RangeBase {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for RangeBase {}
unsafe impl ::core::marker::Send for RangeBase {}
unsafe impl ::core::marker::Sync for RangeBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RangeBaseValueChangedEventArgs(::windows_core::IUnknown);
impl RangeBaseValueChangedEventArgs {
    pub fn OldValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .OldValue)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn NewValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .NewValue)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
    pub fn OriginalSource(
        &self,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<
            super::super::IRoutedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .OriginalSource)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RangeBaseValueChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs;{a1921777-d5c1-4f9c-a7b0-0401b7e6dc5c})",
    );
}
unsafe impl ::windows_core::Interface for RangeBaseValueChangedEventArgs {
    type Vtable = IRangeBaseValueChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RangeBaseValueChangedEventArgs {
    const IID: ::windows_core::GUID = <IRangeBaseValueChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RangeBaseValueChangedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::RoutedEventArgs>
for RangeBaseValueChangedEventArgs {}
unsafe impl ::core::marker::Send for RangeBaseValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for RangeBaseValueChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RepeatButton(::windows_core::IUnknown);
impl RepeatButton {}
impl ::windows_core::RuntimeType for RepeatButton {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.RepeatButton;{02200df9-021a-484a-a93b-0f31020314e5})",
    );
}
unsafe impl ::windows_core::Interface for RepeatButton {
    type Vtable = IRepeatButton_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RepeatButton {
    const IID: ::windows_core::GUID = <IRepeatButton as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RepeatButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RepeatButton";
}
::windows_core::imp::interface_hierarchy!(
    RepeatButton, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ButtonBase> for RepeatButton {}
impl ::windows_core::CanTryInto<super::ContentControl> for RepeatButton {}
impl ::windows_core::CanTryInto<super::Control> for RepeatButton {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for RepeatButton {}
impl ::windows_core::CanTryInto<super::super::UIElement> for RepeatButton {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for RepeatButton {}
unsafe impl ::core::marker::Send for RepeatButton {}
unsafe impl ::core::marker::Sync for RepeatButton {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ScrollBar(::windows_core::IUnknown);
impl ScrollBar {}
impl ::windows_core::RuntimeType for ScrollBar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ScrollBar;{f57ae6ca-d1a6-4b90-a4e9-54df1ba8d2ec})",
    );
}
unsafe impl ::windows_core::Interface for ScrollBar {
    type Vtable = IScrollBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScrollBar {
    const IID: ::windows_core::GUID = <IScrollBar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScrollBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ScrollBar";
}
::windows_core::imp::interface_hierarchy!(
    ScrollBar, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RangeBase> for ScrollBar {}
impl ::windows_core::CanTryInto<super::Control> for ScrollBar {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for ScrollBar {}
impl ::windows_core::CanTryInto<super::super::UIElement> for ScrollBar {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for ScrollBar {}
unsafe impl ::core::marker::Send for ScrollBar {}
unsafe impl ::core::marker::Sync for ScrollBar {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ScrollEventArgs(::windows_core::IUnknown);
impl ScrollEventArgs {}
impl ::windows_core::RuntimeType for ScrollEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ScrollEventArgs;{c57e5168-3afe-448d-b752-2f364c75d743})",
    );
}
unsafe impl ::windows_core::Interface for ScrollEventArgs {
    type Vtable = IScrollEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScrollEventArgs {
    const IID: ::windows_core::GUID = <IScrollEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScrollEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ScrollEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ScrollEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::RoutedEventArgs> for ScrollEventArgs {}
unsafe impl ::core::marker::Send for ScrollEventArgs {}
unsafe impl ::core::marker::Sync for ScrollEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Selector(::windows_core::IUnknown);
impl Selector {}
impl ::windows_core::RuntimeType for Selector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.Selector;{e30eb3a5-b36b-42dc-8527-cd25136c083c})",
    );
}
unsafe impl ::windows_core::Interface for Selector {
    type Vtable = ISelector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Selector {
    const IID: ::windows_core::GUID = <ISelector as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Selector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Selector";
}
::windows_core::imp::interface_hierarchy!(
    Selector, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IItemContainerMapping> for Selector {}
impl ::windows_core::CanTryInto<super::ItemsControl> for Selector {}
impl ::windows_core::CanTryInto<super::Control> for Selector {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for Selector {}
impl ::windows_core::CanTryInto<super::super::UIElement> for Selector {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for Selector {}
unsafe impl ::core::marker::Send for Selector {}
unsafe impl ::core::marker::Sync for Selector {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SelectorItem(::windows_core::IUnknown);
impl SelectorItem {}
impl ::windows_core::RuntimeType for SelectorItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.SelectorItem;{541c8d6c-0283-4581-b945-2a64c28a0646})",
    );
}
unsafe impl ::windows_core::Interface for SelectorItem {
    type Vtable = ISelectorItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SelectorItem {
    const IID: ::windows_core::GUID = <ISelectorItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SelectorItem";
}
::windows_core::imp::interface_hierarchy!(
    SelectorItem, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ContentControl> for SelectorItem {}
impl ::windows_core::CanTryInto<super::Control> for SelectorItem {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for SelectorItem {}
impl ::windows_core::CanTryInto<super::super::UIElement> for SelectorItem {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for SelectorItem {}
unsafe impl ::core::marker::Send for SelectorItem {}
unsafe impl ::core::marker::Sync for SelectorItem {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SettingsFlyoutTemplateSettings(::windows_core::IUnknown);
impl SettingsFlyoutTemplateSettings {}
impl ::windows_core::RuntimeType for SettingsFlyoutTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.SettingsFlyoutTemplateSettings;{bcf14c10-cea7-43f1-9d68-57605ded69d4})",
    );
}
unsafe impl ::windows_core::Interface for SettingsFlyoutTemplateSettings {
    type Vtable = ISettingsFlyoutTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SettingsFlyoutTemplateSettings {
    const IID: ::windows_core::GUID = <ISettingsFlyoutTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SettingsFlyoutTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SettingsFlyoutTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    SettingsFlyoutTemplateSettings, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for SettingsFlyoutTemplateSettings {}
unsafe impl ::core::marker::Send for SettingsFlyoutTemplateSettings {}
unsafe impl ::core::marker::Sync for SettingsFlyoutTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SplitViewTemplateSettings(::windows_core::IUnknown);
impl SplitViewTemplateSettings {}
impl ::windows_core::RuntimeType for SplitViewTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.SplitViewTemplateSettings;{c16ab5a7-4996-4443-b199-6b6b89124eab})",
    );
}
unsafe impl ::windows_core::Interface for SplitViewTemplateSettings {
    type Vtable = ISplitViewTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SplitViewTemplateSettings {
    const IID: ::windows_core::GUID = <ISplitViewTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SplitViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SplitViewTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    SplitViewTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for SplitViewTemplateSettings {}
unsafe impl ::core::marker::Send for SplitViewTemplateSettings {}
unsafe impl ::core::marker::Sync for SplitViewTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Thumb(::windows_core::IUnknown);
impl Thumb {}
impl ::windows_core::RuntimeType for Thumb {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.Thumb;{e8b2b281-0d6a-45cf-b333-2402b037f099})",
    );
}
unsafe impl ::windows_core::Interface for Thumb {
    type Vtable = IThumb_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Thumb {
    const IID: ::windows_core::GUID = <IThumb as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Thumb {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Thumb";
}
::windows_core::imp::interface_hierarchy!(
    Thumb, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::Control> for Thumb {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for Thumb {}
impl ::windows_core::CanTryInto<super::super::UIElement> for Thumb {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for Thumb {}
unsafe impl ::core::marker::Send for Thumb {}
unsafe impl ::core::marker::Sync for Thumb {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct TickBar(::windows_core::IUnknown);
impl TickBar {}
impl ::windows_core::RuntimeType for TickBar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.TickBar;{994683fa-f1f6-487d-a5ac-c15921bfa995})",
    );
}
unsafe impl ::windows_core::Interface for TickBar {
    type Vtable = ITickBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TickBar {
    const IID: ::windows_core::GUID = <ITickBar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TickBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.TickBar";
}
::windows_core::imp::interface_hierarchy!(
    TickBar, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for TickBar {}
impl ::windows_core::CanTryInto<super::super::UIElement> for TickBar {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for TickBar {}
unsafe impl ::core::marker::Send for TickBar {}
unsafe impl ::core::marker::Sync for TickBar {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ToggleButton(::windows_core::IUnknown);
impl ToggleButton {}
impl ::windows_core::RuntimeType for ToggleButton {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ToggleButton;{589877fb-0fc7-4036-9d8b-127dfa75c16d})",
    );
}
unsafe impl ::windows_core::Interface for ToggleButton {
    type Vtable = IToggleButton_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToggleButton {
    const IID: ::windows_core::GUID = <IToggleButton as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleButton";
}
::windows_core::imp::interface_hierarchy!(
    ToggleButton, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ButtonBase> for ToggleButton {}
impl ::windows_core::CanTryInto<super::ContentControl> for ToggleButton {}
impl ::windows_core::CanTryInto<super::Control> for ToggleButton {}
impl ::windows_core::CanTryInto<super::super::FrameworkElement> for ToggleButton {}
impl ::windows_core::CanTryInto<super::super::UIElement> for ToggleButton {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for ToggleButton {}
unsafe impl ::core::marker::Send for ToggleButton {}
unsafe impl ::core::marker::Sync for ToggleButton {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ToggleSwitchTemplateSettings(::windows_core::IUnknown);
impl ToggleSwitchTemplateSettings {}
impl ::windows_core::RuntimeType for ToggleSwitchTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ToggleSwitchTemplateSettings;{02b7bdcd-628a-4363-86e0-51d6e2e89e58})",
    );
}
unsafe impl ::windows_core::Interface for ToggleSwitchTemplateSettings {
    type Vtable = IToggleSwitchTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToggleSwitchTemplateSettings {
    const IID: ::windows_core::GUID = <IToggleSwitchTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToggleSwitchTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleSwitchTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    ToggleSwitchTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ToggleSwitchTemplateSettings {}
unsafe impl ::core::marker::Send for ToggleSwitchTemplateSettings {}
unsafe impl ::core::marker::Sync for ToggleSwitchTemplateSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ToolTipTemplateSettings(::windows_core::IUnknown);
impl ToolTipTemplateSettings {}
impl ::windows_core::RuntimeType for ToolTipTemplateSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Controls.Primitives.ToolTipTemplateSettings;{d4388247-0ec4-4506-affd-afac2225b48c})",
    );
}
unsafe impl ::windows_core::Interface for ToolTipTemplateSettings {
    type Vtable = IToolTipTemplateSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToolTipTemplateSettings {
    const IID: ::windows_core::GUID = <IToolTipTemplateSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToolTipTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToolTipTemplateSettings";
}
::windows_core::imp::interface_hierarchy!(
    ToolTipTemplateSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject>
for ToolTipTemplateSettings {}
unsafe impl ::core::marker::Send for ToolTipTemplateSettings {}
unsafe impl ::core::marker::Sync for ToolTipTemplateSettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationDirection {}
impl ::core::clone::Clone for AnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnimationDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AnimationDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AnimationDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AnimationDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.AnimationDirection;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ComponentResourceLocation(pub i32);
impl ComponentResourceLocation {
    pub const Application: Self = Self(0i32);
    pub const Nested: Self = Self(1i32);
}
impl ::core::marker::Copy for ComponentResourceLocation {}
impl ::core::clone::Clone for ComponentResourceLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ComponentResourceLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ComponentResourceLocation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ComponentResourceLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComponentResourceLocation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ComponentResourceLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.ComponentResourceLocation;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EdgeTransitionLocation(pub i32);
impl EdgeTransitionLocation {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for EdgeTransitionLocation {}
impl ::core::clone::Clone for EdgeTransitionLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EdgeTransitionLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EdgeTransitionLocation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EdgeTransitionLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeTransitionLocation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EdgeTransitionLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.EdgeTransitionLocation;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Full: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
    pub const Auto: Self = Self(13i32);
}
impl ::core::marker::Copy for FlyoutPlacementMode {}
impl ::core::clone::Clone for FlyoutPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlyoutPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FlyoutPlacementMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FlyoutPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutPlacementMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FlyoutPlacementMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutPlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FlyoutShowMode(pub i32);
impl FlyoutShowMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Transient: Self = Self(2i32);
    pub const TransientWithDismissOnPointerMoveAway: Self = Self(3i32);
}
impl ::core::marker::Copy for FlyoutShowMode {}
impl ::core::clone::Clone for FlyoutShowMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlyoutShowMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FlyoutShowMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FlyoutShowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutShowMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FlyoutShowMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutShowMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GeneratorDirection(pub i32);
impl GeneratorDirection {
    pub const Forward: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
}
impl ::core::marker::Copy for GeneratorDirection {}
impl ::core::clone::Clone for GeneratorDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeneratorDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GeneratorDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GeneratorDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneratorDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeneratorDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.GeneratorDirection;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GroupHeaderPlacement(pub i32);
impl GroupHeaderPlacement {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for GroupHeaderPlacement {}
impl ::core::clone::Clone for GroupHeaderPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GroupHeaderPlacement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GroupHeaderPlacement {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GroupHeaderPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GroupHeaderPlacement").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GroupHeaderPlacement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.GroupHeaderPlacement;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ListViewItemPresenterCheckMode(pub i32);
impl ListViewItemPresenterCheckMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterCheckMode {}
impl ::core::clone::Clone for ListViewItemPresenterCheckMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ListViewItemPresenterCheckMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ListViewItemPresenterCheckMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ListViewItemPresenterCheckMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListViewItemPresenterCheckMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ListViewItemPresenterCheckMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenterCheckMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ListViewItemPresenterSelectionIndicatorMode(pub i32);
impl ListViewItemPresenterSelectionIndicatorMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterSelectionIndicatorMode {}
impl ::core::clone::Clone for ListViewItemPresenterSelectionIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ListViewItemPresenterSelectionIndicatorMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ListViewItemPresenterSelectionIndicatorMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ListViewItemPresenterSelectionIndicatorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListViewItemPresenterSelectionIndicatorMode")
            .field(&self.0)
            .finish()
    }
}
impl ::windows_core::RuntimeType for ListViewItemPresenterSelectionIndicatorMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenterSelectionIndicatorMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlacementMode(pub i32);
impl PlacementMode {
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(9i32);
    pub const Mouse: Self = Self(7i32);
    pub const Right: Self = Self(4i32);
    pub const Top: Self = Self(10i32);
}
impl ::core::marker::Copy for PlacementMode {}
impl ::core::clone::Clone for PlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlacementMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlacementMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlacementMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.PlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PopupPlacementMode(pub i32);
impl PopupPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
}
impl ::core::marker::Copy for PopupPlacementMode {}
impl ::core::clone::Clone for PopupPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PopupPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PopupPlacementMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PopupPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupPlacementMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PopupPlacementMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.PopupPlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ScrollEventType(pub i32);
impl ScrollEventType {
    pub const SmallDecrement: Self = Self(0i32);
    pub const SmallIncrement: Self = Self(1i32);
    pub const LargeDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const ThumbPosition: Self = Self(4i32);
    pub const ThumbTrack: Self = Self(5i32);
    pub const First: Self = Self(6i32);
    pub const Last: Self = Self(7i32);
    pub const EndScroll: Self = Self(8i32);
}
impl ::core::marker::Copy for ScrollEventType {}
impl ::core::clone::Clone for ScrollEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ScrollEventType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ScrollEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollEventType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ScrollEventType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.ScrollEventType;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ScrollingIndicatorMode(pub i32);
impl ScrollingIndicatorMode {
    pub const None: Self = Self(0i32);
    pub const TouchIndicator: Self = Self(1i32);
    pub const MouseIndicator: Self = Self(2i32);
}
impl ::core::marker::Copy for ScrollingIndicatorMode {}
impl ::core::clone::Clone for ScrollingIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollingIndicatorMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ScrollingIndicatorMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ScrollingIndicatorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollingIndicatorMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ScrollingIndicatorMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.ScrollingIndicatorMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SliderSnapsTo(pub i32);
impl SliderSnapsTo {
    pub const StepValues: Self = Self(0i32);
    pub const Ticks: Self = Self(1i32);
}
impl ::core::marker::Copy for SliderSnapsTo {}
impl ::core::clone::Clone for SliderSnapsTo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SliderSnapsTo {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SliderSnapsTo {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SliderSnapsTo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SliderSnapsTo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SliderSnapsTo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.SliderSnapsTo;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SnapPointsAlignment(pub i32);
impl SnapPointsAlignment {
    pub const Near: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Far: Self = Self(2i32);
}
impl ::core::marker::Copy for SnapPointsAlignment {}
impl ::core::clone::Clone for SnapPointsAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SnapPointsAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SnapPointsAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SnapPointsAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapPointsAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SnapPointsAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.SnapPointsAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TickPlacement(pub i32);
impl TickPlacement {
    pub const None: Self = Self(0i32);
    pub const TopLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const Outside: Self = Self(3i32);
    pub const Inline: Self = Self(4i32);
}
impl ::core::marker::Copy for TickPlacement {}
impl ::core::clone::Clone for TickPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TickPlacement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TickPlacement {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TickPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TickPlacement").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TickPlacement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.TickPlacement;i4)",
    );
}
#[repr(C)]
pub struct GeneratorPosition {
    pub Index: i32,
    pub Offset: i32,
}
impl ::core::marker::Copy for GeneratorPosition {}
impl ::core::clone::Clone for GeneratorPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GeneratorPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GeneratorPosition")
            .field("Index", &self.Index)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::windows_core::TypeKind for GeneratorPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for GeneratorPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Controls.Primitives.GeneratorPosition;i4;i4)",
    );
}
impl ::core::cmp::PartialEq for GeneratorPosition {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for GeneratorPosition {}
impl ::core::default::Default for GeneratorPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragCompletedEventHandler(pub ::windows_core::IUnknown);
impl DragCompletedEventHandler {}
#[repr(C)]
struct DragCompletedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragCompletedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DragCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragCompletedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> DragCompletedEventHandlerBox<F> {
    const VTABLE: DragCompletedEventHandler_Vtbl = DragCompletedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid
            == <DragCompletedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid
                == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for DragCompletedEventHandler {
    type Vtable = DragCompletedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragCompletedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x36b28888_19ac_4b4e_9137_a6cf2b023883,
    );
}
impl ::windows_core::RuntimeType for DragCompletedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{36b28888-19ac-4b4e-9137-a6cf2b023883}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DragCompletedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragDeltaEventHandler(pub ::windows_core::IUnknown);
impl DragDeltaEventHandler {}
#[repr(C)]
struct DragDeltaEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragDeltaEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DragDeltaEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragDeltaEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> DragDeltaEventHandlerBox<F> {
    const VTABLE: DragDeltaEventHandler_Vtbl = DragDeltaEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid
            == <DragDeltaEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid
                == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for DragDeltaEventHandler {
    type Vtable = DragDeltaEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragDeltaEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4ac24f9f_ac28_49e9_9189_dccffeb66472,
    );
}
impl ::windows_core::RuntimeType for DragDeltaEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{4ac24f9f-ac28-49e9-9189-dccffeb66472}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DragDeltaEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragStartedEventHandler(pub ::windows_core::IUnknown);
impl DragStartedEventHandler {}
#[repr(C)]
struct DragStartedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragStartedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DragStartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragStartedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> DragStartedEventHandlerBox<F> {
    const VTABLE: DragStartedEventHandler_Vtbl = DragStartedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid
            == <DragStartedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid
                == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for DragStartedEventHandler {
    type Vtable = DragStartedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragStartedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd2eea48a_c65a_495d_a2f1_72c66989142d,
    );
}
impl ::windows_core::RuntimeType for DragStartedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{d2eea48a-c65a-495d-a2f1-72c66989142d}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DragStartedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ItemsChangedEventHandler(pub ::windows_core::IUnknown);
impl ItemsChangedEventHandler {}
#[repr(C)]
struct ItemsChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ItemsChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const ItemsChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ItemsChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> ItemsChangedEventHandlerBox<F> {
    const VTABLE: ItemsChangedEventHandler_Vtbl = ItemsChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid
            == <ItemsChangedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid
                == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for ItemsChangedEventHandler {
    type Vtable = ItemsChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ItemsChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x178257be_a304_482f_8bf0_b9d2e39612a3,
    );
}
impl ::windows_core::RuntimeType for ItemsChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{178257be-a304-482f-8bf0-b9d2e39612a3}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ItemsChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RangeBaseValueChangedEventHandler(pub ::windows_core::IUnknown);
impl RangeBaseValueChangedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&RangeBaseValueChangedEventArgs>,
            ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
    >(invoke: F) -> Self {
        let com = RangeBaseValueChangedEventHandlerBox::<F> {
            vtable: &RangeBaseValueChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<RangeBaseValueChangedEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .Invoke)(
                    ::windows_core::Interface::as_raw(this),
                    sender.into_param().abi(),
                    e.into_param().abi(),
                )
                .ok()
        }
    }
}
#[repr(C)]
struct RangeBaseValueChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RangeBaseValueChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const RangeBaseValueChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RangeBaseValueChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> RangeBaseValueChangedEventHandlerBox<F> {
    const VTABLE: RangeBaseValueChangedEventHandler_Vtbl = RangeBaseValueChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid
            == <RangeBaseValueChangedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid
                == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for RangeBaseValueChangedEventHandler {
    type Vtable = RangeBaseValueChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RangeBaseValueChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe3906fd9_4d1b_4ac8_a43c_c3b908742799,
    );
}
impl ::windows_core::RuntimeType for RangeBaseValueChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{e3906fd9-4d1b-4ac8-a43c-c3b908742799}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct RangeBaseValueChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ScrollEventHandler(pub ::windows_core::IUnknown);
impl ScrollEventHandler {}
#[repr(C)]
struct ScrollEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ScrollEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const ScrollEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ScrollEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> ScrollEventHandlerBox<F> {
    const VTABLE: ScrollEventHandler_Vtbl = ScrollEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ScrollEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid
                == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for ScrollEventHandler {
    type Vtable = ScrollEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScrollEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8860b0a4_a383_4c83_b306_a1c39d7db87f,
    );
}
impl ::windows_core::RuntimeType for ScrollEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{8860b0a4-a383-4c83-b306-a1c39d7db87f}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ScrollEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
