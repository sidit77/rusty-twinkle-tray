pub mod Controls;
pub mod Hosting;
pub mod Input;
pub mod Media;
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IAdaptiveTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveTrigger {
    type Vtable = IAdaptiveTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAdaptiveTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa5f04119_0cd9_49f1_a23f_44e547ab9f1a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinWindowWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMinWindowWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MinWindowHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMinWindowHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
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
pub struct IAdaptiveTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveTriggerFactory {
    type Vtable = IAdaptiveTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAdaptiveTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc966d482_5aeb_4841_9247_c1a0bdd6f59f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveTriggerFactory_Vtbl {
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
pub struct IAdaptiveTriggerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveTriggerStatics {
    type Vtable = IAdaptiveTriggerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAdaptiveTriggerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb92e29ea_1615_4350_9c3b_92b2986bf444,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveTriggerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinWindowWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinWindowHeightProperty: unsafe extern "system" fn(
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
pub struct IApplication(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplication {
    type Vtable = IApplication_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplication {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x74b861a1_7487_46a9_9a6e_c78b512726c5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Resources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetResources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DebugSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RequestedTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ApplicationTheme,
    ) -> ::windows_core::HRESULT,
    pub SetRequestedTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ApplicationTheme,
    ) -> ::windows_core::HRESULT,
    pub UnhandledException: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveUnhandledException: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    Suspending: usize,
    pub RemoveSuspending: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Resuming: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveResuming: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Exit: unsafe extern "system" fn(
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
pub struct IApplication2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplication2 {
    type Vtable = IApplication2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplication2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x019104be_522a_5904_f52f_de72010429e0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FocusVisualKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusVisualKind,
    ) -> ::windows_core::HRESULT,
    pub SetFocusVisualKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FocusVisualKind,
    ) -> ::windows_core::HRESULT,
    pub RequiresPointerMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ApplicationRequiresPointerMode,
    ) -> ::windows_core::HRESULT,
    pub SetRequiresPointerMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ApplicationRequiresPointerMode,
    ) -> ::windows_core::HRESULT,
    LeavingBackground: usize,
    pub RemoveLeavingBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    EnteredBackground: usize,
    pub RemoveEnteredBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IApplication3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplication3 {
    type Vtable = IApplication3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplication3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb775ad7c_18b8_45ca_a1b0_dc483e4b1028,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HighContrastAdjustment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ApplicationHighContrastAdjustment,
    ) -> ::windows_core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ApplicationHighContrastAdjustment,
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
pub struct IApplicationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationFactory {
    type Vtable = IApplicationFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x93bbe361_be5a_4ee3_b4a3_95118dc97a89,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationFactory_Vtbl {
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
pub struct IApplicationInitializationCallbackParams(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationInitializationCallbackParams {
    type Vtable = IApplicationInitializationCallbackParams_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationInitializationCallbackParams {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x751b792e_5772_4488_8b87_f547faa64474,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationInitializationCallbackParams_Vtbl {
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
pub struct IApplicationOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationOverrides {
    type Vtable = IApplicationOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x25f99ff7_9347_459a_9fac_b2d0e11c1a0f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    OnActivated: usize,
    OnLaunched: usize,
    OnFileActivated: usize,
    OnSearchActivated: usize,
    OnShareTargetActivated: usize,
    OnFileOpenPickerActivated: usize,
    OnFileSavePickerActivated: usize,
    OnCachedFileUpdaterActivated: usize,
    pub OnWindowCreated: unsafe extern "system" fn(
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
pub struct IApplicationOverrides2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationOverrides2 {
    type Vtable = IApplicationOverrides2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationOverrides2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdb5cd2b9_d3b4_558c_c64e_0434fd1bd889,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationOverrides2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    OnBackgroundActivated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IApplicationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationStatics {
    type Vtable = IApplicationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x06499997_f7b4_45fe_b763_7577d1d3cb4a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LoadComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        component: *mut ::core::ffi::c_void,
        resourcelocator: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LoadComponentWithResourceLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        component: *mut ::core::ffi::c_void,
        resourcelocator: *mut ::core::ffi::c_void,
        componentresourcelocation: Controls::Primitives::ComponentResourceLocation,
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
pub struct IBindingFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingFailedEventArgs {
    type Vtable = IBindingFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x32c1d013_4dbd_446d_bbb8_0de35048a449,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IBringIntoViewOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBringIntoViewOptions {
    type Vtable = IBringIntoViewOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBringIntoViewOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x19bdd1b5_c7cb_46d9_a4dd_a1bbe83ef2fb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBringIntoViewOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AnimationDesired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAnimationDesired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub TargetRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTargetRect: unsafe extern "system" fn(
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
pub struct IBringIntoViewOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBringIntoViewOptions2 {
    type Vtable = IBringIntoViewOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBringIntoViewOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe855e08e_64b6_1211_bddb_1fddbb6e8231,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBringIntoViewOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetHorizontalAlignmentRatio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetVerticalAlignmentRatio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
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
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IBringIntoViewRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBringIntoViewRequestedEventArgs {
    type Vtable = IBringIntoViewRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBringIntoViewRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x0e629ec4_2206_4c8b_94ae_bdb66a4ebfd1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBringIntoViewRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TargetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTargetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AnimationDesired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAnimationDesired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub TargetRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub SetTargetRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
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
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
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
pub struct IBrushTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushTransition {
    type Vtable = IBrushTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBrushTransition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1116972c_9dad_5429_a7dd_b2b7d061ab8e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushTransition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Foundation::TimeSpan,
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
pub struct IBrushTransitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushTransitionFactory {
    type Vtable = IBrushTransitionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBrushTransitionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3dbe7368_13d4_510c_a215_7539f4787b52,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushTransitionFactory_Vtbl {
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
pub struct IColorPaletteResources(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPaletteResources {
    type Vtable = IColorPaletteResources_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorPaletteResources {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x258088c4_aef2_5d3f_833b_c36db6278ed9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPaletteResources_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AltHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAltHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AltLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAltLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AltMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAltMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AltMediumHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAltMediumHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AltMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAltMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BaseHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetBaseHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BaseLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetBaseLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BaseMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetBaseMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BaseMediumHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetBaseMediumHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BaseMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetBaseMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeAltLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeAltLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeBlackHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeBlackHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeBlackLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeBlackLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeBlackMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeBlackMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeBlackMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeBlackMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeDisabledHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeDisabledHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeDisabledLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeDisabledLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeHigh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeMediumLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChromeGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChromeGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ListLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetListLow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ListMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetListMedium: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ErrorText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetErrorText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Accent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAccent: unsafe extern "system" fn(
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
pub struct IColorPaletteResourcesFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPaletteResourcesFactory {
    type Vtable = IColorPaletteResourcesFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorPaletteResourcesFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa57f0783_1876_5cc0_8ea5_bc77b17e0f7e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPaletteResourcesFactory_Vtbl {
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
pub struct ICornerRadiusHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICornerRadiusHelper {
    type Vtable = ICornerRadiusHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICornerRadiusHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfd7be182_1cdb_4288_b8c8_85ee79297bfc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICornerRadiusHelper_Vtbl {
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
pub struct ICornerRadiusHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICornerRadiusHelperStatics {
    type Vtable = ICornerRadiusHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICornerRadiusHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf4a1f659_d4d4_451f_a387_d6bf4b2451d4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICornerRadiusHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromRadii: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        topleft: f64,
        topright: f64,
        bottomright: f64,
        bottomleft: f64,
        result__: *mut CornerRadius,
    ) -> ::windows_core::HRESULT,
    pub FromUniformRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uniformradius: f64,
        result__: *mut CornerRadius,
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
pub struct IDataContextChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataContextChangedEventArgs {
    type Vtable = IDataContextChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataContextChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7da68e21_0b8f_4f9f_a143_f8e7780136a2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataContextChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NewValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
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
pub struct IDataTemplate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataTemplate {
    type Vtable = IDataTemplate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9910aec7_8ab5_4118_9bc6_09f45a35073d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LoadContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDataTemplateExtension(::windows_core::IUnknown);
impl IDataTemplateExtension {}
::windows_core::imp::interface_hierarchy!(
    IDataTemplateExtension, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IDataTemplateExtension {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{595e9547-cdff-4b92-b773-ab396878f353}",
    );
}
unsafe impl ::windows_core::Interface for IDataTemplateExtension {
    type Vtable = IDataTemplateExtension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplateExtension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x595e9547_cdff_4b92_b773_ab396878f353,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateExtension_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResetTemplate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ProcessBinding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        phase: u32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ProcessBindings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        arg: *mut ::core::ffi::c_void,
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
pub struct IDataTemplateFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataTemplateFactory {
    type Vtable = IDataTemplateFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplateFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x51ed9d7e_2b53_475b_9c88_0c1832c8351a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateFactory_Vtbl {
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
pub struct IDataTemplateKey(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataTemplateKey {
    type Vtable = IDataTemplateKey_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplateKey {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x873b6c28_cceb_4b61_86fa_b2cec39cc2fa,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateKey_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DataType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(
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
pub struct IDataTemplateKeyFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataTemplateKeyFactory {
    type Vtable = IDataTemplateKeyFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplateKeyFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe96b2959_d982_4152_91cb_de0e4dfd7693,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateKeyFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        datatype: *mut ::core::ffi::c_void,
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
pub struct IDataTemplateStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataTemplateStatics2 {
    type Vtable = IDataTemplateStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplateStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8af77d73_aa01_471e_bedd_8bad86219b77,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtensionInstanceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetExtensionInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetExtensionInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
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
pub struct IDebugSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDebugSettings {
    type Vtable = IDebugSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDebugSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3d451f98_c6a7_4d17_8398_d83a067183d8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnableFrameRateCounter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetEnableFrameRateCounter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsBindingTracingEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsBindingTracingEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsOverdrawHeatMapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsOverdrawHeatMapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub BindingFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveBindingFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IDebugSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDebugSettings2 {
    type Vtable = IDebugSettings2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDebugSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x48d37585_e1a6_469b_83c8_30825037119e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnableRedrawRegions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetEnableRedrawRegions: unsafe extern "system" fn(
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
pub struct IDebugSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDebugSettings3 {
    type Vtable = IDebugSettings3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDebugSettings3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe6bb5022_0625_479f_8e32_4b583d73b7ac,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSettings3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTextPerformanceVisualizationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsTextPerformanceVisualizationEnabled: unsafe extern "system" fn(
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
pub struct IDebugSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDebugSettings4 {
    type Vtable = IDebugSettings4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDebugSettings4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc9001e45_e824_5a5f_866c_e20cec88a8fc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSettings4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FailFastOnErrors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetFailFastOnErrors: unsafe extern "system" fn(
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
pub struct IDependencyObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyObject {
    type Vtable = IDependencyObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5c526665_f60e_4912_af59_5fe0680f089d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObject_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ClearValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ReadLocalValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAnimationBaseValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Dispatcher: unsafe extern "system" fn(
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
pub struct IDependencyObject2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyObject2 {
    type Vtable = IDependencyObject2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyObject2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x29fed85d_3d22_43a1_add0_17027c08b212,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObject2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RegisterPropertyChangedCallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        result__: *mut i64,
    ) -> ::windows_core::HRESULT,
    pub UnregisterPropertyChangedCallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        token: i64,
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
pub struct IDependencyObjectCollectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyObjectCollectionFactory {
    type Vtable = IDependencyObjectCollectionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyObjectCollectionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x051e79ff_b3a8_49ee_b5af_ac8f68b649e4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObjectCollectionFactory_Vtbl {
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
pub struct IDependencyObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyObjectFactory {
    type Vtable = IDependencyObjectFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyObjectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9a03af92_7d8a_4937_884f_ecf34fe02acb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObjectFactory_Vtbl {
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
pub struct IDependencyProperty(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyProperty {
    type Vtable = IDependencyProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyProperty {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x85b13970_9bc4_4e96_acf1_30c8fd3d55c8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyProperty_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    GetMetadata: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDependencyPropertyChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyPropertyChangedEventArgs {
    type Vtable = IDependencyPropertyChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyPropertyChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x81212c2b_24d0_4957_abc3_224470a93a4e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyPropertyChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OldValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(
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
pub struct IDependencyPropertyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDependencyPropertyStatics {
    type Vtable = IDependencyPropertyStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDependencyPropertyStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x49e5f28f_8259_4d5c_aae0_83d56dbb68d9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyPropertyStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnsetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    Register: usize,
    RegisterAttached: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDispatcherTimer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherTimer {
    type Vtable = IDispatcherTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherTimer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd160ce46_cd22_4f5f_8c97_40e61da3e2dc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherTimer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Interval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Tick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveTick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(
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
pub struct IDispatcherTimerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherTimerFactory {
    type Vtable = IDispatcherTimerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherTimerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe9961e6e_3626_403a_afe0_040d58165632,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherTimerFactory_Vtbl {
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
pub struct IDragEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragEventArgs {
    type Vtable = IDragEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb440c7c3_02b4_4980_9342_25dae1c0f188,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    Data: usize,
    SetData: usize,
    pub GetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Point,
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
pub struct IDragEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragEventArgs2 {
    type Vtable = IDragEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x26336658_2917_411d_bfc3_2f22471cbbe7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    DataView: usize,
    pub DragUIOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    Modifiers: usize,
    AcceptedOperation: usize,
    SetAcceptedOperation: usize,
    pub GetDeferral: unsafe extern "system" fn(
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
pub struct IDragEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragEventArgs3 {
    type Vtable = IDragEventArgs3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragEventArgs3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd04fc3c6_8119_427a_8152_5f9550cc0416,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    AllowedOperations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragOperationDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragOperationDeferral {
    type Vtable = IDragOperationDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragOperationDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xba73ecba_1b73_4086_b3d3_c223beea1633,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOperationDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(
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
pub struct IDragStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragStartingEventArgs {
    type Vtable = IDragStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragStartingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6800d3fa_90b8_46f9_8e30_5ac25f73f0f9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    Data: usize,
    pub DragUI: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Point,
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
pub struct IDragStartingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragStartingEventArgs2 {
    type Vtable = IDragStartingEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragStartingEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd855e08e_44b6_4211_bd0b_7fddbb6e8231,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    AllowedOperations: usize,
    SetAllowedOperations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDragUI(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragUI {
    type Vtable = IDragUI_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragUI {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2d9bd838_7c60_4842_9170_346fe10a226a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragUI_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    SetContentFromBitmapImage: usize,
    SetContentFromBitmapImageWithAnchorPoint: usize,
    SetContentFromSoftwareBitmap: usize,
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub SetContentFromDataPackage: unsafe extern "system" fn(
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
pub struct IDragUIOverride(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragUIOverride {
    type Vtable = IDragUIOverride_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragUIOverride {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbd6c9dfa_c961_4861_b7a5_bf4fe4a8a6ef,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragUIOverride_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Caption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsContentVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    SetContentFromBitmapImage: usize,
    SetContentFromBitmapImageWithAnchorPoint: usize,
    SetContentFromSoftwareBitmap: usize,
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDropCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropCompletedEventArgs {
    type Vtable = IDropCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDropCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6c4fc188_95bc_4261_9ec5_21cab677b734,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    DropResult: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDurationHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDurationHelper {
    type Vtable = IDurationHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDurationHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x25c1659f_4497_4135_940f_ee96f4d6e934,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDurationHelper_Vtbl {
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
pub struct IDurationHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDurationHelperStatics {
    type Vtable = IDurationHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDurationHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbc88093e_3547_4ec0_b519_ffa8f9c4838c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDurationHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Automatic: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Duration,
    ) -> ::windows_core::HRESULT,
    pub Forever: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Duration,
    ) -> ::windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        duration1: Duration,
        duration2: Duration,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub FromTimeSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        timespan: super::super::Foundation::TimeSpan,
        result__: *mut Duration,
    ) -> ::windows_core::HRESULT,
    pub GetHasTimeSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Duration,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Duration,
        duration: Duration,
        result__: *mut Duration,
    ) -> ::windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Duration,
        value: Duration,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Subtract: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Duration,
        duration: Duration,
        result__: *mut Duration,
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
pub struct IEffectiveViewportChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEffectiveViewportChangedEventArgs {
    type Vtable = IEffectiveViewportChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEffectiveViewportChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x55ee2e81_1c18_59ed_bd3d_c4ca8fa7d190,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEffectiveViewportChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EffectiveViewport: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub MaxViewport: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub BringIntoViewDistanceX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub BringIntoViewDistanceY: unsafe extern "system" fn(
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
pub struct IElementFactory(::windows_core::IUnknown);
impl IElementFactory {}
::windows_core::imp::interface_hierarchy!(
    IElementFactory, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IElementFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{17d2ad90-1370-55c8-80e1-78b49004a9e1}",
    );
}
unsafe impl ::windows_core::Interface for IElementFactory {
    type Vtable = IElementFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x17d2ad90_1370_55c8_80e1_78b49004a9e1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RecycleElement: unsafe extern "system" fn(
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
pub struct IElementFactoryGetArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementFactoryGetArgs {
    type Vtable = IElementFactoryGetArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementFactoryGetArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfb508774_41a3_5829_9255_cf452d041df4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementFactoryGetArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetParent: unsafe extern "system" fn(
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
pub struct IElementFactoryGetArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementFactoryGetArgsFactory {
    type Vtable = IElementFactoryGetArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementFactoryGetArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc3b6dae7_883b_5fd7_be80_2059d877e783,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementFactoryGetArgsFactory_Vtbl {
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
pub struct IElementFactoryRecycleArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementFactoryRecycleArgs {
    type Vtable = IElementFactoryRecycleArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementFactoryRecycleArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x86f16b14_37e8_5dd8_a90c_25d3710318b0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementFactoryRecycleArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Element: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetParent: unsafe extern "system" fn(
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
pub struct IElementFactoryRecycleArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementFactoryRecycleArgsFactory {
    type Vtable = IElementFactoryRecycleArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementFactoryRecycleArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8d926509_ea0d_541b_8271_f9e9118f5e7c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementFactoryRecycleArgsFactory_Vtbl {
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
pub struct IElementSoundPlayer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementSoundPlayer {
    type Vtable = IElementSoundPlayer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementSoundPlayer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x387773a5_f036_460c_9b81_f3d6ea43f6f2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementSoundPlayer_Vtbl {
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
pub struct IElementSoundPlayerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementSoundPlayerStatics {
    type Vtable = IElementSoundPlayerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementSoundPlayerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x217a9004_981d_41c9_b152_ada911a4b13a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementSoundPlayerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Volume: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ElementSoundPlayerState,
    ) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ElementSoundPlayerState,
    ) -> ::windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sound: ElementSoundKind,
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
pub struct IElementSoundPlayerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementSoundPlayerStatics2 {
    type Vtable = IElementSoundPlayerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementSoundPlayerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf2505956_ed41_48d7_aae8_f2abcb444929,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementSoundPlayerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SpatialAudioMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ElementSpatialAudioMode,
    ) -> ::windows_core::HRESULT,
    pub SetSpatialAudioMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ElementSpatialAudioMode,
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
pub struct IEventTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEventTrigger {
    type Vtable = IEventTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEventTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdef8f855_0b49_4087_b1a9_b8b38488f786,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RoutedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRoutedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(
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
pub struct IExceptionRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExceptionRoutedEventArgs {
    type Vtable = IExceptionRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IExceptionRoutedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdd9ff16a_4b62_4a6c_a49d_0671ef6136be,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IExceptionRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorMessage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IExceptionRoutedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExceptionRoutedEventArgsFactory {
    type Vtable = IExceptionRoutedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IExceptionRoutedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbba9826d_5d7a_44e7_b893_b2ae0dd24273,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IExceptionRoutedEventArgsFactory_Vtbl {
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
pub struct IFrameworkElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElement {
    type Vtable = IFrameworkElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa391d09b_4a99_4b7c_9d8d_6fa5d01f6fbf,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Triggers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Resources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetResources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ActualWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ActualHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMinWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MaxWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMaxWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MinHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMinHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MaxHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub HorizontalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut HorizontalAlignment,
    ) -> ::windows_core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: HorizontalAlignment,
    ) -> ::windows_core::HRESULT,
    pub VerticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut VerticalAlignment,
    ) -> ::windows_core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: VerticalAlignment,
    ) -> ::windows_core::HRESULT,
    pub Margin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Thickness,
    ) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub BaseUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DataContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDataContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Style: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FlowDirection,
    ) -> ::windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FlowDirection,
    ) -> ::windows_core::HRESULT,
    pub Loaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Unloaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveUnloaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub SizeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveSizeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub LayoutUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveLayoutUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    SetBinding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFrameworkElement2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElement2 {
    type Vtable = IFrameworkElement2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElement2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf19104be_422a_4904_a52f_ee72010429e5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestedTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ElementTheme,
    ) -> ::windows_core::HRESULT,
    pub SetRequestedTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ElementTheme,
    ) -> ::windows_core::HRESULT,
    pub DataContextChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDataContextChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    GetBindingExpression: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFrameworkElement3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElement3 {
    type Vtable = IFrameworkElement3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElement3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc81c2720_5c52_4bbe_a199_2b1e34f00f70,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Loading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IFrameworkElement4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElement4 {
    type Vtable = IFrameworkElement4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElement4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6b765bb3_fba3_4404_bdee_1a45d1ca5f21,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetFocusVisualMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Thickness,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualSecondaryThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetFocusVisualSecondaryThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Thickness,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualPrimaryThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetFocusVisualPrimaryThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Thickness,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualSecondaryBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFocusVisualSecondaryBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualPrimaryBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFocusVisualPrimaryBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusWhenDisabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(
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
pub struct IFrameworkElement6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElement6 {
    type Vtable = IFrameworkElement6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElement6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x792a5d91_62a1_40bf_a0ce_f9c131fcb7a7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ActualTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ElementTheme,
    ) -> ::windows_core::HRESULT,
    pub ActualThemeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveActualThemeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IFrameworkElement7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElement7 {
    type Vtable = IFrameworkElement7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElement7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2263886c_c069_570f_b9cc_9e21dd028d8e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub EffectiveViewportChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveEffectiveViewportChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IFrameworkElementFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementFactory {
    type Vtable = IFrameworkElementFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdeaee126_03ca_4966_b576_604cce93b5e8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementFactory_Vtbl {
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
pub struct IFrameworkElementOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementOverrides {
    type Vtable = IFrameworkElementOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xda007e54_b3c2_4b9a_aa8e_d3f071262b97,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MeasureOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        availablesize: super::super::Foundation::Size,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub ArrangeOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        finalsize: super::super::Foundation::Size,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub OnApplyTemplate: unsafe extern "system" fn(
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
pub struct IFrameworkElementOverrides2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementOverrides2 {
    type Vtable = IFrameworkElementOverrides2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementOverrides2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xcb5cd2b9_e3b4_458c_b64e_1434fd1bd88a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementOverrides2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GoToElementStateCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        statename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        usetransitions: bool,
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
pub struct IFrameworkElementProtected7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementProtected7 {
    type Vtable = IFrameworkElementProtected7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementProtected7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x65aa0480_22e3_5103_ad2a_b626f88ca5ae,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementProtected7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InvalidateViewport: unsafe extern "system" fn(
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
pub struct IFrameworkElementStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementStatics {
    type Vtable = IFrameworkElementStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x48383032_fbeb_4f8a_aed2_ee21fb27a57b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TagProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ActualWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ActualHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub WidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaxWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaxHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HorizontalAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub VerticalAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DataContextProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FlowDirectionProperty: unsafe extern "system" fn(
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
pub struct IFrameworkElementStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementStatics2 {
    type Vtable = IFrameworkElementStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9695db02_c0d8_4fa2_b100_3fa2df8b9538,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestedThemeProperty: unsafe extern "system" fn(
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
pub struct IFrameworkElementStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementStatics4 {
    type Vtable = IFrameworkElementStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9c41b155_c5d8_4663_bff2_d8d54fb5dbb3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualMarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualSecondaryThicknessProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualPrimaryThicknessProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualSecondaryBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusVisualPrimaryBrushProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusWhenDisabledProperty: unsafe extern "system" fn(
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
pub struct IFrameworkElementStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementStatics5 {
    type Vtable = IFrameworkElementStatics5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x525d3941_0b3c_4be6_9978_19a8025c09d8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeferTree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
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
pub struct IFrameworkElementStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkElementStatics6 {
    type Vtable = IFrameworkElementStatics6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkElementStatics6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfcc1529a_69db_4582_a7be_cf6a1cfdacd0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ActualThemeProperty: unsafe extern "system" fn(
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
pub struct IFrameworkTemplate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkTemplate {
    type Vtable = IFrameworkTemplate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkTemplate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa1e254d8_a446_4a27_9a9d_a0f59e1258a5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkTemplate_Vtbl {
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
pub struct IFrameworkTemplateFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkTemplateFactory {
    type Vtable = IFrameworkTemplateFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkTemplateFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1a78a0a5_937d_46d4_832b_94ff14dab061,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkTemplateFactory_Vtbl {
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
pub struct IFrameworkView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkView {
    type Vtable = IFrameworkView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkView {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xddba664b_b603_47aa_942d_3833174f0d80,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkView_Vtbl {
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
pub struct IFrameworkViewSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameworkViewSource {
    type Vtable = IFrameworkViewSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameworkViewSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe3b077da_35ad_4b09_b5b2_27420041ba9f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkViewSource_Vtbl {
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
pub struct IGridLengthHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridLengthHelper {
    type Vtable = IGridLengthHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridLengthHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7a826ce1_07a0_4083_b6d1_b1d917b976ac,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridLengthHelper_Vtbl {
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
pub struct IGridLengthHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridLengthHelperStatics {
    type Vtable = IGridLengthHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridLengthHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9d457b9b_019f_4266_8872_215f198f6a9d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridLengthHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Auto: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GridLength,
    ) -> ::windows_core::HRESULT,
    pub FromPixels: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pixels: f64,
        result__: *mut GridLength,
    ) -> ::windows_core::HRESULT,
    pub FromValueAndType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
        r#type: GridUnitType,
        result__: *mut GridLength,
    ) -> ::windows_core::HRESULT,
    pub GetIsAbsolute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: GridLength,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub GetIsAuto: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: GridLength,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub GetIsStar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: GridLength,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: GridLength,
        value: GridLength,
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
pub struct IMediaFailedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFailedRoutedEventArgs {
    type Vtable = IMediaFailedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaFailedRoutedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x46d1fa8d_5149_4153_ba3c_b03e64ee531e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFailedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorTrace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IPointHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointHelper {
    type Vtable = IPointHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x727bdd92_64b0_49cf_a321_a9793e73e2e7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointHelper_Vtbl {
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
pub struct IPointHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointHelperStatics {
    type Vtable = IPointHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x015aca75_76d8_4b7e_8a33_7d79204691ee,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromCoordinates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        result__: *mut super::super::Foundation::Point,
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
pub struct IPropertyMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyMetadata {
    type Vtable = IPropertyMetadata_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyMetadata {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x814ef30d_8d18_448a_8644_f2cb51e70380,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyMetadata_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DefaultValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateDefaultValueCallback: unsafe extern "system" fn(
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
pub struct IPropertyMetadataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyMetadataFactory {
    type Vtable = IPropertyMetadataFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyMetadataFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc1b81cc0_57cd_4f2f_b0a9_e1801b28f76b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyMetadataFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDefaultValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        defaultvalue: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithDefaultValueAndCallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        defaultvalue: *mut ::core::ffi::c_void,
        propertychangedcallback: *mut ::core::ffi::c_void,
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
pub struct IPropertyMetadataStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyMetadataStatics {
    type Vtable = IPropertyMetadataStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyMetadataStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3b01077a_6e06_45e9_8b5c_af243458c062,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyMetadataStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithDefaultValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        defaultvalue: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithDefaultValueAndCallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        defaultvalue: *mut ::core::ffi::c_void,
        propertychangedcallback: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        createdefaultvaluecallback: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithFactoryAndCallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        createdefaultvaluecallback: *mut ::core::ffi::c_void,
        propertychangedcallback: *mut ::core::ffi::c_void,
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
pub struct IPropertyPath(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyPath {
    type Vtable = IPropertyPath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyPath {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x300e5d8a_1ff3_4d2c_95ec_27f81debacb8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPath_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IPropertyPathFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyPathFactory {
    type Vtable = IPropertyPathFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyPathFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4e4cdf99_9826_4e56_847c_ca055f162905,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPathFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IRectHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectHelper {
    type Vtable = IRectHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRectHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa38781e2_4bfb_4ee2_afe5_89f31b37478d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectHelper_Vtbl {
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
pub struct IRectHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectHelperStatics {
    type Vtable = IRectHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRectHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5ee163e4_c17e_494f_b580_2f0574fc3a15,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Empty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub FromCoordinatesAndDimensions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub FromPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point1: super::super::Foundation::Point,
        point2: super::super::Foundation::Point,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub FromLocationAndSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        location: super::super::Foundation::Point,
        size: super::super::Foundation::Size,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub GetIsEmpty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub GetBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub GetLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub GetRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub GetTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        point: super::super::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        value: super::super::Foundation::Rect,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Intersect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        rect: super::super::Foundation::Rect,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub UnionWithPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        point: super::super::Foundation::Point,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub UnionWithRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Rect,
        rect: super::super::Foundation::Rect,
        result__: *mut super::super::Foundation::Rect,
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
pub struct IResourceDictionary(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceDictionary {
    type Vtable = IResourceDictionary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceDictionary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc1ea4f24_d6de_4191_8e3a_f48601f7489c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceDictionary_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MergedDictionaries: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ThemeDictionaries: unsafe extern "system" fn(
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
pub struct IResourceDictionaryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceDictionaryFactory {
    type Vtable = IResourceDictionaryFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceDictionaryFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xea3639b5_31b7_4271_92c9_7c9584a91c22,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceDictionaryFactory_Vtbl {
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
pub struct IRoutedEvent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutedEvent {
    type Vtable = IRoutedEvent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRoutedEvent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa6b25818_43c1_4c70_865c_7bdd5a32e327,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEvent_Vtbl {
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
pub struct IRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutedEventArgs {
    type Vtable = IRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRoutedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5c985ac6_d802_4b38_a223_bf070c43fedf,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OriginalSource: unsafe extern "system" fn(
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
pub struct IRoutedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutedEventArgsFactory {
    type Vtable = IRoutedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRoutedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb61c4d87_70e5_412e_b520_1a41ee76bbf4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgsFactory_Vtbl {
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
pub struct IScalarTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScalarTransition {
    type Vtable = IScalarTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScalarTransition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4cb68238_e15d_524e_a73c_9d4dcfbea226,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScalarTransition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Foundation::TimeSpan,
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
pub struct IScalarTransitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScalarTransitionFactory {
    type Vtable = IScalarTransitionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScalarTransitionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc9b1e9ee_90da_5ddd_be64_3e47977ea280,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScalarTransitionFactory_Vtbl {
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
pub struct ISetter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetter {
    type Vtable = ISetter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISetter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa73ded29_b4ae_4a81_be85_e690ba0d3b6e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
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
pub struct ISetter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetter2 {
    type Vtable = ISetter2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISetter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x70169561_05b1_4fa3_9d53_8e0c8c747afc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Target: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(
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
pub struct ISetterBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetterBase {
    type Vtable = ISetterBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISetterBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x418be27c_2ac4_4f22_8097_dea3aeeb2fb3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetterBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSealed: unsafe extern "system" fn(
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
pub struct ISetterBaseCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetterBaseCollection {
    type Vtable = ISetterBaseCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISetterBaseCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x03c40ca8_909e_4117_811c_a4529496bdf1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetterBaseCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSealed: unsafe extern "system" fn(
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
pub struct ISetterBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetterBaseFactory {
    type Vtable = ISetterBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISetterBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x81f8ad60_1ce8_469d_a667_16e37cef8ba9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetterBaseFactory_Vtbl {
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
pub struct ISetterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetterFactory {
    type Vtable = ISetterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISetterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd3ca3d42_09b1_49d5_8891_e7b5648e02a2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        targetproperty: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
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
pub struct ISizeChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISizeChangedEventArgs {
    type Vtable = ISizeChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISizeChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd5312e60_5cc1_42a1_920c_1af46be2f986,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISizeChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PreviousSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub NewSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Size,
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
pub struct ISizeHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISizeHelper {
    type Vtable = ISizeHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISizeHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe7225a94_5d03_4a03_ba94_967fc68fcefe,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISizeHelper_Vtbl {
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
pub struct ISizeHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISizeHelperStatics {
    type Vtable = ISizeHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISizeHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6286c5b2_cf78_4915_aa40_76004a165f5e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISizeHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Empty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub FromDimensions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub GetIsEmpty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Size,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: super::super::Foundation::Size,
        value: super::super::Foundation::Size,
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
pub struct IStateTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStateTrigger {
    type Vtable = IStateTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStateTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x67adef2e_d8d9_49f7_a1fd_2e35eedd23cd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStateTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(
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
pub struct IStateTriggerBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStateTriggerBase {
    type Vtable = IStateTriggerBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStateTriggerBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x48b20698_af06_466c_8052_93666dde0e49,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStateTriggerBase_Vtbl {
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
pub struct IStateTriggerBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStateTriggerBaseFactory {
    type Vtable = IStateTriggerBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStateTriggerBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x970e2c4b_bfaf_47b0_be42_c1d711bb2e9f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStateTriggerBaseFactory_Vtbl {
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
pub struct IStateTriggerBaseProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStateTriggerBaseProtected {
    type Vtable = IStateTriggerBaseProtected_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStateTriggerBaseProtected {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3c41e253_8d14_4216_994c_f9930429f6e5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStateTriggerBaseProtected_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        isactive: bool,
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
pub struct IStateTriggerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStateTriggerStatics {
    type Vtable = IStateTriggerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStateTriggerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x71e95c90_b3fe_4dd3_a8a8_44a2ce25e0b8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStateTriggerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsActiveProperty: unsafe extern "system" fn(
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
pub struct IStyle(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStyle {
    type Vtable = IStyle_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStyle {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc4a9f225_9db7_4a7d_b6d1_f74edb9293c2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStyle_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSealed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Setters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    TargetType: usize,
    SetTargetType: usize,
    pub BasedOn: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetBasedOn: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Seal: unsafe extern "system" fn(
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
pub struct IStyleFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStyleFactory {
    type Vtable = IStyleFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStyleFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa36824e3_3d81_4ce5_aa51_8b410f602fcd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStyleFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ITargetPropertyPath(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetPropertyPath {
    type Vtable = ITargetPropertyPath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITargetPropertyPath {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x40740f8e_085f_4ced_be70_6f47acf15ad0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetPropertyPath_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Target: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(
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
pub struct ITargetPropertyPathFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITargetPropertyPathFactory {
    type Vtable = ITargetPropertyPathFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITargetPropertyPathFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x88eeccc8_99e2_4a44_9907_b44bc86e2bbe,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetPropertyPathFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        targetproperty: *mut ::core::ffi::c_void,
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
pub struct IThicknessHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThicknessHelper {
    type Vtable = IThicknessHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThicknessHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa86bae4b_1e8f_4eeb_9013_0b2838a97b34,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThicknessHelper_Vtbl {
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
pub struct IThicknessHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThicknessHelperStatics {
    type Vtable = IThicknessHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThicknessHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc0991a7c_070c_4da6_8784_01ca800eb73a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThicknessHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromLengths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        result__: *mut Thickness,
    ) -> ::windows_core::HRESULT,
    pub FromUniformLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uniformlength: f64,
        result__: *mut Thickness,
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
pub struct ITriggerAction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITriggerAction {
    type Vtable = ITriggerAction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITriggerAction {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa2c0df02_63d5_4b46_9b83_0868d3079621,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerAction_Vtbl {
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
pub struct ITriggerActionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITriggerActionFactory {
    type Vtable = ITriggerActionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITriggerActionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x68d2c0b9_3289_414f_8f6e_c6b97aedda03,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerActionFactory_Vtbl {
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
pub struct ITriggerBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITriggerBase {
    type Vtable = ITriggerBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITriggerBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe7ea222f_dee6_4393_a8b2_8923d641f395,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerBase_Vtbl {
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
pub struct ITriggerBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITriggerBaseFactory {
    type Vtable = ITriggerBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITriggerBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6a3b9e57_fc5d_42d0_8cb9_ca50667af746,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerBaseFactory_Vtbl {
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
pub struct IUIElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement {
    type Vtable = IUIElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x676d0be9_b65c_41c6_ba40_58cf87f201c1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DesiredSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub AllowDrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowDrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Opacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Clip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetClip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RenderTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRenderTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Projection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetProjection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RenderTransformOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetRenderTransformOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub IsHitTestVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsHitTestVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Visibility: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Visibility,
    ) -> ::windows_core::HRESULT,
    pub SetVisibility: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Visibility,
    ) -> ::windows_core::HRESULT,
    pub RenderSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub UseLayoutRounding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetUseLayoutRounding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    Transitions: usize,
    SetTransitions: usize,
    pub CacheMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCacheMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsTapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsTapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsDoubleTapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsDoubleTapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsRightTapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsRightTapEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsHoldingEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsHoldingEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ManipulationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::ManipulationModes,
    ) -> ::windows_core::HRESULT,
    pub SetManipulationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::ManipulationModes,
    ) -> ::windows_core::HRESULT,
    pub PointerCaptures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveKeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub KeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveKeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DragEnter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragEnter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DragLeave: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragLeave: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DragOver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragOver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Drop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerCaptureLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerCaptureLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PointerWheelChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePointerWheelChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Tapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DoubleTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDoubleTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Holding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveHolding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RightTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveRightTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ManipulationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveManipulationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ManipulationInertiaStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ManipulationStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveManipulationStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveManipulationDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Measure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        availablesize: super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub Arrange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        finalrect: super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub CapturePointer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ReleasePointerCapture: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ReleasePointerCaptures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        routedevent: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        handledeventstoo: bool,
    ) -> ::windows_core::HRESULT,
    pub RemoveHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        routedevent: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TransformToVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        visual: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InvalidateMeasure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InvalidateArrange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UpdateLayout: unsafe extern "system" fn(
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
pub struct IUIElement10(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement10 {
    type Vtable = IUIElement10_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement10 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd531c629_ad2c_5f6b_adcf_fb87287d18d7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement10_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    ActualOffset: usize,
    ActualSize: usize,
    pub XamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UIContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Shadow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(
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
pub struct IUIElement2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement2 {
    type Vtable = IUIElement2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x676d0bf9_b66c_41d6_ba50_58cf87f201d1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CompositeMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Media::ElementCompositeMode,
    ) -> ::windows_core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Media::ElementCompositeMode,
    ) -> ::windows_core::HRESULT,
    pub CancelDirectManipulations: unsafe extern "system" fn(
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
pub struct IUIElement3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement3 {
    type Vtable = IUIElement3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbc2b28f1_26f2_4aab_b256_3b5350881e37,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    Transform3D: usize,
    SetTransform3D: usize,
    pub CanDrag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanDrag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub DragStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDragStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub DropCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDropCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    StartDragAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IUIElement4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement4 {
    type Vtable = IUIElement4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x69145cd4_199a_4657_9e57_e99e8f136712,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContextFlyout: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetContextFlyout: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ContextRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveContextRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ContextCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveContextCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IUIElement5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement5 {
    type Vtable = IUIElement5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8eed9bc2_a58c_4453_af0f_a92ee06d0317,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Lights: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::KeyTipPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::KeyTipPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub XYFocusKeyboardNavigation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusKeyboardNavigation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows_core::HRESULT,
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    pub HighContrastAdjustment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ElementHighContrastAdjustment,
    ) -> ::windows_core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ElementHighContrastAdjustment,
    ) -> ::windows_core::HRESULT,
    pub TabFocusNavigation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::KeyboardNavigationMode,
    ) -> ::windows_core::HRESULT,
    pub SetTabFocusNavigation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::KeyboardNavigationMode,
    ) -> ::windows_core::HRESULT,
    pub GettingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveGettingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub LosingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveLosingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub NoFocusCandidateFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveNoFocusCandidateFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub StartBringIntoView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StartBringIntoViewWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: *mut ::core::ffi::c_void,
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
pub struct IUIElement7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement7 {
    type Vtable = IUIElement7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xcafc4968_6369_4249_80f9_3d656319e811,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyboardAccelerators: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CharacterReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveCharacterReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ProcessKeyboardAccelerators: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveProcessKeyboardAccelerators: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PreviewKeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePreviewKeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub PreviewKeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemovePreviewKeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
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
pub struct IUIElement8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement8 {
    type Vtable = IUIElement8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement8 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3ab70e85_d508_4477_b6f8_0e435701c836,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyTipTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetKeyTipTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetKeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyboardAcceleratorPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub SetKeyboardAcceleratorPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows_core::HRESULT,
    pub BringIntoViewRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveBringIntoViewRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IUIElement9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElement9 {
    type Vtable = IUIElement9_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElement9 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb4a04776_4e88_50ca_8f2b_08940d6c5f94,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement9_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanBeScrollAnchor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanBeScrollAnchor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub OpacityTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetOpacityTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    Translation: usize,
    SetTranslation: usize,
    pub TranslationTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTranslationTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub RotationTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRotationTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    Scale: usize,
    SetScale: usize,
    pub ScaleTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetScaleTransition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    TransformMatrix: usize,
    SetTransformMatrix: usize,
    CenterPoint: usize,
    SetCenterPoint: usize,
    RotationAxis: usize,
    SetRotationAxis: usize,
    StartAnimation: usize,
    StopAnimation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IUIElementFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementFactory {
    type Vtable = IUIElementFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb9ee93fe_a338_419f_ac32_91dcaadf5d08,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementFactory_Vtbl {
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
pub struct IUIElementOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementOverrides {
    type Vtable = IUIElementOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x608d2f1d_7858_4aeb_89e4_b54e2c7ed3d3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    OnCreateAutomationPeer: usize,
    pub OnDisconnectVisualChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindSubElementsForTouchTargeting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: super::super::Foundation::Point,
        boundingrect: super::super::Foundation::Rect,
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
pub struct IUIElementOverrides7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementOverrides7 {
    type Vtable = IUIElementOverrides7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementOverrides7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb97f7f68_c29b_4c99_a1c3_952619d6e720,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetChildrenInTabFocusOrder: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
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
pub struct IUIElementOverrides8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementOverrides8 {
    type Vtable = IUIElementOverrides8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementOverrides8 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4a5a645c_548d_48cf_b998_7844d6e235a1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnKeyboardAcceleratorInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OnBringIntoViewRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
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
pub struct IUIElementOverrides9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementOverrides9 {
    type Vtable = IUIElementOverrides9_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementOverrides9 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9a6e5973_6d63_54f2_90fa_62813b20b7b9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides9_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    PopulatePropertyInfoOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IUIElementStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics {
    type Vtable = IUIElementStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x58d3573b_f52c_45be_988b_a5869564873c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyDownEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyUpEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerEnteredEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerPressedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerMovedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerReleasedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerExitedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerCaptureLostEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerCanceledEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerWheelChangedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TappedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DoubleTappedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HoldingEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RightTappedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ManipulationStartingEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ManipulationInertiaStartingEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ManipulationStartedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ManipulationDeltaEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ManipulationCompletedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragEnterEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragLeaveEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DragOverEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DropEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowDropProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ClipProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RenderTransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ProjectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RenderTransformOriginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsHitTestVisibleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub VisibilityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UseLayoutRoundingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TransitionsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CacheModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsTapEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsDoubleTapEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsRightTapEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsHoldingEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ManipulationModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PointerCapturesProperty: unsafe extern "system" fn(
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
pub struct IUIElementStatics10(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics10 {
    type Vtable = IUIElementStatics10_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics10 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x60d25362_4b3e_53da_8b78_38db94ae8f26,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics10_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShadowProperty: unsafe extern "system" fn(
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
pub struct IUIElementStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics2 {
    type Vtable = IUIElementStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x58d3574b_f53c_45be_989b_a5869564874c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CompositeModeProperty: unsafe extern "system" fn(
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
pub struct IUIElementStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics3 {
    type Vtable = IUIElementStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd1f87ade_eca1_4561_a32b_64601b4e0597,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Transform3DProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CanDragProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryStartDirectManipulation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
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
pub struct IUIElementStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics4 {
    type Vtable = IUIElementStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1d157d61_16af_411f_b774_272375a4ac2c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContextFlyoutProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
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
pub struct IUIElementStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics5 {
    type Vtable = IUIElementStatics5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x59bd7d91_8fa3_4c65_ba1b_40df38556cbb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LightsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusKeyboardNavigationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HighContrastAdjustmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TabFocusNavigationProperty: unsafe extern "system" fn(
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
pub struct IUIElementStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics6 {
    type Vtable = IUIElementStatics6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x647e03b7_036a_4dea_9540_1dd7fd1266f1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GettingFocusEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LosingFocusEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NoFocusCandidateFoundEvent: unsafe extern "system" fn(
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
pub struct IUIElementStatics7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics7 {
    type Vtable = IUIElementStatics7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xda9b4493_a695_4145_ae93_888024396a0f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PreviewKeyDownEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CharacterReceivedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PreviewKeyUpEvent: unsafe extern "system" fn(
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
pub struct IUIElementStatics8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics8 {
    type Vtable = IUIElementStatics8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics8 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x17be3487_4875_4915_b0b1_a4c0f851df3f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BringIntoViewRequestedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ContextRequestedEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipTargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyboardAcceleratorPlacementTargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyboardAcceleratorPlacementModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RegisterAsScrollPort: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
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
pub struct IUIElementStatics9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementStatics9 {
    type Vtable = IUIElementStatics9_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementStatics9 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x71467e77_8ca3_5ed7_95db_d51cdad77f81,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics9_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanBeScrollAnchorProperty: unsafe extern "system" fn(
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
pub struct IUIElementWeakCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementWeakCollection {
    type Vtable = IUIElementWeakCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementWeakCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x10341223_e66d_519e_acf8_556bd244eac3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementWeakCollection_Vtbl {
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
pub struct IUIElementWeakCollectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIElementWeakCollectionFactory {
    type Vtable = IUIElementWeakCollectionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIElementWeakCollectionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x57242561_188a_5304_8792_a43f35d90f99,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementWeakCollectionFactory_Vtbl {
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
pub struct IUnhandledExceptionEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnhandledExceptionEventArgs {
    type Vtable = IUnhandledExceptionEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUnhandledExceptionEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7230269c_054e_4cf3_86c5_be90eb6863d5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledExceptionEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Exception: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::HRESULT,
    ) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
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
pub struct IVector3Transition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVector3Transition {
    type Vtable = IVector3Transition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVector3Transition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd2e209dc_c4a2_5101_9a68_fa0150505589,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVector3Transition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub Components: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Vector3TransitionComponents,
    ) -> ::windows_core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Vector3TransitionComponents,
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
pub struct IVector3TransitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVector3TransitionFactory {
    type Vtable = IVector3TransitionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVector3TransitionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc3706699_ee9b_50dc_8807_f51d5a759495,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVector3TransitionFactory_Vtbl {
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
pub struct IVisualState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualState {
    type Vtable = IVisualState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualState {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6320affc_c31a_4450_afde_f6ea7bd1f586,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualState_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    Storyboard: usize,
    SetStoryboard: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IVisualState2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualState2 {
    type Vtable = IVisualState2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualState2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x0fa0f896_64c0_45fb_8d24_fb83298c0d93,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualState2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Setters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StateTriggers: unsafe extern "system" fn(
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
pub struct IVisualStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateChangedEventArgs {
    type Vtable = IVisualStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfe216ab1_f31f_4791_8989_c70e1d9b59ff,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetOldState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NewState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetNewState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Control: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetControl: unsafe extern "system" fn(
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
pub struct IVisualStateGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateGroup {
    type Vtable = IVisualStateGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe4f9d9a4_e028_44de_9b15_4929ae0a26c2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Transitions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub States: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CurrentState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CurrentStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub CurrentStateChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveCurrentStateChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IVisualStateManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateManager {
    type Vtable = IVisualStateManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6fda9f9a_6fab_4112_9258_1006a3c3476e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateManager_Vtbl {
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
pub struct IVisualStateManagerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateManagerFactory {
    type Vtable = IVisualStateManagerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateManagerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x85e598fd_a575_47b6_9e30_383cd08585f2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateManagerFactory_Vtbl {
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
pub struct IVisualStateManagerOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateManagerOverrides {
    type Vtable = IVisualStateManagerOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateManagerOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4a66910e_7979_43c8_8ff4_ec6122750006,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateManagerOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GoToStateCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        control: *mut ::core::ffi::c_void,
        templateroot: *mut ::core::ffi::c_void,
        statename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        group: *mut ::core::ffi::c_void,
        state: *mut ::core::ffi::c_void,
        usetransitions: bool,
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
pub struct IVisualStateManagerProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateManagerProtected {
    type Vtable = IVisualStateManagerProtected_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateManagerProtected {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4b3b8640_b0b7_404c_9ef4_d949640e245d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateManagerProtected_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RaiseCurrentStateChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stategroup: *mut ::core::ffi::c_void,
        oldstate: *mut ::core::ffi::c_void,
        newstate: *mut ::core::ffi::c_void,
        control: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RaiseCurrentStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stategroup: *mut ::core::ffi::c_void,
        oldstate: *mut ::core::ffi::c_void,
        newstate: *mut ::core::ffi::c_void,
        control: *mut ::core::ffi::c_void,
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
pub struct IVisualStateManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualStateManagerStatics {
    type Vtable = IVisualStateManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualStateManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x01d0e9e0_d713_414e_a74e_e63ec7ac8c3d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualStateManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetVisualStateGroups: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        obj: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CustomVisualStateManagerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCustomVisualStateManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        obj: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCustomVisualStateManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        obj: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GoToState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        control: *mut ::core::ffi::c_void,
        statename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        usetransitions: bool,
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
pub struct IVisualTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTransition {
    type Vtable = IVisualTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualTransition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x55c5905e_2bc7_400d_aaa4_1a2981491ee0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTransition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GeneratedDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Duration,
    ) -> ::windows_core::HRESULT,
    pub SetGeneratedDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Duration,
    ) -> ::windows_core::HRESULT,
    GeneratedEasingFunction: usize,
    SetGeneratedEasingFunction: usize,
    pub To: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    Storyboard: usize,
    SetStoryboard: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IVisualTransitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTransitionFactory {
    type Vtable = IVisualTransitionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualTransitionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xea75864f_d1e0_4dae_b429_89fc322724f4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTransitionFactory_Vtbl {
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
pub struct IWindow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindow {
    type Vtable = IWindow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindow {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3276167d_c9f6_462d_9de2_ae4c1fd8c2e5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Dispatcher: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub SizeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveSizeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub VisibilityChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveVisibilityChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(
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
pub struct IWindow2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindow2 {
    type Vtable = IWindow2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindow2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd384759f_34f6_4482_8435_f552f9b24cc8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTitleBar: unsafe extern "system" fn(
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
pub struct IWindow3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindow3 {
    type Vtable = IWindow3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindow3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb70bdc9d_1c35_462a_9b97_808d5af9f28e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    Compositor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IWindow4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindow4 {
    type Vtable = IWindow4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindow4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbfe1b8ce_6c40_50f9_854c_7021d2bc9de6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UIContext: unsafe extern "system" fn(
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
pub struct IWindowCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowCreatedEventArgs {
    type Vtable = IWindowCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowCreatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x31b71470_feff_4654_af48_9b398ab5772b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Window: unsafe extern "system" fn(
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
pub struct IWindowStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowStatics {
    type Vtable = IWindowStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x93328409_4ea1_4afa_83dc_0c4e73e88bb1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
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
pub struct IXamlRoot(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRoot {
    type Vtable = IXamlRoot_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlRoot {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x34b50756_1696_5b6d_8e9b_c71464ccad5a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRoot_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub IsHostVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub UIContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::Foundation::EventRegistrationToken,
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
pub struct IXamlRootChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRootChangedEventArgs {
    type Vtable = IXamlRootChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlRootChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x92d71c21_d23c_5a17_bcb8_001504b6bb19,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRootChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct AdaptiveTrigger(::windows_core::IUnknown);
impl AdaptiveTrigger {}
impl ::windows_core::RuntimeType for AdaptiveTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.AdaptiveTrigger;{a5f04119-0cd9-49f1-a23f-44e547ab9f1a})",
    );
}
unsafe impl ::windows_core::Interface for AdaptiveTrigger {
    type Vtable = IAdaptiveTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AdaptiveTrigger {
    const IID: ::windows_core::GUID = <IAdaptiveTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.AdaptiveTrigger";
}
::windows_core::imp::interface_hierarchy!(
    AdaptiveTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<StateTriggerBase> for AdaptiveTrigger {}
impl ::windows_core::CanTryInto<DependencyObject> for AdaptiveTrigger {}
unsafe impl ::core::marker::Send for AdaptiveTrigger {}
unsafe impl ::core::marker::Sync for AdaptiveTrigger {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Application(::windows_core::IUnknown);
impl Application {}
impl ::windows_core::RuntimeType for Application {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Application;{74b861a1-7487-46a9-9a6e-c78b512726c5})",
    );
}
unsafe impl ::windows_core::Interface for Application {
    type Vtable = IApplication_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Application {
    const IID: ::windows_core::GUID = <IApplication as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Application {
    const NAME: &'static str = "Windows.UI.Xaml.Application";
}
::windows_core::imp::interface_hierarchy!(
    Application, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Application {}
unsafe impl ::core::marker::Sync for Application {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ApplicationInitializationCallbackParams(::windows_core::IUnknown);
impl ApplicationInitializationCallbackParams {}
impl ::windows_core::RuntimeType for ApplicationInitializationCallbackParams {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ApplicationInitializationCallbackParams;{751b792e-5772-4488-8b87-f547faa64474})",
    );
}
unsafe impl ::windows_core::Interface for ApplicationInitializationCallbackParams {
    type Vtable = IApplicationInitializationCallbackParams_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ApplicationInitializationCallbackParams {
    const IID: ::windows_core::GUID = <IApplicationInitializationCallbackParams as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ApplicationInitializationCallbackParams {
    const NAME: &'static str = "Windows.UI.Xaml.ApplicationInitializationCallbackParams";
}
::windows_core::imp::interface_hierarchy!(
    ApplicationInitializationCallbackParams, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ApplicationInitializationCallbackParams {}
unsafe impl ::core::marker::Sync for ApplicationInitializationCallbackParams {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct BindingFailedEventArgs(::windows_core::IUnknown);
impl BindingFailedEventArgs {}
impl ::windows_core::RuntimeType for BindingFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.BindingFailedEventArgs;{32c1d013-4dbd-446d-bbb8-0de35048a449})",
    );
}
unsafe impl ::windows_core::Interface for BindingFailedEventArgs {
    type Vtable = IBindingFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindingFailedEventArgs {
    const IID: ::windows_core::GUID = <IBindingFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BindingFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.BindingFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    BindingFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for BindingFailedEventArgs {}
unsafe impl ::core::marker::Sync for BindingFailedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct BringIntoViewOptions(::windows_core::IUnknown);
impl BringIntoViewOptions {}
impl ::windows_core::RuntimeType for BringIntoViewOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.BringIntoViewOptions;{19bdd1b5-c7cb-46d9-a4dd-a1bbe83ef2fb})",
    );
}
unsafe impl ::windows_core::Interface for BringIntoViewOptions {
    type Vtable = IBringIntoViewOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BringIntoViewOptions {
    const IID: ::windows_core::GUID = <IBringIntoViewOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BringIntoViewOptions {
    const NAME: &'static str = "Windows.UI.Xaml.BringIntoViewOptions";
}
::windows_core::imp::interface_hierarchy!(
    BringIntoViewOptions, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for BringIntoViewOptions {}
unsafe impl ::core::marker::Sync for BringIntoViewOptions {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct BringIntoViewRequestedEventArgs(::windows_core::IUnknown);
impl BringIntoViewRequestedEventArgs {}
impl ::windows_core::RuntimeType for BringIntoViewRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.BringIntoViewRequestedEventArgs;{0e629ec4-2206-4c8b-94ae-bdb66a4ebfd1})",
    );
}
unsafe impl ::windows_core::Interface for BringIntoViewRequestedEventArgs {
    type Vtable = IBringIntoViewRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BringIntoViewRequestedEventArgs {
    const IID: ::windows_core::GUID = <IBringIntoViewRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BringIntoViewRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.BringIntoViewRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    BringIntoViewRequestedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RoutedEventArgs> for BringIntoViewRequestedEventArgs {}
unsafe impl ::core::marker::Send for BringIntoViewRequestedEventArgs {}
unsafe impl ::core::marker::Sync for BringIntoViewRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct BrushTransition(::windows_core::IUnknown);
impl BrushTransition {}
impl ::windows_core::RuntimeType for BrushTransition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.BrushTransition;{1116972c-9dad-5429-a7dd-b2b7d061ab8e})",
    );
}
unsafe impl ::windows_core::Interface for BrushTransition {
    type Vtable = IBrushTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BrushTransition {
    const IID: ::windows_core::GUID = <IBrushTransition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BrushTransition {
    const NAME: &'static str = "Windows.UI.Xaml.BrushTransition";
}
::windows_core::imp::interface_hierarchy!(
    BrushTransition, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for BrushTransition {}
unsafe impl ::core::marker::Sync for BrushTransition {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ColorPaletteResources(::windows_core::IUnknown);
impl ColorPaletteResources {
    pub fn First(
        &self,
    ) -> ::windows_core::Result<
        super::super::Foundation::Collections::IIterator<
            super::super::Foundation::Collections::IKeyValuePair<
                ::windows_core::IInspectable,
                ::windows_core::IInspectable,
            >,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows_core::IInspectable,
                    ::windows_core::IInspectable,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .First)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ColorPaletteResources {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ColorPaletteResources;{258088c4-aef2-5d3f-833b-c36db6278ed9})",
    );
}
unsafe impl ::windows_core::Interface for ColorPaletteResources {
    type Vtable = IColorPaletteResources_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ColorPaletteResources {
    const IID: ::windows_core::GUID = <IColorPaletteResources as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ColorPaletteResources {
    const NAME: &'static str = "Windows.UI.Xaml.ColorPaletteResources";
}
impl ::core::iter::IntoIterator for ColorPaletteResources {
    type Item = super::super::Foundation::Collections::IKeyValuePair<
        ::windows_core::IInspectable,
        ::windows_core::IInspectable,
    >;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &ColorPaletteResources {
    type Item = super::super::Foundation::Collections::IKeyValuePair<
        ::windows_core::IInspectable,
        ::windows_core::IInspectable,
    >;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(
    ColorPaletteResources, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<
        super::super::Foundation::Collections::IKeyValuePair<
            ::windows_core::IInspectable,
            ::windows_core::IInspectable,
        >,
    >,
> for ColorPaletteResources {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IMap<
        ::windows_core::IInspectable,
        ::windows_core::IInspectable,
    >,
> for ColorPaletteResources {}
impl ::windows_core::CanTryInto<ResourceDictionary> for ColorPaletteResources {}
impl ::windows_core::CanTryInto<DependencyObject> for ColorPaletteResources {}
unsafe impl ::core::marker::Send for ColorPaletteResources {}
unsafe impl ::core::marker::Sync for ColorPaletteResources {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct CornerRadiusHelper(::windows_core::IUnknown);
impl CornerRadiusHelper {}
impl ::windows_core::RuntimeType for CornerRadiusHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.CornerRadiusHelper;{fd7be182-1cdb-4288-b8c8-85ee79297bfc})",
    );
}
unsafe impl ::windows_core::Interface for CornerRadiusHelper {
    type Vtable = ICornerRadiusHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CornerRadiusHelper {
    const IID: ::windows_core::GUID = <ICornerRadiusHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CornerRadiusHelper {
    const NAME: &'static str = "Windows.UI.Xaml.CornerRadiusHelper";
}
::windows_core::imp::interface_hierarchy!(
    CornerRadiusHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CornerRadiusHelper {}
unsafe impl ::core::marker::Sync for CornerRadiusHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DataContextChangedEventArgs(::windows_core::IUnknown);
impl DataContextChangedEventArgs {}
impl ::windows_core::RuntimeType for DataContextChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DataContextChangedEventArgs;{7da68e21-0b8f-4f9f-a143-f8e7780136a2})",
    );
}
unsafe impl ::windows_core::Interface for DataContextChangedEventArgs {
    type Vtable = IDataContextChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataContextChangedEventArgs {
    const IID: ::windows_core::GUID = <IDataContextChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataContextChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.DataContextChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DataContextChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DataContextChangedEventArgs {}
unsafe impl ::core::marker::Sync for DataContextChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DataTemplate(::windows_core::IUnknown);
impl DataTemplate {}
impl ::windows_core::RuntimeType for DataTemplate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DataTemplate;{9910aec7-8ab5-4118-9bc6-09f45a35073d})",
    );
}
unsafe impl ::windows_core::Interface for DataTemplate {
    type Vtable = IDataTemplate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataTemplate {
    const IID: ::windows_core::GUID = <IDataTemplate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.DataTemplate";
}
::windows_core::imp::interface_hierarchy!(
    DataTemplate, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IElementFactory> for DataTemplate {}
impl ::windows_core::CanTryInto<FrameworkTemplate> for DataTemplate {}
impl ::windows_core::CanTryInto<DependencyObject> for DataTemplate {}
unsafe impl ::core::marker::Send for DataTemplate {}
unsafe impl ::core::marker::Sync for DataTemplate {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DataTemplateKey(::windows_core::IUnknown);
impl DataTemplateKey {}
impl ::windows_core::RuntimeType for DataTemplateKey {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DataTemplateKey;{873b6c28-cceb-4b61-86fa-b2cec39cc2fa})",
    );
}
unsafe impl ::windows_core::Interface for DataTemplateKey {
    type Vtable = IDataTemplateKey_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataTemplateKey {
    const IID: ::windows_core::GUID = <IDataTemplateKey as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataTemplateKey {
    const NAME: &'static str = "Windows.UI.Xaml.DataTemplateKey";
}
::windows_core::imp::interface_hierarchy!(
    DataTemplateKey, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DataTemplateKey {}
unsafe impl ::core::marker::Sync for DataTemplateKey {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DebugSettings(::windows_core::IUnknown);
impl DebugSettings {}
impl ::windows_core::RuntimeType for DebugSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DebugSettings;{3d451f98-c6a7-4d17-8398-d83a067183d8})",
    );
}
unsafe impl ::windows_core::Interface for DebugSettings {
    type Vtable = IDebugSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DebugSettings {
    const IID: ::windows_core::GUID = <IDebugSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DebugSettings {
    const NAME: &'static str = "Windows.UI.Xaml.DebugSettings";
}
::windows_core::imp::interface_hierarchy!(
    DebugSettings, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DebugSettings {}
unsafe impl ::core::marker::Sync for DebugSettings {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DependencyObject(::windows_core::IUnknown);
impl DependencyObject {}
impl ::windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DependencyObject;{5c526665-f60e-4912-af59-5fe0680f089d})",
    );
}
unsafe impl ::windows_core::Interface for DependencyObject {
    type Vtable = IDependencyObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DependencyObject {
    const IID: ::windows_core::GUID = <IDependencyObject as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Windows.UI.Xaml.DependencyObject";
}
::windows_core::imp::interface_hierarchy!(
    DependencyObject, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DependencyObject {}
unsafe impl ::core::marker::Sync for DependencyObject {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DependencyObjectCollection(::windows_core::IUnknown);
impl DependencyObjectCollection {}
impl ::windows_core::RuntimeType for DependencyObjectCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DependencyObjectCollection;pinterface({5917eb53-50b4-4a0d-b309-65862b3f1dbc};rc(Windows.UI.Xaml.DependencyObject;{5c526665-f60e-4912-af59-5fe0680f089d})))",
    );
}
unsafe impl ::windows_core::Interface for DependencyObjectCollection {
    type Vtable = super::super::Foundation::Collections::IObservableVector_Vtbl<
        DependencyObject,
    >;
}
unsafe impl ::windows_core::ComInterface for DependencyObjectCollection {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IObservableVector<
        DependencyObject,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DependencyObjectCollection {
    const NAME: &'static str = "Windows.UI.Xaml.DependencyObjectCollection";
}
impl ::core::iter::IntoIterator for DependencyObjectCollection {
    type Item = DependencyObject;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &DependencyObjectCollection {
    type Item = DependencyObject;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    DependencyObjectCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<DependencyObject>,
> for DependencyObjectCollection {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IObservableVector<DependencyObject>,
> for DependencyObjectCollection {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IVector<DependencyObject>,
> for DependencyObjectCollection {}
impl ::windows_core::CanTryInto<DependencyObject> for DependencyObjectCollection {}
unsafe impl ::core::marker::Send for DependencyObjectCollection {}
unsafe impl ::core::marker::Sync for DependencyObjectCollection {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DependencyProperty(::windows_core::IUnknown);
impl DependencyProperty {}
impl ::windows_core::RuntimeType for DependencyProperty {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DependencyProperty;{85b13970-9bc4-4e96-acf1-30c8fd3d55c8})",
    );
}
unsafe impl ::windows_core::Interface for DependencyProperty {
    type Vtable = IDependencyProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DependencyProperty {
    const IID: ::windows_core::GUID = <IDependencyProperty as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DependencyProperty {
    const NAME: &'static str = "Windows.UI.Xaml.DependencyProperty";
}
::windows_core::imp::interface_hierarchy!(
    DependencyProperty, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DependencyProperty {}
unsafe impl ::core::marker::Sync for DependencyProperty {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DependencyPropertyChangedEventArgs(::windows_core::IUnknown);
impl DependencyPropertyChangedEventArgs {}
impl ::windows_core::RuntimeType for DependencyPropertyChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DependencyPropertyChangedEventArgs;{81212c2b-24d0-4957-abc3-224470a93a4e})",
    );
}
unsafe impl ::windows_core::Interface for DependencyPropertyChangedEventArgs {
    type Vtable = IDependencyPropertyChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DependencyPropertyChangedEventArgs {
    const IID: ::windows_core::GUID = <IDependencyPropertyChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DependencyPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.DependencyPropertyChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DependencyPropertyChangedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DependencyPropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for DependencyPropertyChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DispatcherTimer(::windows_core::IUnknown);
impl DispatcherTimer {}
impl ::windows_core::RuntimeType for DispatcherTimer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DispatcherTimer;{d160ce46-cd22-4f5f-8c97-40e61da3e2dc})",
    );
}
unsafe impl ::windows_core::Interface for DispatcherTimer {
    type Vtable = IDispatcherTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherTimer {
    const IID: ::windows_core::GUID = <IDispatcherTimer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherTimer {
    const NAME: &'static str = "Windows.UI.Xaml.DispatcherTimer";
}
::windows_core::imp::interface_hierarchy!(
    DispatcherTimer, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DispatcherTimer {}
unsafe impl ::core::marker::Sync for DispatcherTimer {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragEventArgs(::windows_core::IUnknown);
impl DragEventArgs {}
impl ::windows_core::RuntimeType for DragEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DragEventArgs;{b440c7c3-02b4-4980-9342-25dae1c0f188})",
    );
}
unsafe impl ::windows_core::Interface for DragEventArgs {
    type Vtable = IDragEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragEventArgs {
    const IID: ::windows_core::GUID = <IDragEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.DragEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DragEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RoutedEventArgs> for DragEventArgs {}
unsafe impl ::core::marker::Send for DragEventArgs {}
unsafe impl ::core::marker::Sync for DragEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragOperationDeferral(::windows_core::IUnknown);
impl DragOperationDeferral {}
impl ::windows_core::RuntimeType for DragOperationDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DragOperationDeferral;{ba73ecba-1b73-4086-b3d3-c223beea1633})",
    );
}
unsafe impl ::windows_core::Interface for DragOperationDeferral {
    type Vtable = IDragOperationDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragOperationDeferral {
    const IID: ::windows_core::GUID = <IDragOperationDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragOperationDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.DragOperationDeferral";
}
::windows_core::imp::interface_hierarchy!(
    DragOperationDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DragOperationDeferral {}
unsafe impl ::core::marker::Sync for DragOperationDeferral {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragStartingEventArgs(::windows_core::IUnknown);
impl DragStartingEventArgs {}
impl ::windows_core::RuntimeType for DragStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DragStartingEventArgs;{6800d3fa-90b8-46f9-8e30-5ac25f73f0f9})",
    );
}
unsafe impl ::windows_core::Interface for DragStartingEventArgs {
    type Vtable = IDragStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragStartingEventArgs {
    const IID: ::windows_core::GUID = <IDragStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.DragStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DragStartingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RoutedEventArgs> for DragStartingEventArgs {}
unsafe impl ::core::marker::Send for DragStartingEventArgs {}
unsafe impl ::core::marker::Sync for DragStartingEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragUI(::windows_core::IUnknown);
impl DragUI {}
impl ::windows_core::RuntimeType for DragUI {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DragUI;{2d9bd838-7c60-4842-9170-346fe10a226a})",
    );
}
unsafe impl ::windows_core::Interface for DragUI {
    type Vtable = IDragUI_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragUI {
    const IID: ::windows_core::GUID = <IDragUI as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragUI {
    const NAME: &'static str = "Windows.UI.Xaml.DragUI";
}
::windows_core::imp::interface_hierarchy!(
    DragUI, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DragUI {}
unsafe impl ::core::marker::Sync for DragUI {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DragUIOverride(::windows_core::IUnknown);
impl DragUIOverride {}
impl ::windows_core::RuntimeType for DragUIOverride {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DragUIOverride;{bd6c9dfa-c961-4861-b7a5-bf4fe4a8a6ef})",
    );
}
unsafe impl ::windows_core::Interface for DragUIOverride {
    type Vtable = IDragUIOverride_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragUIOverride {
    const IID: ::windows_core::GUID = <IDragUIOverride as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragUIOverride {
    const NAME: &'static str = "Windows.UI.Xaml.DragUIOverride";
}
::windows_core::imp::interface_hierarchy!(
    DragUIOverride, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DragUIOverride {}
unsafe impl ::core::marker::Sync for DragUIOverride {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DropCompletedEventArgs(::windows_core::IUnknown);
impl DropCompletedEventArgs {}
impl ::windows_core::RuntimeType for DropCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DropCompletedEventArgs;{6c4fc188-95bc-4261-9ec5-21cab677b734})",
    );
}
unsafe impl ::windows_core::Interface for DropCompletedEventArgs {
    type Vtable = IDropCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DropCompletedEventArgs {
    const IID: ::windows_core::GUID = <IDropCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DropCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.DropCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DropCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RoutedEventArgs> for DropCompletedEventArgs {}
unsafe impl ::core::marker::Send for DropCompletedEventArgs {}
unsafe impl ::core::marker::Sync for DropCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DurationHelper(::windows_core::IUnknown);
impl DurationHelper {}
impl ::windows_core::RuntimeType for DurationHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.DurationHelper;{25c1659f-4497-4135-940f-ee96f4d6e934})",
    );
}
unsafe impl ::windows_core::Interface for DurationHelper {
    type Vtable = IDurationHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DurationHelper {
    const IID: ::windows_core::GUID = <IDurationHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DurationHelper {
    const NAME: &'static str = "Windows.UI.Xaml.DurationHelper";
}
::windows_core::imp::interface_hierarchy!(
    DurationHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DurationHelper {}
unsafe impl ::core::marker::Sync for DurationHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct EffectiveViewportChangedEventArgs(::windows_core::IUnknown);
impl EffectiveViewportChangedEventArgs {}
impl ::windows_core::RuntimeType for EffectiveViewportChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.EffectiveViewportChangedEventArgs;{55ee2e81-1c18-59ed-bd3d-c4ca8fa7d190})",
    );
}
unsafe impl ::windows_core::Interface for EffectiveViewportChangedEventArgs {
    type Vtable = IEffectiveViewportChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EffectiveViewportChangedEventArgs {
    const IID: ::windows_core::GUID = <IEffectiveViewportChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EffectiveViewportChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.EffectiveViewportChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    EffectiveViewportChangedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for EffectiveViewportChangedEventArgs {}
unsafe impl ::core::marker::Sync for EffectiveViewportChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ElementFactoryGetArgs(::windows_core::IUnknown);
impl ElementFactoryGetArgs {}
impl ::windows_core::RuntimeType for ElementFactoryGetArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ElementFactoryGetArgs;{fb508774-41a3-5829-9255-cf452d041df4})",
    );
}
unsafe impl ::windows_core::Interface for ElementFactoryGetArgs {
    type Vtable = IElementFactoryGetArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ElementFactoryGetArgs {
    const IID: ::windows_core::GUID = <IElementFactoryGetArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ElementFactoryGetArgs {
    const NAME: &'static str = "Windows.UI.Xaml.ElementFactoryGetArgs";
}
::windows_core::imp::interface_hierarchy!(
    ElementFactoryGetArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ElementFactoryGetArgs {}
unsafe impl ::core::marker::Sync for ElementFactoryGetArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ElementFactoryRecycleArgs(::windows_core::IUnknown);
impl ElementFactoryRecycleArgs {}
impl ::windows_core::RuntimeType for ElementFactoryRecycleArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ElementFactoryRecycleArgs;{86f16b14-37e8-5dd8-a90c-25d3710318b0})",
    );
}
unsafe impl ::windows_core::Interface for ElementFactoryRecycleArgs {
    type Vtable = IElementFactoryRecycleArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ElementFactoryRecycleArgs {
    const IID: ::windows_core::GUID = <IElementFactoryRecycleArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ElementFactoryRecycleArgs {
    const NAME: &'static str = "Windows.UI.Xaml.ElementFactoryRecycleArgs";
}
::windows_core::imp::interface_hierarchy!(
    ElementFactoryRecycleArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ElementFactoryRecycleArgs {}
unsafe impl ::core::marker::Sync for ElementFactoryRecycleArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ElementSoundPlayer(::windows_core::IUnknown);
impl ElementSoundPlayer {}
impl ::windows_core::RuntimeType for ElementSoundPlayer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ElementSoundPlayer;{387773a5-f036-460c-9b81-f3d6ea43f6f2})",
    );
}
unsafe impl ::windows_core::Interface for ElementSoundPlayer {
    type Vtable = IElementSoundPlayer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ElementSoundPlayer {
    const IID: ::windows_core::GUID = <IElementSoundPlayer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ElementSoundPlayer {
    const NAME: &'static str = "Windows.UI.Xaml.ElementSoundPlayer";
}
::windows_core::imp::interface_hierarchy!(
    ElementSoundPlayer, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ElementSoundPlayer {}
unsafe impl ::core::marker::Sync for ElementSoundPlayer {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct EventTrigger(::windows_core::IUnknown);
impl EventTrigger {}
impl ::windows_core::RuntimeType for EventTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.EventTrigger;{def8f855-0b49-4087-b1a9-b8b38488f786})",
    );
}
unsafe impl ::windows_core::Interface for EventTrigger {
    type Vtable = IEventTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EventTrigger {
    const IID: ::windows_core::GUID = <IEventTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EventTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.EventTrigger";
}
::windows_core::imp::interface_hierarchy!(
    EventTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<TriggerBase> for EventTrigger {}
impl ::windows_core::CanTryInto<DependencyObject> for EventTrigger {}
unsafe impl ::core::marker::Send for EventTrigger {}
unsafe impl ::core::marker::Sync for EventTrigger {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ExceptionRoutedEventArgs(::windows_core::IUnknown);
impl ExceptionRoutedEventArgs {}
impl ::windows_core::RuntimeType for ExceptionRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ExceptionRoutedEventArgs;{dd9ff16a-4b62-4a6c-a49d-0671ef6136be})",
    );
}
unsafe impl ::windows_core::Interface for ExceptionRoutedEventArgs {
    type Vtable = IExceptionRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExceptionRoutedEventArgs {
    const IID: ::windows_core::GUID = <IExceptionRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ExceptionRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.ExceptionRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ExceptionRoutedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RoutedEventArgs> for ExceptionRoutedEventArgs {}
unsafe impl ::core::marker::Send for ExceptionRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ExceptionRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FrameworkElement(::windows_core::IUnknown);
impl FrameworkElement {}
impl ::windows_core::RuntimeType for FrameworkElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.FrameworkElement;{a391d09b-4a99-4b7c-9d8d-6fa5d01f6fbf})",
    );
}
unsafe impl ::windows_core::Interface for FrameworkElement {
    type Vtable = IFrameworkElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FrameworkElement {
    const IID: ::windows_core::GUID = <IFrameworkElement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FrameworkElement {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkElement";
}
::windows_core::imp::interface_hierarchy!(
    FrameworkElement, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<UIElement> for FrameworkElement {}
impl ::windows_core::CanTryInto<DependencyObject> for FrameworkElement {}
unsafe impl ::core::marker::Send for FrameworkElement {}
unsafe impl ::core::marker::Sync for FrameworkElement {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FrameworkTemplate(::windows_core::IUnknown);
impl FrameworkTemplate {}
impl ::windows_core::RuntimeType for FrameworkTemplate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.FrameworkTemplate;{a1e254d8-a446-4a27-9a9d-a0f59e1258a5})",
    );
}
unsafe impl ::windows_core::Interface for FrameworkTemplate {
    type Vtable = IFrameworkTemplate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FrameworkTemplate {
    const IID: ::windows_core::GUID = <IFrameworkTemplate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FrameworkTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkTemplate";
}
::windows_core::imp::interface_hierarchy!(
    FrameworkTemplate, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for FrameworkTemplate {}
unsafe impl ::core::marker::Send for FrameworkTemplate {}
unsafe impl ::core::marker::Sync for FrameworkTemplate {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FrameworkView(::windows_core::IUnknown);
impl FrameworkView {}
impl ::windows_core::RuntimeType for FrameworkView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.FrameworkView;{ddba664b-b603-47aa-942d-3833174f0d80})",
    );
}
unsafe impl ::windows_core::Interface for FrameworkView {
    type Vtable = IFrameworkView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FrameworkView {
    const IID: ::windows_core::GUID = <IFrameworkView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FrameworkView {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkView";
}
::windows_core::imp::interface_hierarchy!(
    FrameworkView, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FrameworkView {}
unsafe impl ::core::marker::Sync for FrameworkView {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct FrameworkViewSource(::windows_core::IUnknown);
impl FrameworkViewSource {}
impl ::windows_core::RuntimeType for FrameworkViewSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.FrameworkViewSource;{e3b077da-35ad-4b09-b5b2-27420041ba9f})",
    );
}
unsafe impl ::windows_core::Interface for FrameworkViewSource {
    type Vtable = IFrameworkViewSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FrameworkViewSource {
    const IID: ::windows_core::GUID = <IFrameworkViewSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FrameworkViewSource {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkViewSource";
}
::windows_core::imp::interface_hierarchy!(
    FrameworkViewSource, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FrameworkViewSource {}
unsafe impl ::core::marker::Sync for FrameworkViewSource {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct GridLengthHelper(::windows_core::IUnknown);
impl GridLengthHelper {}
impl ::windows_core::RuntimeType for GridLengthHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.GridLengthHelper;{7a826ce1-07a0-4083-b6d1-b1d917b976ac})",
    );
}
unsafe impl ::windows_core::Interface for GridLengthHelper {
    type Vtable = IGridLengthHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GridLengthHelper {
    const IID: ::windows_core::GUID = <IGridLengthHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GridLengthHelper {
    const NAME: &'static str = "Windows.UI.Xaml.GridLengthHelper";
}
::windows_core::imp::interface_hierarchy!(
    GridLengthHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for GridLengthHelper {}
unsafe impl ::core::marker::Sync for GridLengthHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct MediaFailedRoutedEventArgs(::windows_core::IUnknown);
impl MediaFailedRoutedEventArgs {}
impl ::windows_core::RuntimeType for MediaFailedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.MediaFailedRoutedEventArgs;{46d1fa8d-5149-4153-ba3c-b03e64ee531e})",
    );
}
unsafe impl ::windows_core::Interface for MediaFailedRoutedEventArgs {
    type Vtable = IMediaFailedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaFailedRoutedEventArgs {
    const IID: ::windows_core::GUID = <IMediaFailedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaFailedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.MediaFailedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    MediaFailedRoutedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ExceptionRoutedEventArgs>
for MediaFailedRoutedEventArgs {}
impl ::windows_core::CanTryInto<RoutedEventArgs> for MediaFailedRoutedEventArgs {}
unsafe impl ::core::marker::Send for MediaFailedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for MediaFailedRoutedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PointHelper(::windows_core::IUnknown);
impl PointHelper {}
impl ::windows_core::RuntimeType for PointHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.PointHelper;{727bdd92-64b0-49cf-a321-a9793e73e2e7})",
    );
}
unsafe impl ::windows_core::Interface for PointHelper {
    type Vtable = IPointHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointHelper {
    const IID: ::windows_core::GUID = <IPointHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointHelper {
    const NAME: &'static str = "Windows.UI.Xaml.PointHelper";
}
::windows_core::imp::interface_hierarchy!(
    PointHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PointHelper {}
unsafe impl ::core::marker::Sync for PointHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PropertyMetadata(::windows_core::IUnknown);
impl PropertyMetadata {}
impl ::windows_core::RuntimeType for PropertyMetadata {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.PropertyMetadata;{814ef30d-8d18-448a-8644-f2cb51e70380})",
    );
}
unsafe impl ::windows_core::Interface for PropertyMetadata {
    type Vtable = IPropertyMetadata_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PropertyMetadata {
    const IID: ::windows_core::GUID = <IPropertyMetadata as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PropertyMetadata {
    const NAME: &'static str = "Windows.UI.Xaml.PropertyMetadata";
}
::windows_core::imp::interface_hierarchy!(
    PropertyMetadata, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PropertyMetadata {}
unsafe impl ::core::marker::Sync for PropertyMetadata {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct PropertyPath(::windows_core::IUnknown);
impl PropertyPath {}
impl ::windows_core::RuntimeType for PropertyPath {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.PropertyPath;{300e5d8a-1ff3-4d2c-95ec-27f81debacb8})",
    );
}
unsafe impl ::windows_core::Interface for PropertyPath {
    type Vtable = IPropertyPath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PropertyPath {
    const IID: ::windows_core::GUID = <IPropertyPath as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PropertyPath {
    const NAME: &'static str = "Windows.UI.Xaml.PropertyPath";
}
::windows_core::imp::interface_hierarchy!(
    PropertyPath, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for PropertyPath {}
unsafe impl ::core::marker::Send for PropertyPath {}
unsafe impl ::core::marker::Sync for PropertyPath {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RectHelper(::windows_core::IUnknown);
impl RectHelper {}
impl ::windows_core::RuntimeType for RectHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.RectHelper;{a38781e2-4bfb-4ee2-afe5-89f31b37478d})",
    );
}
unsafe impl ::windows_core::Interface for RectHelper {
    type Vtable = IRectHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RectHelper {
    const IID: ::windows_core::GUID = <IRectHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RectHelper {
    const NAME: &'static str = "Windows.UI.Xaml.RectHelper";
}
::windows_core::imp::interface_hierarchy!(
    RectHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RectHelper {}
unsafe impl ::core::marker::Sync for RectHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ResourceDictionary(::windows_core::IUnknown);
impl ResourceDictionary {
    pub fn First(
        &self,
    ) -> ::windows_core::Result<
        super::super::Foundation::Collections::IIterator<
            super::super::Foundation::Collections::IKeyValuePair<
                ::windows_core::IInspectable,
                ::windows_core::IInspectable,
            >,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows_core::IInspectable,
                    ::windows_core::IInspectable,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .First)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceDictionary {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ResourceDictionary;{c1ea4f24-d6de-4191-8e3a-f48601f7489c})",
    );
}
unsafe impl ::windows_core::Interface for ResourceDictionary {
    type Vtable = IResourceDictionary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceDictionary {
    const IID: ::windows_core::GUID = <IResourceDictionary as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceDictionary {
    const NAME: &'static str = "Windows.UI.Xaml.ResourceDictionary";
}
impl ::core::iter::IntoIterator for ResourceDictionary {
    type Item = super::super::Foundation::Collections::IKeyValuePair<
        ::windows_core::IInspectable,
        ::windows_core::IInspectable,
    >;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &ResourceDictionary {
    type Item = super::super::Foundation::Collections::IKeyValuePair<
        ::windows_core::IInspectable,
        ::windows_core::IInspectable,
    >;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(
    ResourceDictionary, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<
        super::super::Foundation::Collections::IKeyValuePair<
            ::windows_core::IInspectable,
            ::windows_core::IInspectable,
        >,
    >,
> for ResourceDictionary {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IMap<
        ::windows_core::IInspectable,
        ::windows_core::IInspectable,
    >,
> for ResourceDictionary {}
impl ::windows_core::CanTryInto<DependencyObject> for ResourceDictionary {}
unsafe impl ::core::marker::Send for ResourceDictionary {}
unsafe impl ::core::marker::Sync for ResourceDictionary {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RoutedEvent(::windows_core::IUnknown);
impl RoutedEvent {}
impl ::windows_core::RuntimeType for RoutedEvent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.RoutedEvent;{a6b25818-43c1-4c70-865c-7bdd5a32e327})",
    );
}
unsafe impl ::windows_core::Interface for RoutedEvent {
    type Vtable = IRoutedEvent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RoutedEvent {
    const IID: ::windows_core::GUID = <IRoutedEvent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RoutedEvent {
    const NAME: &'static str = "Windows.UI.Xaml.RoutedEvent";
}
::windows_core::imp::interface_hierarchy!(
    RoutedEvent, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RoutedEvent {}
unsafe impl ::core::marker::Sync for RoutedEvent {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct RoutedEventArgs(::windows_core::IUnknown);
impl RoutedEventArgs {}
impl ::windows_core::RuntimeType for RoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.RoutedEventArgs;{5c985ac6-d802-4b38-a223-bf070c43fedf})",
    );
}
unsafe impl ::windows_core::Interface for RoutedEventArgs {
    type Vtable = IRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RoutedEventArgs {
    const IID: ::windows_core::GUID = <IRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.RoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RoutedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RoutedEventArgs {}
unsafe impl ::core::marker::Sync for RoutedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ScalarTransition(::windows_core::IUnknown);
impl ScalarTransition {}
impl ::windows_core::RuntimeType for ScalarTransition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ScalarTransition;{4cb68238-e15d-524e-a73c-9d4dcfbea226})",
    );
}
unsafe impl ::windows_core::Interface for ScalarTransition {
    type Vtable = IScalarTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScalarTransition {
    const IID: ::windows_core::GUID = <IScalarTransition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScalarTransition {
    const NAME: &'static str = "Windows.UI.Xaml.ScalarTransition";
}
::windows_core::imp::interface_hierarchy!(
    ScalarTransition, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ScalarTransition {}
unsafe impl ::core::marker::Sync for ScalarTransition {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Setter(::windows_core::IUnknown);
impl Setter {}
impl ::windows_core::RuntimeType for Setter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Setter;{a73ded29-b4ae-4a81-be85-e690ba0d3b6e})",
    );
}
unsafe impl ::windows_core::Interface for Setter {
    type Vtable = ISetter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Setter {
    const IID: ::windows_core::GUID = <ISetter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Setter {
    const NAME: &'static str = "Windows.UI.Xaml.Setter";
}
::windows_core::imp::interface_hierarchy!(
    Setter, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<SetterBase> for Setter {}
impl ::windows_core::CanTryInto<DependencyObject> for Setter {}
unsafe impl ::core::marker::Send for Setter {}
unsafe impl ::core::marker::Sync for Setter {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SetterBase(::windows_core::IUnknown);
impl SetterBase {}
impl ::windows_core::RuntimeType for SetterBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.SetterBase;{418be27c-2ac4-4f22-8097-dea3aeeb2fb3})",
    );
}
unsafe impl ::windows_core::Interface for SetterBase {
    type Vtable = ISetterBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SetterBase {
    const IID: ::windows_core::GUID = <ISetterBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SetterBase {
    const NAME: &'static str = "Windows.UI.Xaml.SetterBase";
}
::windows_core::imp::interface_hierarchy!(
    SetterBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for SetterBase {}
unsafe impl ::core::marker::Send for SetterBase {}
unsafe impl ::core::marker::Sync for SetterBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SetterBaseCollection(::windows_core::IUnknown);
impl SetterBaseCollection {}
impl ::windows_core::RuntimeType for SetterBaseCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.SetterBaseCollection;{03c40ca8-909e-4117-811c-a4529496bdf1})",
    );
}
unsafe impl ::windows_core::Interface for SetterBaseCollection {
    type Vtable = ISetterBaseCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SetterBaseCollection {
    const IID: ::windows_core::GUID = <ISetterBaseCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SetterBaseCollection {
    const NAME: &'static str = "Windows.UI.Xaml.SetterBaseCollection";
}
impl ::core::iter::IntoIterator for SetterBaseCollection {
    type Item = SetterBase;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &SetterBaseCollection {
    type Item = SetterBase;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    SetterBaseCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<SetterBase>,
> for SetterBaseCollection {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IVector<SetterBase>,
> for SetterBaseCollection {}
unsafe impl ::core::marker::Send for SetterBaseCollection {}
unsafe impl ::core::marker::Sync for SetterBaseCollection {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SizeChangedEventArgs(::windows_core::IUnknown);
impl SizeChangedEventArgs {}
impl ::windows_core::RuntimeType for SizeChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.SizeChangedEventArgs;{d5312e60-5cc1-42a1-920c-1af46be2f986})",
    );
}
unsafe impl ::windows_core::Interface for SizeChangedEventArgs {
    type Vtable = ISizeChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SizeChangedEventArgs {
    const IID: ::windows_core::GUID = <ISizeChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.SizeChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    SizeChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RoutedEventArgs> for SizeChangedEventArgs {}
unsafe impl ::core::marker::Send for SizeChangedEventArgs {}
unsafe impl ::core::marker::Sync for SizeChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct SizeHelper(::windows_core::IUnknown);
impl SizeHelper {}
impl ::windows_core::RuntimeType for SizeHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.SizeHelper;{e7225a94-5d03-4a03-ba94-967fc68fcefe})",
    );
}
unsafe impl ::windows_core::Interface for SizeHelper {
    type Vtable = ISizeHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SizeHelper {
    const IID: ::windows_core::GUID = <ISizeHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SizeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.SizeHelper";
}
::windows_core::imp::interface_hierarchy!(
    SizeHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SizeHelper {}
unsafe impl ::core::marker::Sync for SizeHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct StateTrigger(::windows_core::IUnknown);
impl StateTrigger {}
impl ::windows_core::RuntimeType for StateTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.StateTrigger;{67adef2e-d8d9-49f7-a1fd-2e35eedd23cd})",
    );
}
unsafe impl ::windows_core::Interface for StateTrigger {
    type Vtable = IStateTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StateTrigger {
    const IID: ::windows_core::GUID = <IStateTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StateTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.StateTrigger";
}
::windows_core::imp::interface_hierarchy!(
    StateTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<StateTriggerBase> for StateTrigger {}
impl ::windows_core::CanTryInto<DependencyObject> for StateTrigger {}
unsafe impl ::core::marker::Send for StateTrigger {}
unsafe impl ::core::marker::Sync for StateTrigger {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct StateTriggerBase(::windows_core::IUnknown);
impl StateTriggerBase {}
impl ::windows_core::RuntimeType for StateTriggerBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.StateTriggerBase;{48b20698-af06-466c-8052-93666dde0e49})",
    );
}
unsafe impl ::windows_core::Interface for StateTriggerBase {
    type Vtable = IStateTriggerBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StateTriggerBase {
    const IID: ::windows_core::GUID = <IStateTriggerBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StateTriggerBase {
    const NAME: &'static str = "Windows.UI.Xaml.StateTriggerBase";
}
::windows_core::imp::interface_hierarchy!(
    StateTriggerBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for StateTriggerBase {}
unsafe impl ::core::marker::Send for StateTriggerBase {}
unsafe impl ::core::marker::Sync for StateTriggerBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Style(::windows_core::IUnknown);
impl Style {}
impl ::windows_core::RuntimeType for Style {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Style;{c4a9f225-9db7-4a7d-b6d1-f74edb9293c2})",
    );
}
unsafe impl ::windows_core::Interface for Style {
    type Vtable = IStyle_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Style {
    const IID: ::windows_core::GUID = <IStyle as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Style {
    const NAME: &'static str = "Windows.UI.Xaml.Style";
}
::windows_core::imp::interface_hierarchy!(
    Style, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for Style {}
unsafe impl ::core::marker::Send for Style {}
unsafe impl ::core::marker::Sync for Style {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct TargetPropertyPath(::windows_core::IUnknown);
impl TargetPropertyPath {}
impl ::windows_core::RuntimeType for TargetPropertyPath {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.TargetPropertyPath;{40740f8e-085f-4ced-be70-6f47acf15ad0})",
    );
}
unsafe impl ::windows_core::Interface for TargetPropertyPath {
    type Vtable = ITargetPropertyPath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TargetPropertyPath {
    const IID: ::windows_core::GUID = <ITargetPropertyPath as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TargetPropertyPath {
    const NAME: &'static str = "Windows.UI.Xaml.TargetPropertyPath";
}
::windows_core::imp::interface_hierarchy!(
    TargetPropertyPath, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TargetPropertyPath {}
unsafe impl ::core::marker::Sync for TargetPropertyPath {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ThicknessHelper(::windows_core::IUnknown);
impl ThicknessHelper {}
impl ::windows_core::RuntimeType for ThicknessHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.ThicknessHelper;{a86bae4b-1e8f-4eeb-9013-0b2838a97b34})",
    );
}
unsafe impl ::windows_core::Interface for ThicknessHelper {
    type Vtable = IThicknessHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ThicknessHelper {
    const IID: ::windows_core::GUID = <IThicknessHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ThicknessHelper {
    const NAME: &'static str = "Windows.UI.Xaml.ThicknessHelper";
}
::windows_core::imp::interface_hierarchy!(
    ThicknessHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ThicknessHelper {}
unsafe impl ::core::marker::Sync for ThicknessHelper {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct TriggerAction(::windows_core::IUnknown);
impl TriggerAction {}
impl ::windows_core::RuntimeType for TriggerAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.TriggerAction;{a2c0df02-63d5-4b46-9b83-0868d3079621})",
    );
}
unsafe impl ::windows_core::Interface for TriggerAction {
    type Vtable = ITriggerAction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TriggerAction {
    const IID: ::windows_core::GUID = <ITriggerAction as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TriggerAction {
    const NAME: &'static str = "Windows.UI.Xaml.TriggerAction";
}
::windows_core::imp::interface_hierarchy!(
    TriggerAction, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for TriggerAction {}
unsafe impl ::core::marker::Send for TriggerAction {}
unsafe impl ::core::marker::Sync for TriggerAction {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct TriggerActionCollection(::windows_core::IUnknown);
impl TriggerActionCollection {}
impl ::windows_core::RuntimeType for TriggerActionCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.TriggerActionCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.TriggerAction;{a2c0df02-63d5-4b46-9b83-0868d3079621})))",
    );
}
unsafe impl ::windows_core::Interface for TriggerActionCollection {
    type Vtable = super::super::Foundation::Collections::IVector_Vtbl<TriggerAction>;
}
unsafe impl ::windows_core::ComInterface for TriggerActionCollection {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVector<
        TriggerAction,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TriggerActionCollection {
    const NAME: &'static str = "Windows.UI.Xaml.TriggerActionCollection";
}
impl ::core::iter::IntoIterator for TriggerActionCollection {
    type Item = TriggerAction;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &TriggerActionCollection {
    type Item = TriggerAction;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    TriggerActionCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<TriggerAction>,
> for TriggerActionCollection {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IVector<TriggerAction>,
> for TriggerActionCollection {}
unsafe impl ::core::marker::Send for TriggerActionCollection {}
unsafe impl ::core::marker::Sync for TriggerActionCollection {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct TriggerBase(::windows_core::IUnknown);
impl TriggerBase {}
impl ::windows_core::RuntimeType for TriggerBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.TriggerBase;{e7ea222f-dee6-4393-a8b2-8923d641f395})",
    );
}
unsafe impl ::windows_core::Interface for TriggerBase {
    type Vtable = ITriggerBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TriggerBase {
    const IID: ::windows_core::GUID = <ITriggerBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TriggerBase {
    const NAME: &'static str = "Windows.UI.Xaml.TriggerBase";
}
::windows_core::imp::interface_hierarchy!(
    TriggerBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for TriggerBase {}
unsafe impl ::core::marker::Send for TriggerBase {}
unsafe impl ::core::marker::Sync for TriggerBase {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct TriggerCollection(::windows_core::IUnknown);
impl TriggerCollection {}
impl ::windows_core::RuntimeType for TriggerCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.TriggerCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.TriggerBase;{e7ea222f-dee6-4393-a8b2-8923d641f395})))",
    );
}
unsafe impl ::windows_core::Interface for TriggerCollection {
    type Vtable = super::super::Foundation::Collections::IVector_Vtbl<TriggerBase>;
}
unsafe impl ::windows_core::ComInterface for TriggerCollection {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVector<
        TriggerBase,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TriggerCollection {
    const NAME: &'static str = "Windows.UI.Xaml.TriggerCollection";
}
impl ::core::iter::IntoIterator for TriggerCollection {
    type Item = TriggerBase;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &TriggerCollection {
    type Item = TriggerBase;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    TriggerCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<TriggerBase>,
> for TriggerCollection {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IVector<TriggerBase>,
> for TriggerCollection {}
unsafe impl ::core::marker::Send for TriggerCollection {}
unsafe impl ::core::marker::Sync for TriggerCollection {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct UIElement(::windows_core::IUnknown);
impl UIElement {
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__)
                .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UIElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.UIElement;{676d0be9-b65c-41c6-ba40-58cf87f201c1})",
    );
}
unsafe impl ::windows_core::Interface for UIElement {
    type Vtable = IUIElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UIElement {
    const IID: ::windows_core::GUID = <IUIElement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UIElement {
    const NAME: &'static str = "Windows.UI.Xaml.UIElement";
}
::windows_core::imp::interface_hierarchy!(
    UIElement, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for UIElement {}
unsafe impl ::core::marker::Send for UIElement {}
unsafe impl ::core::marker::Sync for UIElement {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct UIElementWeakCollection(::windows_core::IUnknown);
impl UIElementWeakCollection {}
impl ::windows_core::RuntimeType for UIElementWeakCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.UIElementWeakCollection;{10341223-e66d-519e-acf8-556bd244eac3})",
    );
}
unsafe impl ::windows_core::Interface for UIElementWeakCollection {
    type Vtable = IUIElementWeakCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UIElementWeakCollection {
    const IID: ::windows_core::GUID = <IUIElementWeakCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UIElementWeakCollection {
    const NAME: &'static str = "Windows.UI.Xaml.UIElementWeakCollection";
}
impl ::core::iter::IntoIterator for UIElementWeakCollection {
    type Item = UIElement;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &UIElementWeakCollection {
    type Item = UIElement;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    UIElementWeakCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IIterable<UIElement>,
> for UIElementWeakCollection {}
impl ::windows_core::CanTryInto<
    super::super::Foundation::Collections::IVector<UIElement>,
> for UIElementWeakCollection {}
unsafe impl ::core::marker::Send for UIElementWeakCollection {}
unsafe impl ::core::marker::Sync for UIElementWeakCollection {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct UnhandledExceptionEventArgs(::windows_core::IUnknown);
impl UnhandledExceptionEventArgs {}
impl ::windows_core::RuntimeType for UnhandledExceptionEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.UnhandledExceptionEventArgs;{7230269c-054e-4cf3-86c5-be90eb6863d5})",
    );
}
unsafe impl ::windows_core::Interface for UnhandledExceptionEventArgs {
    type Vtable = IUnhandledExceptionEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UnhandledExceptionEventArgs {
    const IID: ::windows_core::GUID = <IUnhandledExceptionEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UnhandledExceptionEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.UnhandledExceptionEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    UnhandledExceptionEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for UnhandledExceptionEventArgs {}
unsafe impl ::core::marker::Sync for UnhandledExceptionEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Vector3Transition(::windows_core::IUnknown);
impl Vector3Transition {}
impl ::windows_core::RuntimeType for Vector3Transition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Vector3Transition;{d2e209dc-c4a2-5101-9a68-fa0150505589})",
    );
}
unsafe impl ::windows_core::Interface for Vector3Transition {
    type Vtable = IVector3Transition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Vector3Transition {
    const IID: ::windows_core::GUID = <IVector3Transition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Vector3Transition {
    const NAME: &'static str = "Windows.UI.Xaml.Vector3Transition";
}
::windows_core::imp::interface_hierarchy!(
    Vector3Transition, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Vector3Transition {}
unsafe impl ::core::marker::Sync for Vector3Transition {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct VisualState(::windows_core::IUnknown);
impl VisualState {}
impl ::windows_core::RuntimeType for VisualState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.VisualState;{6320affc-c31a-4450-afde-f6ea7bd1f586})",
    );
}
unsafe impl ::windows_core::Interface for VisualState {
    type Vtable = IVisualState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualState {
    const IID: ::windows_core::GUID = <IVisualState as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualState {
    const NAME: &'static str = "Windows.UI.Xaml.VisualState";
}
::windows_core::imp::interface_hierarchy!(
    VisualState, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for VisualState {}
unsafe impl ::core::marker::Send for VisualState {}
unsafe impl ::core::marker::Sync for VisualState {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct VisualStateChangedEventArgs(::windows_core::IUnknown);
impl VisualStateChangedEventArgs {}
impl ::windows_core::RuntimeType for VisualStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.VisualStateChangedEventArgs;{fe216ab1-f31f-4791-8989-c70e1d9b59ff})",
    );
}
unsafe impl ::windows_core::Interface for VisualStateChangedEventArgs {
    type Vtable = IVisualStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IVisualStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualStateChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.VisualStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    VisualStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for VisualStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for VisualStateChangedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct VisualStateGroup(::windows_core::IUnknown);
impl VisualStateGroup {}
impl ::windows_core::RuntimeType for VisualStateGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.VisualStateGroup;{e4f9d9a4-e028-44de-9b15-4929ae0a26c2})",
    );
}
unsafe impl ::windows_core::Interface for VisualStateGroup {
    type Vtable = IVisualStateGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualStateGroup {
    const IID: ::windows_core::GUID = <IVisualStateGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualStateGroup {
    const NAME: &'static str = "Windows.UI.Xaml.VisualStateGroup";
}
::windows_core::imp::interface_hierarchy!(
    VisualStateGroup, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for VisualStateGroup {}
unsafe impl ::core::marker::Send for VisualStateGroup {}
unsafe impl ::core::marker::Sync for VisualStateGroup {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct VisualStateManager(::windows_core::IUnknown);
impl VisualStateManager {}
impl ::windows_core::RuntimeType for VisualStateManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.VisualStateManager;{6fda9f9a-6fab-4112-9258-1006a3c3476e})",
    );
}
unsafe impl ::windows_core::Interface for VisualStateManager {
    type Vtable = IVisualStateManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualStateManager {
    const IID: ::windows_core::GUID = <IVisualStateManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualStateManager {
    const NAME: &'static str = "Windows.UI.Xaml.VisualStateManager";
}
::windows_core::imp::interface_hierarchy!(
    VisualStateManager, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for VisualStateManager {}
unsafe impl ::core::marker::Send for VisualStateManager {}
unsafe impl ::core::marker::Sync for VisualStateManager {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct VisualTransition(::windows_core::IUnknown);
impl VisualTransition {}
impl ::windows_core::RuntimeType for VisualTransition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.VisualTransition;{55c5905e-2bc7-400d-aaa4-1a2981491ee0})",
    );
}
unsafe impl ::windows_core::Interface for VisualTransition {
    type Vtable = IVisualTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualTransition {
    const IID: ::windows_core::GUID = <IVisualTransition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualTransition {
    const NAME: &'static str = "Windows.UI.Xaml.VisualTransition";
}
::windows_core::imp::interface_hierarchy!(
    VisualTransition, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<DependencyObject> for VisualTransition {}
unsafe impl ::core::marker::Send for VisualTransition {}
unsafe impl ::core::marker::Sync for VisualTransition {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct Window(::windows_core::IUnknown);
impl Window {}
impl ::windows_core::RuntimeType for Window {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Window;{3276167d-c9f6-462d-9de2-ae4c1fd8c2e5})",
    );
}
unsafe impl ::windows_core::Interface for Window {
    type Vtable = IWindow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Window {
    const IID: ::windows_core::GUID = <IWindow as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Window {
    const NAME: &'static str = "Windows.UI.Xaml.Window";
}
::windows_core::imp::interface_hierarchy!(
    Window, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Window {}
unsafe impl ::core::marker::Sync for Window {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct WindowCreatedEventArgs(::windows_core::IUnknown);
impl WindowCreatedEventArgs {}
impl ::windows_core::RuntimeType for WindowCreatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.WindowCreatedEventArgs;{31b71470-feff-4654-af48-9b398ab5772b})",
    );
}
unsafe impl ::windows_core::Interface for WindowCreatedEventArgs {
    type Vtable = IWindowCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowCreatedEventArgs {
    const IID: ::windows_core::GUID = <IWindowCreatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.WindowCreatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    WindowCreatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WindowCreatedEventArgs {}
unsafe impl ::core::marker::Sync for WindowCreatedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct XamlRoot(::windows_core::IUnknown);
impl XamlRoot {}
impl ::windows_core::RuntimeType for XamlRoot {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.XamlRoot;{34b50756-1696-5b6d-8e9b-c71464ccad5a})",
    );
}
unsafe impl ::windows_core::Interface for XamlRoot {
    type Vtable = IXamlRoot_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlRoot {
    const IID: ::windows_core::GUID = <IXamlRoot as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlRoot {
    const NAME: &'static str = "Windows.UI.Xaml.XamlRoot";
}
::windows_core::imp::interface_hierarchy!(
    XamlRoot, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlRoot {}
unsafe impl ::core::marker::Sync for XamlRoot {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct XamlRootChangedEventArgs(::windows_core::IUnknown);
impl XamlRootChangedEventArgs {}
impl ::windows_core::RuntimeType for XamlRootChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.XamlRootChangedEventArgs;{92d71c21-d23c-5a17-bcb8-001504b6bb19})",
    );
}
unsafe impl ::windows_core::Interface for XamlRootChangedEventArgs {
    type Vtable = IXamlRootChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlRootChangedEventArgs {
    const IID: ::windows_core::GUID = <IXamlRootChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlRootChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.XamlRootChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    XamlRootChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlRootChangedEventArgs {}
unsafe impl ::core::marker::Sync for XamlRootChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationHighContrastAdjustment(pub u32);
impl ApplicationHighContrastAdjustment {
    pub const None: Self = Self(0u32);
    pub const Auto: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ApplicationHighContrastAdjustment {}
impl ::core::clone::Clone for ApplicationHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationHighContrastAdjustment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ApplicationHighContrastAdjustment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ApplicationHighContrastAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationHighContrastAdjustment").field(&self.0).finish()
    }
}
impl ApplicationHighContrastAdjustment {}
impl ::core::ops::BitOr for ApplicationHighContrastAdjustment {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ApplicationHighContrastAdjustment {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationHighContrastAdjustment {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationHighContrastAdjustment {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ApplicationHighContrastAdjustment {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for ApplicationHighContrastAdjustment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ApplicationHighContrastAdjustment;u4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationRequiresPointerMode(pub i32);
impl ApplicationRequiresPointerMode {
    pub const Auto: Self = Self(0i32);
    pub const WhenRequested: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationRequiresPointerMode {}
impl ::core::clone::Clone for ApplicationRequiresPointerMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationRequiresPointerMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ApplicationRequiresPointerMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ApplicationRequiresPointerMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationRequiresPointerMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ApplicationRequiresPointerMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ApplicationRequiresPointerMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationTheme(pub i32);
impl ApplicationTheme {
    pub const Light: Self = Self(0i32);
    pub const Dark: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationTheme {}
impl ::core::clone::Clone for ApplicationTheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationTheme {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ApplicationTheme {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ApplicationTheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationTheme").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ApplicationTheme {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ApplicationTheme;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationTextAttributesEnum(pub i32);
impl AutomationTextAttributesEnum {
    pub const AnimationStyleAttribute: Self = Self(40000i32);
    pub const BackgroundColorAttribute: Self = Self(40001i32);
    pub const BulletStyleAttribute: Self = Self(40002i32);
    pub const CapStyleAttribute: Self = Self(40003i32);
    pub const CultureAttribute: Self = Self(40004i32);
    pub const FontNameAttribute: Self = Self(40005i32);
    pub const FontSizeAttribute: Self = Self(40006i32);
    pub const FontWeightAttribute: Self = Self(40007i32);
    pub const ForegroundColorAttribute: Self = Self(40008i32);
    pub const HorizontalTextAlignmentAttribute: Self = Self(40009i32);
    pub const IndentationFirstLineAttribute: Self = Self(40010i32);
    pub const IndentationLeadingAttribute: Self = Self(40011i32);
    pub const IndentationTrailingAttribute: Self = Self(40012i32);
    pub const IsHiddenAttribute: Self = Self(40013i32);
    pub const IsItalicAttribute: Self = Self(40014i32);
    pub const IsReadOnlyAttribute: Self = Self(40015i32);
    pub const IsSubscriptAttribute: Self = Self(40016i32);
    pub const IsSuperscriptAttribute: Self = Self(40017i32);
    pub const MarginBottomAttribute: Self = Self(40018i32);
    pub const MarginLeadingAttribute: Self = Self(40019i32);
    pub const MarginTopAttribute: Self = Self(40020i32);
    pub const MarginTrailingAttribute: Self = Self(40021i32);
    pub const OutlineStylesAttribute: Self = Self(40022i32);
    pub const OverlineColorAttribute: Self = Self(40023i32);
    pub const OverlineStyleAttribute: Self = Self(40024i32);
    pub const StrikethroughColorAttribute: Self = Self(40025i32);
    pub const StrikethroughStyleAttribute: Self = Self(40026i32);
    pub const TabsAttribute: Self = Self(40027i32);
    pub const TextFlowDirectionsAttribute: Self = Self(40028i32);
    pub const UnderlineColorAttribute: Self = Self(40029i32);
    pub const UnderlineStyleAttribute: Self = Self(40030i32);
    pub const AnnotationTypesAttribute: Self = Self(40031i32);
    pub const AnnotationObjectsAttribute: Self = Self(40032i32);
    pub const StyleNameAttribute: Self = Self(40033i32);
    pub const StyleIdAttribute: Self = Self(40034i32);
    pub const LinkAttribute: Self = Self(40035i32);
    pub const IsActiveAttribute: Self = Self(40036i32);
    pub const SelectionActiveEndAttribute: Self = Self(40037i32);
    pub const CaretPositionAttribute: Self = Self(40038i32);
    pub const CaretBidiModeAttribute: Self = Self(40039i32);
}
impl ::core::marker::Copy for AutomationTextAttributesEnum {}
impl ::core::clone::Clone for AutomationTextAttributesEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationTextAttributesEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationTextAttributesEnum {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationTextAttributesEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextAttributesEnum").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationTextAttributesEnum {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.AutomationTextAttributesEnum;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DurationType(pub i32);
impl DurationType {
    pub const Automatic: Self = Self(0i32);
    pub const TimeSpan: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for DurationType {}
impl ::core::clone::Clone for DurationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DurationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DurationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DurationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DurationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DurationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.DurationType;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementHighContrastAdjustment(pub u32);
impl ElementHighContrastAdjustment {
    pub const None: Self = Self(0u32);
    pub const Application: Self = Self(2147483648u32);
    pub const Auto: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ElementHighContrastAdjustment {}
impl ::core::clone::Clone for ElementHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementHighContrastAdjustment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementHighContrastAdjustment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementHighContrastAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementHighContrastAdjustment").field(&self.0).finish()
    }
}
impl ElementHighContrastAdjustment {}
impl ::core::ops::BitOr for ElementHighContrastAdjustment {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ElementHighContrastAdjustment {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ElementHighContrastAdjustment {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ElementHighContrastAdjustment {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ElementHighContrastAdjustment {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for ElementHighContrastAdjustment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ElementHighContrastAdjustment;u4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementSoundKind(pub i32);
impl ElementSoundKind {
    pub const Focus: Self = Self(0i32);
    pub const Invoke: Self = Self(1i32);
    pub const Show: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
    pub const MovePrevious: Self = Self(4i32);
    pub const MoveNext: Self = Self(5i32);
    pub const GoBack: Self = Self(6i32);
}
impl ::core::marker::Copy for ElementSoundKind {}
impl ::core::clone::Clone for ElementSoundKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementSoundKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementSoundKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementSoundKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementSoundKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementSoundKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ElementSoundKind;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementSoundMode(pub i32);
impl ElementSoundMode {
    pub const Default: Self = Self(0i32);
    pub const FocusOnly: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSoundMode {}
impl ::core::clone::Clone for ElementSoundMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementSoundMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementSoundMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementSoundMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementSoundMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementSoundMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ElementSoundMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementSoundPlayerState(pub i32);
impl ElementSoundPlayerState {
    pub const Auto: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSoundPlayerState {}
impl ::core::clone::Clone for ElementSoundPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementSoundPlayerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementSoundPlayerState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementSoundPlayerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementSoundPlayerState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementSoundPlayerState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ElementSoundPlayerState;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementSpatialAudioMode(pub i32);
impl ElementSpatialAudioMode {
    pub const Auto: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSpatialAudioMode {}
impl ::core::clone::Clone for ElementSpatialAudioMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementSpatialAudioMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementSpatialAudioMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementSpatialAudioMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementSpatialAudioMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementSpatialAudioMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ElementSpatialAudioMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementTheme {}
impl ::core::clone::Clone for ElementTheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementTheme {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementTheme {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementTheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementTheme").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementTheme {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.ElementTheme;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FlowDirection(pub i32);
impl FlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for FlowDirection {}
impl ::core::clone::Clone for FlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlowDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FlowDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlowDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FlowDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FlowDirection;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FocusState(pub i32);
impl FocusState {
    pub const Unfocused: Self = Self(0i32);
    pub const Pointer: Self = Self(1i32);
    pub const Keyboard: Self = Self(2i32);
    pub const Programmatic: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusState {}
impl ::core::clone::Clone for FocusState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FocusState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FocusState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FocusState;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FocusVisualKind(pub i32);
impl FocusVisualKind {
    pub const DottedLine: Self = Self(0i32);
    pub const HighVisibility: Self = Self(1i32);
    pub const Reveal: Self = Self(2i32);
}
impl ::core::marker::Copy for FocusVisualKind {}
impl ::core::clone::Clone for FocusVisualKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusVisualKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FocusVisualKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FocusVisualKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusVisualKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusVisualKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FocusVisualKind;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontCapitals(pub i32);
impl FontCapitals {
    pub const Normal: Self = Self(0i32);
    pub const AllSmallCaps: Self = Self(1i32);
    pub const SmallCaps: Self = Self(2i32);
    pub const AllPetiteCaps: Self = Self(3i32);
    pub const PetiteCaps: Self = Self(4i32);
    pub const Unicase: Self = Self(5i32);
    pub const Titling: Self = Self(6i32);
}
impl ::core::marker::Copy for FontCapitals {}
impl ::core::clone::Clone for FontCapitals {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontCapitals {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontCapitals {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontCapitals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontCapitals").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontCapitals {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontCapitals;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontEastAsianLanguage(pub i32);
impl FontEastAsianLanguage {
    pub const Normal: Self = Self(0i32);
    pub const HojoKanji: Self = Self(1i32);
    pub const Jis04: Self = Self(2i32);
    pub const Jis78: Self = Self(3i32);
    pub const Jis83: Self = Self(4i32);
    pub const Jis90: Self = Self(5i32);
    pub const NlcKanji: Self = Self(6i32);
    pub const Simplified: Self = Self(7i32);
    pub const Traditional: Self = Self(8i32);
    pub const TraditionalNames: Self = Self(9i32);
}
impl ::core::marker::Copy for FontEastAsianLanguage {}
impl ::core::clone::Clone for FontEastAsianLanguage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontEastAsianLanguage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontEastAsianLanguage {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontEastAsianLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontEastAsianLanguage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontEastAsianLanguage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontEastAsianLanguage;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontEastAsianWidths(pub i32);
impl FontEastAsianWidths {
    pub const Normal: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const Half: Self = Self(2i32);
    pub const Proportional: Self = Self(3i32);
    pub const Quarter: Self = Self(4i32);
    pub const Third: Self = Self(5i32);
}
impl ::core::marker::Copy for FontEastAsianWidths {}
impl ::core::clone::Clone for FontEastAsianWidths {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontEastAsianWidths {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontEastAsianWidths {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontEastAsianWidths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontEastAsianWidths").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontEastAsianWidths {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontEastAsianWidths;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontFraction(pub i32);
impl FontFraction {
    pub const Normal: Self = Self(0i32);
    pub const Stacked: Self = Self(1i32);
    pub const Slashed: Self = Self(2i32);
}
impl ::core::marker::Copy for FontFraction {}
impl ::core::clone::Clone for FontFraction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontFraction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontFraction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontFraction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontFraction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontFraction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontFraction;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontNumeralAlignment(pub i32);
impl FontNumeralAlignment {
    pub const Normal: Self = Self(0i32);
    pub const Proportional: Self = Self(1i32);
    pub const Tabular: Self = Self(2i32);
}
impl ::core::marker::Copy for FontNumeralAlignment {}
impl ::core::clone::Clone for FontNumeralAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontNumeralAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontNumeralAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontNumeralAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontNumeralAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontNumeralAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontNumeralAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontNumeralStyle(pub i32);
impl FontNumeralStyle {
    pub const Normal: Self = Self(0i32);
    pub const Lining: Self = Self(1i32);
    pub const OldStyle: Self = Self(2i32);
}
impl ::core::marker::Copy for FontNumeralStyle {}
impl ::core::clone::Clone for FontNumeralStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontNumeralStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontNumeralStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontNumeralStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontNumeralStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontNumeralStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontNumeralStyle;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FontVariants(pub i32);
impl FontVariants {
    pub const Normal: Self = Self(0i32);
    pub const Superscript: Self = Self(1i32);
    pub const Subscript: Self = Self(2i32);
    pub const Ordinal: Self = Self(3i32);
    pub const Inferior: Self = Self(4i32);
    pub const Ruby: Self = Self(5i32);
}
impl ::core::marker::Copy for FontVariants {}
impl ::core::clone::Clone for FontVariants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontVariants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FontVariants {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FontVariants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontVariants").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontVariants {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.FontVariants;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: Self = Self(0i32);
    pub const Pixel: Self = Self(1i32);
    pub const Star: Self = Self(2i32);
}
impl ::core::marker::Copy for GridUnitType {}
impl ::core::clone::Clone for GridUnitType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GridUnitType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GridUnitType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GridUnitType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridUnitType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GridUnitType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.GridUnitType;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl ::core::marker::Copy for HorizontalAlignment {}
impl ::core::clone::Clone for HorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HorizontalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HorizontalAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HorizontalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HorizontalAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HorizontalAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.HorizontalAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineStackingStrategy(pub i32);
impl LineStackingStrategy {
    pub const MaxHeight: Self = Self(0i32);
    pub const BlockLineHeight: Self = Self(1i32);
    pub const BaselineToBaseline: Self = Self(2i32);
}
impl ::core::marker::Copy for LineStackingStrategy {}
impl ::core::clone::Clone for LineStackingStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineStackingStrategy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LineStackingStrategy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LineStackingStrategy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineStackingStrategy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LineStackingStrategy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.LineStackingStrategy;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OpticalMarginAlignment(pub i32);
impl OpticalMarginAlignment {
    pub const None: Self = Self(0i32);
    pub const TrimSideBearings: Self = Self(1i32);
}
impl ::core::marker::Copy for OpticalMarginAlignment {}
impl ::core::clone::Clone for OpticalMarginAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OpticalMarginAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OpticalMarginAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OpticalMarginAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpticalMarginAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for OpticalMarginAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.OpticalMarginAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextAlignment(pub i32);
impl TextAlignment {
    pub const Center: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Start: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const End: Self = Self(2i32);
    pub const Justify: Self = Self(3i32);
    pub const DetectFromContent: Self = Self(4i32);
}
impl ::core::marker::Copy for TextAlignment {}
impl ::core::clone::Clone for TextAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TextAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TextAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TextAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.TextAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextLineBounds(pub i32);
impl TextLineBounds {
    pub const Full: Self = Self(0i32);
    pub const TrimToCapHeight: Self = Self(1i32);
    pub const TrimToBaseline: Self = Self(2i32);
    pub const Tight: Self = Self(3i32);
}
impl ::core::marker::Copy for TextLineBounds {}
impl ::core::clone::Clone for TextLineBounds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextLineBounds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TextLineBounds {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TextLineBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextLineBounds").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TextLineBounds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.TextLineBounds;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextReadingOrder(pub i32);
impl TextReadingOrder {
    pub const Default: Self = Self(0i32);
    pub const UseFlowDirection: Self = Self(0i32);
    pub const DetectFromContent: Self = Self(1i32);
}
impl ::core::marker::Copy for TextReadingOrder {}
impl ::core::clone::Clone for TextReadingOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextReadingOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TextReadingOrder {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TextReadingOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextReadingOrder").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TextReadingOrder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.TextReadingOrder;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextTrimming(pub i32);
impl TextTrimming {
    pub const None: Self = Self(0i32);
    pub const CharacterEllipsis: Self = Self(1i32);
    pub const WordEllipsis: Self = Self(2i32);
    pub const Clip: Self = Self(3i32);
}
impl ::core::marker::Copy for TextTrimming {}
impl ::core::clone::Clone for TextTrimming {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextTrimming {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TextTrimming {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TextTrimming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextTrimming").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TextTrimming {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.TextTrimming;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: Self = Self(1i32);
    pub const Wrap: Self = Self(2i32);
    pub const WrapWholeWords: Self = Self(3i32);
}
impl ::core::marker::Copy for TextWrapping {}
impl ::core::clone::Clone for TextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextWrapping {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TextWrapping {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TextWrapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextWrapping").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TextWrapping {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.TextWrapping;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Vector3TransitionComponents(pub u32);
impl Vector3TransitionComponents {
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for Vector3TransitionComponents {}
impl ::core::clone::Clone for Vector3TransitionComponents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Vector3TransitionComponents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Vector3TransitionComponents {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Vector3TransitionComponents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Vector3TransitionComponents").field(&self.0).finish()
    }
}
impl Vector3TransitionComponents {}
impl ::core::ops::BitOr for Vector3TransitionComponents {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for Vector3TransitionComponents {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for Vector3TransitionComponents {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for Vector3TransitionComponents {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for Vector3TransitionComponents {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for Vector3TransitionComponents {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Vector3TransitionComponents;u4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl ::core::marker::Copy for VerticalAlignment {}
impl ::core::clone::Clone for VerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VerticalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VerticalAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VerticalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VerticalAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VerticalAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.VerticalAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Visibility(pub i32);
impl Visibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
}
impl ::core::marker::Copy for Visibility {}
impl ::core::clone::Clone for Visibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Visibility {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Visibility {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Visibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Visibility").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Visibility {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Visibility;i4)",
    );
}
#[repr(C)]
pub struct CornerRadius {
    pub TopLeft: f64,
    pub TopRight: f64,
    pub BottomRight: f64,
    pub BottomLeft: f64,
}
impl ::core::marker::Copy for CornerRadius {}
impl ::core::clone::Clone for CornerRadius {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CornerRadius {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CornerRadius")
            .field("TopLeft", &self.TopLeft)
            .field("TopRight", &self.TopRight)
            .field("BottomRight", &self.BottomRight)
            .field("BottomLeft", &self.BottomLeft)
            .finish()
    }
}
impl ::windows_core::TypeKind for CornerRadius {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for CornerRadius {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.CornerRadius;f8;f8;f8;f8)",
    );
}
impl ::core::cmp::PartialEq for CornerRadius {
    fn eq(&self, other: &Self) -> bool {
        self.TopLeft == other.TopLeft && self.TopRight == other.TopRight
            && self.BottomRight == other.BottomRight
            && self.BottomLeft == other.BottomLeft
    }
}
impl ::core::cmp::Eq for CornerRadius {}
impl ::core::default::Default for CornerRadius {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Duration {
    pub TimeSpan: super::super::Foundation::TimeSpan,
    pub Type: DurationType,
}
impl ::core::marker::Copy for Duration {}
impl ::core::clone::Clone for Duration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Duration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Duration")
            .field("TimeSpan", &self.TimeSpan)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::windows_core::TypeKind for Duration {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for Duration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Duration;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.UI.Xaml.DurationType;i4))",
    );
}
impl ::core::cmp::PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.TimeSpan == other.TimeSpan && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for Duration {}
impl ::core::default::Default for Duration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GridLength {
    pub Value: f64,
    pub GridUnitType: GridUnitType,
}
impl ::core::marker::Copy for GridLength {}
impl ::core::clone::Clone for GridLength {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GridLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GridLength")
            .field("Value", &self.Value)
            .field("GridUnitType", &self.GridUnitType)
            .finish()
    }
}
impl ::windows_core::TypeKind for GridLength {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for GridLength {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.GridLength;f8;enum(Windows.UI.Xaml.GridUnitType;i4))",
    );
}
impl ::core::cmp::PartialEq for GridLength {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.GridUnitType == other.GridUnitType
    }
}
impl ::core::cmp::Eq for GridLength {}
impl ::core::default::Default for GridLength {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Thickness {
    pub Left: f64,
    pub Top: f64,
    pub Right: f64,
    pub Bottom: f64,
}
impl ::core::marker::Copy for Thickness {}
impl ::core::clone::Clone for Thickness {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Thickness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Thickness")
            .field("Left", &self.Left)
            .field("Top", &self.Top)
            .field("Right", &self.Right)
            .field("Bottom", &self.Bottom)
            .finish()
    }
}
impl ::windows_core::TypeKind for Thickness {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for Thickness {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Thickness;f8;f8;f8;f8)",
    );
}
impl ::core::cmp::PartialEq for Thickness {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Right == other.Right
            && self.Bottom == other.Bottom
    }
}
impl ::core::cmp::Eq for Thickness {}
impl ::core::default::Default for Thickness {
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
pub struct ApplicationInitializationCallback(pub ::windows_core::IUnknown);
impl ApplicationInitializationCallback {}
#[repr(C)]
struct ApplicationInitializationCallbackBox<
    F: FnMut(
            ::core::option::Option<&ApplicationInitializationCallbackParams>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const ApplicationInitializationCallback_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&ApplicationInitializationCallbackParams>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> ApplicationInitializationCallbackBox<F> {
    const VTABLE: ApplicationInitializationCallback_Vtbl = ApplicationInitializationCallback_Vtbl {
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
            == <ApplicationInitializationCallback as ::windows_core::ComInterface>::IID
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
        p: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&p)).into()
    }
}
unsafe impl ::windows_core::Interface for ApplicationInitializationCallback {
    type Vtable = ApplicationInitializationCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ApplicationInitializationCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb6351c55_c284_46e4_8310_fb0967fab76f,
    );
}
impl ::windows_core::RuntimeType for ApplicationInitializationCallback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{b6351c55-c284-46e4-8310-fb0967fab76f}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ApplicationInitializationCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        p: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct BindingFailedEventHandler(pub ::windows_core::IUnknown);
impl BindingFailedEventHandler {}
#[repr(C)]
struct BindingFailedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&BindingFailedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const BindingFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&BindingFailedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> BindingFailedEventHandlerBox<F> {
    const VTABLE: BindingFailedEventHandler_Vtbl = BindingFailedEventHandler_Vtbl {
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
            == <BindingFailedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for BindingFailedEventHandler {
    type Vtable = BindingFailedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindingFailedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x136b1782_54ba_420d_a1aa_82828721cde6,
    );
}
impl ::windows_core::RuntimeType for BindingFailedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{136b1782-54ba-420d-a1aa-82828721cde6}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct BindingFailedEventHandler_Vtbl {
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
pub struct CreateDefaultValueCallback(pub ::windows_core::IUnknown);
impl CreateDefaultValueCallback {}
#[repr(C)]
struct CreateDefaultValueCallbackBox<
    F: FnMut() -> ::windows_core::Result<::windows_core::IInspectable>
        + ::core::marker::Send + 'static,
> {
    vtable: *const CreateDefaultValueCallback_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut() -> ::windows_core::Result<::windows_core::IInspectable>
        + ::core::marker::Send + 'static,
> CreateDefaultValueCallbackBox<F> {
    const VTABLE: CreateDefaultValueCallback_Vtbl = CreateDefaultValueCallback_Vtbl {
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
            == <CreateDefaultValueCallback as ::windows_core::ComInterface>::IID
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
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        match ((*this).invoke)() {
            ::core::result::Result::Ok(ok__) => {
                ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                ::core::mem::forget(ok__);
                ::windows_core::HRESULT(0)
            }
            ::core::result::Result::Err(err) => err.into(),
        }
    }
}
unsafe impl ::windows_core::Interface for CreateDefaultValueCallback {
    type Vtable = CreateDefaultValueCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CreateDefaultValueCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd6ecb12c_15b5_4ec8_b95c_cdd208f08153,
    );
}
impl ::windows_core::RuntimeType for CreateDefaultValueCallback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{d6ecb12c-15b5-4ec8-b95c-cdd208f08153}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct CreateDefaultValueCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DependencyPropertyChangedCallback(pub ::windows_core::IUnknown);
impl DependencyPropertyChangedCallback {}
#[repr(C)]
struct DependencyPropertyChangedCallbackBox<
    F: FnMut(
            ::core::option::Option<&DependencyObject>,
            ::core::option::Option<&DependencyProperty>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DependencyPropertyChangedCallback_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&DependencyObject>,
            ::core::option::Option<&DependencyProperty>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> DependencyPropertyChangedCallbackBox<F> {
    const VTABLE: DependencyPropertyChangedCallback_Vtbl = DependencyPropertyChangedCallback_Vtbl {
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
            == <DependencyPropertyChangedCallback as ::windows_core::ComInterface>::IID
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
        dp: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&dp),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for DependencyPropertyChangedCallback {
    type Vtable = DependencyPropertyChangedCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DependencyPropertyChangedCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x45883d16_27bf_4bc1_ac26_94c1601f3a49,
    );
}
impl ::windows_core::RuntimeType for DependencyPropertyChangedCallback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{45883d16-27bf-4bc1-ac26-94c1601f3a49}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DependencyPropertyChangedCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DependencyPropertyChangedEventHandler(pub ::windows_core::IUnknown);
impl DependencyPropertyChangedEventHandler {}
#[repr(C)]
struct DependencyPropertyChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DependencyPropertyChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DependencyPropertyChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DependencyPropertyChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> DependencyPropertyChangedEventHandlerBox<F> {
    const VTABLE: DependencyPropertyChangedEventHandler_Vtbl = DependencyPropertyChangedEventHandler_Vtbl {
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
            == <DependencyPropertyChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for DependencyPropertyChangedEventHandler {
    type Vtable = DependencyPropertyChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DependencyPropertyChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x09223e5a_75be_4499_8180_1ddc005421c0,
    );
}
impl ::windows_core::RuntimeType for DependencyPropertyChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{09223e5a-75be-4499-8180-1ddc005421c0}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DependencyPropertyChangedEventHandler_Vtbl {
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
pub struct DragEventHandler(pub ::windows_core::IUnknown);
impl DragEventHandler {}
#[repr(C)]
struct DragEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DragEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DragEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> DragEventHandlerBox<F> {
    const VTABLE: DragEventHandler_Vtbl = DragEventHandler_Vtbl {
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
        *interface = if *iid == <DragEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for DragEventHandler {
    type Vtable = DragEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2ab1a205_1e73_4bcf_aabc_57b97e21961d,
    );
}
impl ::windows_core::RuntimeType for DragEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{2ab1a205-1e73-4bcf-aabc-57b97e21961d}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DragEventHandler_Vtbl {
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
pub struct ExceptionRoutedEventHandler(pub ::windows_core::IUnknown);
impl ExceptionRoutedEventHandler {}
#[repr(C)]
struct ExceptionRoutedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ExceptionRoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const ExceptionRoutedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ExceptionRoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> ExceptionRoutedEventHandlerBox<F> {
    const VTABLE: ExceptionRoutedEventHandler_Vtbl = ExceptionRoutedEventHandler_Vtbl {
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
            == <ExceptionRoutedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for ExceptionRoutedEventHandler {
    type Vtable = ExceptionRoutedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExceptionRoutedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x68e0e810_f6ea_42bc_855b_5d9b67e6a262,
    );
}
impl ::windows_core::RuntimeType for ExceptionRoutedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{68e0e810-f6ea-42bc-855b-5d9b67e6a262}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ExceptionRoutedEventHandler_Vtbl {
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
pub struct PropertyChangedCallback(pub ::windows_core::IUnknown);
impl PropertyChangedCallback {}
#[repr(C)]
struct PropertyChangedCallbackBox<
    F: FnMut(
            ::core::option::Option<&DependencyObject>,
            ::core::option::Option<&DependencyPropertyChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const PropertyChangedCallback_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&DependencyObject>,
            ::core::option::Option<&DependencyPropertyChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> PropertyChangedCallbackBox<F> {
    const VTABLE: PropertyChangedCallback_Vtbl = PropertyChangedCallback_Vtbl {
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
            == <PropertyChangedCallback as ::windows_core::ComInterface>::IID
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
        d: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&d),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for PropertyChangedCallback {
    type Vtable = PropertyChangedCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PropertyChangedCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5a9f8a25_d142_44a4_8231_fd676724f29b,
    );
}
impl ::windows_core::RuntimeType for PropertyChangedCallback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{5a9f8a25-d142-44a4-8231-fd676724f29b}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct PropertyChangedCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        d: *mut ::core::ffi::c_void,
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
pub struct RoutedEventHandler(pub ::windows_core::IUnknown);
impl RoutedEventHandler {}
#[repr(C)]
struct RoutedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const RoutedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> RoutedEventHandlerBox<F> {
    const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
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
        *interface = if *iid == <RoutedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for RoutedEventHandler {
    type Vtable = RoutedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RoutedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa856e674_b0b6_4bc3_bba8_1ba06e40d4b5,
    );
}
impl ::windows_core::RuntimeType for RoutedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{a856e674-b0b6-4bc3-bba8-1ba06e40d4b5}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct RoutedEventHandler_Vtbl {
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
pub struct SizeChangedEventHandler(pub ::windows_core::IUnknown);
impl SizeChangedEventHandler {}
#[repr(C)]
struct SizeChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&SizeChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const SizeChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&SizeChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> SizeChangedEventHandlerBox<F> {
    const VTABLE: SizeChangedEventHandler_Vtbl = SizeChangedEventHandler_Vtbl {
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
            == <SizeChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for SizeChangedEventHandler {
    type Vtable = SizeChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SizeChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1115b13c_25d2_480b_89dc_eb3dcbd6b7fa,
    );
}
impl ::windows_core::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{1115b13c-25d2-480b-89dc-eb3dcbd6b7fa}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct SizeChangedEventHandler_Vtbl {
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
pub struct UnhandledExceptionEventHandler(pub ::windows_core::IUnknown);
impl UnhandledExceptionEventHandler {}
#[repr(C)]
struct UnhandledExceptionEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&UnhandledExceptionEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const UnhandledExceptionEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&UnhandledExceptionEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> UnhandledExceptionEventHandlerBox<F> {
    const VTABLE: UnhandledExceptionEventHandler_Vtbl = UnhandledExceptionEventHandler_Vtbl {
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
            == <UnhandledExceptionEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for UnhandledExceptionEventHandler {
    type Vtable = UnhandledExceptionEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UnhandledExceptionEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9274e6bd_49a1_4958_beee_d0e19587b6e3,
    );
}
impl ::windows_core::RuntimeType for UnhandledExceptionEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{9274e6bd-49a1-4958-beee-d0e19587b6e3}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct UnhandledExceptionEventHandler_Vtbl {
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
pub struct VisualStateChangedEventHandler(pub ::windows_core::IUnknown);
impl VisualStateChangedEventHandler {}
#[repr(C)]
struct VisualStateChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&VisualStateChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const VisualStateChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&VisualStateChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> VisualStateChangedEventHandlerBox<F> {
    const VTABLE: VisualStateChangedEventHandler_Vtbl = VisualStateChangedEventHandler_Vtbl {
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
            == <VisualStateChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for VisualStateChangedEventHandler {
    type Vtable = VisualStateChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualStateChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe6d5bbd5_e029_43a6_b36d_84a81042d774,
    );
}
impl ::windows_core::RuntimeType for VisualStateChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{e6d5bbd5-e029-43a6-b36d-84a81042d774}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct VisualStateChangedEventHandler_Vtbl {
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
pub struct WindowActivatedEventHandler(pub ::windows_core::IUnknown);
impl WindowActivatedEventHandler {}
#[repr(C)]
struct WindowActivatedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::WindowActivatedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const WindowActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::WindowActivatedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> WindowActivatedEventHandlerBox<F> {
    const VTABLE: WindowActivatedEventHandler_Vtbl = WindowActivatedEventHandler_Vtbl {
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
            == <WindowActivatedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for WindowActivatedEventHandler {
    type Vtable = WindowActivatedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowActivatedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x18026348_8619_4c7b_b534_ced45d9de219,
    );
}
impl ::windows_core::RuntimeType for WindowActivatedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{18026348-8619-4c7b-b534-ced45d9de219}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct WindowActivatedEventHandler_Vtbl {
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
pub struct WindowClosedEventHandler(pub ::windows_core::IUnknown);
impl WindowClosedEventHandler {}
#[repr(C)]
struct WindowClosedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::CoreWindowEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const WindowClosedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::CoreWindowEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> WindowClosedEventHandlerBox<F> {
    const VTABLE: WindowClosedEventHandler_Vtbl = WindowClosedEventHandler_Vtbl {
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
            == <WindowClosedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for WindowClosedEventHandler {
    type Vtable = WindowClosedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowClosedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x0db89161_20d7_45df_9122_ba89576703ba,
    );
}
impl ::windows_core::RuntimeType for WindowClosedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{0db89161-20d7-45df-9122-ba89576703ba}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct WindowClosedEventHandler_Vtbl {
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
pub struct WindowSizeChangedEventHandler(pub ::windows_core::IUnknown);
impl WindowSizeChangedEventHandler {}
#[repr(C)]
struct WindowSizeChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::WindowSizeChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const WindowSizeChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::WindowSizeChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> WindowSizeChangedEventHandlerBox<F> {
    const VTABLE: WindowSizeChangedEventHandler_Vtbl = WindowSizeChangedEventHandler_Vtbl {
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
            == <WindowSizeChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for WindowSizeChangedEventHandler {
    type Vtable = WindowSizeChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowSizeChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5c21c742_2ced_4fd9_ba38_7118d40e966b,
    );
}
impl ::windows_core::RuntimeType for WindowSizeChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{5c21c742-2ced-4fd9-ba38-7118d40e966b}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct WindowSizeChangedEventHandler_Vtbl {
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
pub struct WindowVisibilityChangedEventHandler(pub ::windows_core::IUnknown);
impl WindowVisibilityChangedEventHandler {}
#[repr(C)]
struct WindowVisibilityChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::VisibilityChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const WindowVisibilityChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&super::Core::VisibilityChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> WindowVisibilityChangedEventHandlerBox<F> {
    const VTABLE: WindowVisibilityChangedEventHandler_Vtbl = WindowVisibilityChangedEventHandler_Vtbl {
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
            == <WindowVisibilityChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for WindowVisibilityChangedEventHandler {
    type Vtable = WindowVisibilityChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowVisibilityChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x10406ad6_b090_4a4a_b2ad_d682df27130f,
    );
}
impl ::windows_core::RuntimeType for WindowVisibilityChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{10406ad6-b090-4a4a-b2ad-d682df27130f}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct WindowVisibilityChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
