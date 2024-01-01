#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDesktopWindowXamlSourceNative(::windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    pub unsafe fn AttachToWindow<P0>(&self, parentwnd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self)
            .AttachToWindow)(
                ::windows_core::Interface::as_raw(self),
                parentwnd.into_param().abi(),
            )
            .ok()
    }
    pub unsafe fn WindowHandle(
        &self,
    ) -> ::windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .WindowHandle)(::windows_core::Interface::as_raw(self), &mut result__)
            .from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(
    IDesktopWindowXamlSourceNative, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AttachToWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwnd: super::super::super::Foundation::HWND,
    ) -> ::windows_core::HRESULT,
    pub WindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hwnd: *mut super::super::super::Foundation::HWND,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IDesktopWindowXamlSourceNative2(::windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative2 {}
::windows_core::imp::interface_hierarchy!(
    IDesktopWindowXamlSourceNative2, ::windows_core::IUnknown,
    IDesktopWindowXamlSourceNative
);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceNative2 {
    type Vtable = IDesktopWindowXamlSourceNative2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceNative2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe3dcd8c7_3057_4692_99c3_7b7720afda31,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2_Vtbl {
    pub base__: IDesktopWindowXamlSourceNative_Vtbl,
    PreTranslateMessage: usize,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IFindReferenceTargetsCallback(::windows_core::IUnknown);
impl IFindReferenceTargetsCallback {}
::windows_core::imp::interface_hierarchy!(
    IFindReferenceTargetsCallback, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFindReferenceTargetsCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x04b3486c_4687_4229_8d14_505ab584dd88,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FoundTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IReferenceTracker(::windows_core::IUnknown);
impl IReferenceTracker {}
::windows_core::imp::interface_hierarchy!(IReferenceTracker, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTracker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x11d3b13a_180e_4789_a8be_7712882893e6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConnectFromTrackerSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisconnectFromTrackerSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddRefFromTrackerSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ReleaseFromTrackerSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PegFromTrackerSource: unsafe extern "system" fn(
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
pub struct IReferenceTrackerExtension(::windows_core::IUnknown);
impl IReferenceTrackerExtension {}
::windows_core::imp::interface_hierarchy!(
    IReferenceTrackerExtension, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerExtension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IReferenceTrackerHost(::windows_core::IUnknown);
impl IReferenceTrackerHost {}
::windows_core::imp::interface_hierarchy!(
    IReferenceTrackerHost, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x29a71c6a_3c42_4416_a39d_e2825a07a773,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: XAML_REFERENCETRACKER_DISCONNECT,
    ) -> ::windows_core::HRESULT,
    pub ReleaseDisconnectedReferenceSources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unknown: *mut ::core::ffi::c_void,
        newreference: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows_core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IReferenceTrackerManager(::windows_core::IUnknown);
impl IReferenceTrackerManager {}
::windows_core::imp::interface_hierarchy!(
    IReferenceTrackerManager, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReferenceTrackingStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        findfailed: u8,
    ) -> ::windows_core::HRESULT,
    pub ReferenceTrackingCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IReferenceTrackerTarget(::windows_core::IUnknown);
impl IReferenceTrackerTarget {}
::windows_core::imp::interface_hierarchy!(
    IReferenceTrackerTarget, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x64bd43f8_bfee_4ec4_b7eb_2935158dae21,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddRefFromReferenceTracker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> u32,
    pub ReleaseFromReferenceTracker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> u32,
    pub Peg: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Unpeg: unsafe extern "system" fn(
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
pub struct ISurfaceImageSourceManagerNative(::windows_core::IUnknown);
impl ISurfaceImageSourceManagerNative {}
::windows_core::imp::interface_hierarchy!(
    ISurfaceImageSourceManagerNative, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceManagerNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4c8798b7_1d88_4a0f_b59b_b93f600de8c8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FlushAllSurfacesWithDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISurfaceImageSourceNative(::windows_core::IUnknown);
impl ISurfaceImageSourceNative {}
::windows_core::imp::interface_hierarchy!(
    ISurfaceImageSourceNative, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf2e9edc1_d307_4525_9886_0fafaa44163c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    SetDevice: usize,
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(
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
pub struct ISurfaceImageSourceNativeWithD2D(::windows_core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {}
::windows_core::imp::interface_hierarchy!(
    ISurfaceImageSourceNativeWithD2D, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceNativeWithD2D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x54298223_41e1_4a41_9c08_02e8256864a1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginDraw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updaterect: *const super::super::super::Foundation::RECT,
        iid: *const ::windows_core::GUID,
        updateobject: *mut *mut ::core::ffi::c_void,
        offset: *mut super::super::super::Foundation::POINT,
    ) -> ::windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(
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
pub struct ISwapChainBackgroundPanelNative(::windows_core::IUnknown);
impl ISwapChainBackgroundPanelNative {}
::windows_core::imp::interface_hierarchy!(
    ISwapChainBackgroundPanelNative, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISwapChainBackgroundPanelNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x43bebd4e_add5_4035_8f85_5608d08e9dc9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    SetSwapChain: usize,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISwapChainPanelNative(::windows_core::IUnknown);
impl ISwapChainPanelNative {}
::windows_core::imp::interface_hierarchy!(
    ISwapChainPanelNative, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISwapChainPanelNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf92f19d2_3ade_45a6_a20c_f6f1ea90554b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    SetSwapChain: usize,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ISwapChainPanelNative2(::windows_core::IUnknown);
impl ISwapChainPanelNative2 {}
::windows_core::imp::interface_hierarchy!(
    ISwapChainPanelNative2, ::windows_core::IUnknown, ISwapChainPanelNative
);
unsafe impl ::windows_core::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISwapChainPanelNative2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd5a2f60c_37b2_44a2_937b_8d8eb9726821,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_Vtbl {
    pub base__: ISwapChainPanelNative_Vtbl,
    pub SetSwapChainHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        swapchainhandle: super::super::super::Foundation::HANDLE,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct ITrackerOwner(::windows_core::IUnknown);
impl ITrackerOwner {}
::windows_core::imp::interface_hierarchy!(ITrackerOwner, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITrackerOwner {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xeb24c20b_9816_4ac7_8cff_36f67a118f4e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        returnvalue: *mut TrackerHandle,
    ) -> ::windows_core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: TrackerHandle,
    ) -> ::windows_core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: TrackerHandle,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: TrackerHandle,
        returnvalue: *mut *mut ::core::ffi::c_void,
    ) -> u8,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IVirtualSurfaceImageSourceNative(::windows_core::IUnknown);
impl IVirtualSurfaceImageSourceNative {}
::windows_core::imp::interface_hierarchy!(
    IVirtualSurfaceImageSourceNative, ::windows_core::IUnknown, ISurfaceImageSourceNative
);
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualSurfaceImageSourceNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe9550983_360b_4f53_b391_afd695078691,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_Vtbl {
    pub base__: ISurfaceImageSourceNative_Vtbl,
    pub Invalidate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updaterect: super::super::super::Foundation::RECT,
    ) -> ::windows_core::HRESULT,
    pub GetUpdateRectCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub GetUpdateRects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updates: *mut super::super::super::Foundation::RECT,
        count: u32,
    ) -> ::windows_core::HRESULT,
    pub GetVisibleBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bounds: *mut super::super::super::Foundation::RECT,
    ) -> ::windows_core::HRESULT,
    pub RegisterForUpdatesNeeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newwidth: i32,
        newheight: i32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IVirtualSurfaceUpdatesCallbackNative(::windows_core::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {}
::windows_core::imp::interface_hierarchy!(
    IVirtualSurfaceUpdatesCallbackNative, ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualSurfaceUpdatesCallbackNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdbf2e947_8e6c_4254_9eee_7738f71386c9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub UpdatesNeeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(
    0i32,
);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(
    1i32,
);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XAML_REFERENCETRACKER_DISCONNECT(pub i32);
impl ::core::marker::Copy for XAML_REFERENCETRACKER_DISCONNECT {}
impl ::core::clone::Clone for XAML_REFERENCETRACKER_DISCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XAML_REFERENCETRACKER_DISCONNECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XAML_REFERENCETRACKER_DISCONNECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XAML_REFERENCETRACKER_DISCONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XAML_REFERENCETRACKER_DISCONNECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TrackerHandle(pub isize);
impl ::core::default::Default for TrackerHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for TrackerHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for TrackerHandle {}
impl ::core::fmt::Debug for TrackerHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrackerHandle").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for TrackerHandle {
    type TypeKind = ::windows_core::CopyType;
}
