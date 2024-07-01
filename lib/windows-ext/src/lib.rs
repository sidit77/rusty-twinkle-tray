#![allow(non_snake_case, clashing_extern_declarations, non_upper_case_globals, non_camel_case_types, clippy::all)]

include!("Windows/mod.rs");

use windows_core::{Result, HSTRING};

use crate::UI::Xaml::Media::{FontFamily, IFontFamilyFactory};

pub trait FontFamilyExt: Sized {
    fn new(name: &HSTRING) -> Result<Self>;
}

impl FontFamilyExt for FontFamily {
    fn new(name: &HSTRING) -> Result<Self> {
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
fn IFontFamilyFactoryFn<R, F: FnOnce(&IFontFamilyFactory) -> Result<R>>(callback: F) -> Result<R> {
    static SHARED: ::windows_core::imp::FactoryCache<FontFamily, IFontFamilyFactory> = ::windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
}
