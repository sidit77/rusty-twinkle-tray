#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDesignerAppExitedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesignerAppExitedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf6aac86a_0cad_410c_8f62_dc2936151c74,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppExitedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExitCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
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
pub struct IDesignerAppManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppManager {
    type Vtable = IDesignerAppManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesignerAppManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa6272caa_d5c6_40cb_abd9_27ba43831bb7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DesignerAppExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveDesignerAppExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub CreateNewViewAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        initialviewstate: DesignerAppViewState,
        initialviewsize: super::super::super::Foundation::Size,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LoadObjectIntoAppAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dllname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        classid: ::windows_core::GUID,
        initializationdata: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IDesignerAppManagerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppManagerFactory {
    type Vtable = IDesignerAppManagerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesignerAppManagerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8f9d633b_1266_4c0e_8499_0db85bbd4c43,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManagerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appusermodelid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
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
pub struct IDesignerAppView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppView {
    type Vtable = IDesignerAppView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesignerAppView {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5c777cea_dd71_4a84_a56f_dacb4b14706f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ApplicationViewId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ViewState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DesignerAppViewState,
    ) -> ::windows_core::HRESULT,
    pub ViewSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub UpdateViewAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        viewstate: DesignerAppViewState,
        viewsize: super::super::super::Foundation::Size,
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
pub struct IDesktopWindowXamlSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd585bfe1_00ff_51be_ba1d_a1329956ea0a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HasFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub NavigateFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: *mut ::core::ffi::c_void,
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
pub struct IDesktopWindowXamlSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceFactory {
    type Vtable = IDesktopWindowXamlSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5cd61dc0_2561_56e1_8e75_6e44173805e3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactory_Vtbl {
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
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceGotFocusEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
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
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface
for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface
for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
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
pub struct IElementCompositionPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementCompositionPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
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
pub struct IElementCompositionPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementCompositionPreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x08c92b38_ec99_4c55_bc85_a1c180b27646,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    GetElementVisual: usize,
    GetElementChildVisual: usize,
    SetElementChildVisual: usize,
    GetScrollViewerManipulationPropertySet: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IElementCompositionPreviewStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics2 {
    type Vtable = IElementCompositionPreviewStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementCompositionPreviewStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x24148fbb_23d6_4f37_ba0c_0733e799722d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    SetImplicitShowAnimation: usize,
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    GetPointerPositionPropertySet: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IElementCompositionPreviewStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics3 {
    type Vtable = IElementCompositionPreviewStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementCompositionPreviewStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x843bc4c3_c105_59fe_a3d1_373c1d3e6fbc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    SetAppWindowContent: usize,
    GetAppWindowContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IWindowsXamlManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsXamlManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x56096c31_1aa0_5288_8818_6e74a2dcaff5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManager_Vtbl {
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
pub struct IWindowsXamlManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsXamlManagerStatics {
    type Vtable = IWindowsXamlManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsXamlManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x28258a12_7d82_505b_b210_712b04a58882,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InitializeForCurrentThread: unsafe extern "system" fn(
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
pub struct IXamlSourceFocusNavigationRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfbb93bb5_1496_5a80_ac00_e757359755e6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut XamlSourceFocusNavigationReason,
    ) -> ::windows_core::HRESULT,
    pub HintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::GUID,
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
pub struct IXamlSourceFocusNavigationRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationRequestFactory {
    type Vtable = IXamlSourceFocusNavigationRequestFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationRequestFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe746ab8f_b4ef_5390_97e5_cc0a2779c574,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: XamlSourceFocusNavigationReason,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithHintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: XamlSourceFocusNavigationReason,
        hintrect: super::super::super::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithHintRectAndCorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: XamlSourceFocusNavigationReason,
        hintrect: super::super::super::Foundation::Rect,
        correlationid: ::windows_core::GUID,
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
pub struct IXamlSourceFocusNavigationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WasFocusMoved: unsafe extern "system" fn(
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
pub struct IXamlSourceFocusNavigationResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationResultFactory {
    type Vtable = IXamlSourceFocusNavigationResultFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationResultFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x43bbadbf_f9e1_5527_b8c5_09339ff2ca76,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusmoved: bool,
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
pub struct IXamlUIPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUIPresenter {
    type Vtable = IXamlUIPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUIPresenter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa714944a_1619_4fc6_b31b_89512ef022a2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RootElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRootElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ThemeKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetThemeKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ThemeResourcesXaml: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetThemeResourcesXaml: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        width: i32,
        height: i32,
    ) -> ::windows_core::HRESULT,
    pub Render: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Present: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IXamlUIPresenterHost(::windows_core::IUnknown);
impl IXamlUIPresenterHost {}
::windows_core::imp::interface_hierarchy!(
    IXamlUIPresenterHost, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlUIPresenterHost {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{aafb84cd-9f6d-4f80-ac2c-0e6cb9f31659}",
    );
}
unsafe impl ::windows_core::Interface for IXamlUIPresenterHost {
    type Vtable = IXamlUIPresenterHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUIPresenterHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xaafb84cd_9f6d_4f80_ac2c_0e6cb9f31659,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResolveFileResource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IXamlUIPresenterHost2(::windows_core::IUnknown);
impl IXamlUIPresenterHost2 {}
::windows_core::imp::interface_hierarchy!(
    IXamlUIPresenterHost2, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlUIPresenterHost2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{61595672-7ca4-4a21-b56a-88f4812388ca}",
    );
}
unsafe impl ::windows_core::Interface for IXamlUIPresenterHost2 {
    type Vtable = IXamlUIPresenterHost2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUIPresenterHost2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x61595672_7ca4_4a21_b56a_88f4812388ca,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetGenericXamlFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IXamlUIPresenterHost3(::windows_core::IUnknown);
impl IXamlUIPresenterHost3 {}
::windows_core::imp::interface_hierarchy!(
    IXamlUIPresenterHost3, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlUIPresenterHost3 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{b14292bf-7320-41bb-9f26-4d6fd34db45a}",
    );
}
unsafe impl ::windows_core::Interface for IXamlUIPresenterHost3 {
    type Vtable = IXamlUIPresenterHost3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUIPresenterHost3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb14292bf_7320_41bb_9f26_4d6fd34db45a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResolveDictionaryResource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dictionary: *mut ::core::ffi::c_void,
        dictionarykey: *mut ::core::ffi::c_void,
        suggestedvalue: *mut ::core::ffi::c_void,
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
pub struct IXamlUIPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUIPresenterStatics {
    type Vtable = IXamlUIPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUIPresenterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x71eaeac8_45e1_4192_85aa_3a422edd23cf,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CompleteTimelinesAutomatically: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCompleteTimelinesAutomatically: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub SetHost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        host: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NotifyWindowSizeChanged: unsafe extern "system" fn(
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
pub struct IXamlUIPresenterStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUIPresenterStatics2 {
    type Vtable = IXamlUIPresenterStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUIPresenterStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5c6b68d2_cf1c_4f53_bf09_6a745f7a9703,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetFlyoutPlacementTargetInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        placementtarget: *mut ::core::ffi::c_void,
        preferredplacement: super::Controls::Primitives::FlyoutPlacementMode,
        targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode,
        allowfallbacks: *mut bool,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub GetFlyoutPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        placementtargetbounds: super::super::super::Foundation::Rect,
        controlsize: super::super::super::Foundation::Size,
        mincontrolsize: super::super::super::Foundation::Size,
        containerrect: super::super::super::Foundation::Rect,
        targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode,
        allowfallbacks: bool,
        chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DesignerAppExitedEventArgs(::windows_core::IUnknown);
impl DesignerAppExitedEventArgs {}
impl ::windows_core::RuntimeType for DesignerAppExitedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs;{f6aac86a-0cad-410c-8f62-dc2936151c74})",
    );
}
unsafe impl ::windows_core::Interface for DesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesignerAppExitedEventArgs {
    const IID: ::windows_core::GUID = <IDesignerAppExitedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesignerAppExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DesignerAppExitedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DesignerAppExitedEventArgs {}
unsafe impl ::core::marker::Sync for DesignerAppExitedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DesignerAppManager(::windows_core::IUnknown);
impl DesignerAppManager {}
impl ::windows_core::RuntimeType for DesignerAppManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.DesignerAppManager;{a6272caa-d5c6-40cb-abd9-27ba43831bb7})",
    );
}
unsafe impl ::windows_core::Interface for DesignerAppManager {
    type Vtable = IDesignerAppManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesignerAppManager {
    const IID: ::windows_core::GUID = <IDesignerAppManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesignerAppManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppManager";
}
::windows_core::imp::interface_hierarchy!(
    DesignerAppManager, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable>
for DesignerAppManager {}
unsafe impl ::core::marker::Send for DesignerAppManager {}
unsafe impl ::core::marker::Sync for DesignerAppManager {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DesignerAppView(::windows_core::IUnknown);
impl DesignerAppView {}
impl ::windows_core::RuntimeType for DesignerAppView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.DesignerAppView;{5c777cea-dd71-4a84-a56f-dacb4b14706f})",
    );
}
unsafe impl ::windows_core::Interface for DesignerAppView {
    type Vtable = IDesignerAppView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesignerAppView {
    const IID: ::windows_core::GUID = <IDesignerAppView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesignerAppView {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppView";
}
::windows_core::imp::interface_hierarchy!(
    DesignerAppView, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable>
for DesignerAppView {}
unsafe impl ::core::marker::Send for DesignerAppView {}
unsafe impl ::core::marker::Sync for DesignerAppView {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DesktopWindowXamlSource(::windows_core::IUnknown);
impl DesktopWindowXamlSource {
    pub fn SetContent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this)
                .SetContent)(
                    ::windows_core::Interface::as_raw(this),
                    value.try_into_param()?.abi(),
                )
                .ok()
        }
    }
    pub fn new() -> ::windows_core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
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
    pub fn IDesktopWindowXamlSourceFactory<
        R,
        F: FnOnce(&IDesktopWindowXamlSourceFactory) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DesktopWindowXamlSource,
            IDesktopWindowXamlSourceFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSource;{d585bfe1-00ff-51be-ba1d-a1329956ea0a})",
    );
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopWindowXamlSource {
    const IID: ::windows_core::GUID = <IDesktopWindowXamlSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSource {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
}
::windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSource, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable>
for DesktopWindowXamlSource {}
unsafe impl ::core::marker::Send for DesktopWindowXamlSource {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSource {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(::windows_core::IUnknown);
impl DesktopWindowXamlSourceGotFocusEventArgs {}
impl ::windows_core::RuntimeType for DesktopWindowXamlSourceGotFocusEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs;{39be4849-d9cc-5b70-8f05-1ad9a4aaa342})",
    );
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopWindowXamlSourceGotFocusEventArgs {
    const IID: ::windows_core::GUID = <IDesktopWindowXamlSourceGotFocusEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSourceGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSourceGotFocusEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceGotFocusEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows_core::IUnknown);
impl DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
impl ::windows_core::RuntimeType for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs;{fe61e4b9-a7af-52b3-bdb9-c3305c0b8df2})",
    );
}
unsafe impl ::windows_core::Interface
for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface
for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const IID: ::windows_core::GUID = <IDesktopWindowXamlSourceTakeFocusRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSourceTakeFocusRequestedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ElementCompositionPreview(::windows_core::IUnknown);
impl ElementCompositionPreview {}
impl ::windows_core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.ElementCompositionPreview;{b6f1a676-cfe6-46ac-acf6-c4687bb65e60})",
    );
}
unsafe impl ::windows_core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ElementCompositionPreview {
    const IID: ::windows_core::GUID = <IElementCompositionPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.ElementCompositionPreview";
}
::windows_core::imp::interface_hierarchy!(
    ElementCompositionPreview, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct WindowsXamlManager(::windows_core::IUnknown);
impl WindowsXamlManager {
    pub fn InitializeForCurrentThread() -> ::windows_core::Result<WindowsXamlManager> {
        Self::IWindowsXamlManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .InitializeForCurrentThread)(
                    ::windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsXamlManagerStatics<
        R,
        F: FnOnce(&IWindowsXamlManagerStatics) -> ::windows_core::Result<R>,
    >(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            WindowsXamlManager,
            IWindowsXamlManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.WindowsXamlManager;{56096c31-1aa0-5288-8818-6e74a2dcaff5})",
    );
}
unsafe impl ::windows_core::Interface for WindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsXamlManager {
    const IID: ::windows_core::GUID = <IWindowsXamlManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsXamlManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.WindowsXamlManager";
}
::windows_core::imp::interface_hierarchy!(
    WindowsXamlManager, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable>
for WindowsXamlManager {}
unsafe impl ::core::marker::Send for WindowsXamlManager {}
unsafe impl ::core::marker::Sync for WindowsXamlManager {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct XamlSourceFocusNavigationRequest(::windows_core::IUnknown);
impl XamlSourceFocusNavigationRequest {}
impl ::windows_core::RuntimeType for XamlSourceFocusNavigationRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest;{fbb93bb5-1496-5a80-ac00-e757359755e6})",
    );
}
unsafe impl ::windows_core::Interface for XamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlSourceFocusNavigationRequest {
    const IID: ::windows_core::GUID = <IXamlSourceFocusNavigationRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlSourceFocusNavigationRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
}
::windows_core::imp::interface_hierarchy!(
    XamlSourceFocusNavigationRequest, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationRequest {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct XamlSourceFocusNavigationResult(::windows_core::IUnknown);
impl XamlSourceFocusNavigationResult {}
impl ::windows_core::RuntimeType for XamlSourceFocusNavigationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult;{88d55a5f-9603-5d8f-9cc7-d1c4070d9801})",
    );
}
unsafe impl ::windows_core::Interface for XamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlSourceFocusNavigationResult {
    const IID: ::windows_core::GUID = <IXamlSourceFocusNavigationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlSourceFocusNavigationResult {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
}
::windows_core::imp::interface_hierarchy!(
    XamlSourceFocusNavigationResult, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationResult {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationResult {}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct XamlUIPresenter(::windows_core::IUnknown);
impl XamlUIPresenter {}
impl ::windows_core::RuntimeType for XamlUIPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Hosting.XamlUIPresenter;{a714944a-1619-4fc6-b31b-89512ef022a2})",
    );
}
unsafe impl ::windows_core::Interface for XamlUIPresenter {
    type Vtable = IXamlUIPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlUIPresenter {
    const IID: ::windows_core::GUID = <IXamlUIPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlUIPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlUIPresenter";
}
::windows_core::imp::interface_hierarchy!(
    XamlUIPresenter, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlUIPresenter {}
unsafe impl ::core::marker::Sync for XamlUIPresenter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for DesignerAppViewState {}
impl ::core::clone::Clone for DesignerAppViewState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DesignerAppViewState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DesignerAppViewState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DesignerAppViewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppViewState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DesignerAppViewState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Hosting.DesignerAppViewState;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
    pub const Left: Self = Self(7i32);
    pub const Up: Self = Self(8i32);
    pub const Right: Self = Self(9i32);
    pub const Down: Self = Self(10i32);
}
impl ::core::marker::Copy for XamlSourceFocusNavigationReason {}
impl ::core::clone::Clone for XamlSourceFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlSourceFocusNavigationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XamlSourceFocusNavigationReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XamlSourceFocusNavigationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XamlSourceFocusNavigationReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason;i4)",
    );
}
