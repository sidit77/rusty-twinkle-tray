#![allow(non_snake_case, clashing_extern_declarations, non_upper_case_globals, non_camel_case_types, clippy::all)]

include!("Windows/mod.rs");

pub trait FontFamilyExt: Sized {
    fn new(name: &windows_core::HSTRING) -> windows_core::Result<Self>;
}

impl FontFamilyExt for crate::UI::Xaml::Media::FontFamily {
    fn new(name: &windows_core::HSTRING) -> windows_core::Result<Self> {
        IFontFamilyFactoryFn(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::ptr::null_mut(),
                &mut Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__
            )
            .from_abi(result__)
        })
    }
}
fn IFontFamilyFactoryFn<R, F: FnOnce(&crate::UI::Xaml::Media::IFontFamilyFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
    static SHARED: ::windows_core::imp::FactoryCache<
        crate::UI::Xaml::Media::FontFamily,
        crate::UI::Xaml::Media::IFontFamilyFactory
    > = ::windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
}

#[doc(hidden)]
#[repr(C)]
pub struct IXamlSourceTransparency_Vtbl {
    pub base: windows_core::IInspectable_Vtbl,
    pub IsBackgroundTransparent: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsBackgroundTransparent: unsafe extern "system" fn(this: *mut std::ffi::c_void, bool) -> windows_core::HRESULT,
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IXamlSourceTransparency(windows_core::IUnknown);

impl IXamlSourceTransparency {

    pub fn IsBackgroundTransparent(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(self)
                .IsBackgroundTransparent)(::windows_core::Interface::as_raw(self), &mut result__)
                .from_abi(result__)
        }
    }

    pub fn SetIsBackgroundTransparent(&self, transparent: bool) -> windows_core::Result<()> {
        unsafe {
            (::windows_core::Interface::vtable(self)
                .SetIsBackgroundTransparent)(::windows_core::Interface::as_raw(self), transparent)
                .ok()
        }
    }

}

windows_core::imp::interface_hierarchy!(IXamlSourceTransparency, windows_core::IUnknown, windows_core::IInspectable);
unsafe impl windows_core::Interface for IXamlSourceTransparency {
    type Vtable = IXamlSourceTransparency_Vtbl;
}

unsafe impl windows_core::ComInterface for IXamlSourceTransparency {
    const IID: windows_core::GUID = windows_core::GUID::from_u128(0x06636c29_5a17_458d_8ea2_2422d997a922);
}


