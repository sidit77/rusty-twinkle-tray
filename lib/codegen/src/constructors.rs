use std::collections::HashSet;
use syn::{Item, parse_quote};
use quote::format_ident;
use crate::utils::get_indent;

pub fn generate_constructors(items: &mut Vec<Item>, classes: &HashSet<String>) {
    items
        .iter_mut()
        .for_each(|item| match item {
            Item::Impl(item) if item.trait_.is_none() => {
                let ty = get_indent(&item.self_ty);

                if classes.contains(&ty) {
                    let class = format_ident!("{}", ty);
                    let factory = format_ident!("I{}Factory", ty);
                    item.items.push(parse_quote! {
                        pub fn new() -> ::windows_core::Result<#class> {
                            Self::#factory(|this| unsafe {
                                let mut result__ = ::std::mem::zeroed();
                                (::windows_core::Interface::vtable(this).CreateInstance)(
                                    ::windows_core::Interface::as_raw(this),
                                    ::core::ptr::null_mut(),
                                    &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                                    &mut result__,
                                )
                                .from_abi(result__)
                            })
                        }
                    });

                    item.items.push(parse_quote! {
                        #[doc(hidden)]
                        pub fn #factory <
                            R,
                            F: FnOnce(&#factory) -> ::windows_core::Result<R>,
                        >(
                            callback: F,
                        ) -> ::windows_core::Result<R> {
                            static SHARED: ::windows_core::imp::FactoryCache<
                                #class,
                                #factory,
                            > = ::windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    })
                }
            }
            _ => { }
        });
}