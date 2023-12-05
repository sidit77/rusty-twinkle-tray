use std::collections::HashSet;
use syn::{Attribute, Expr, ExprLit, Fields, ImplItem, Item, Lit, Meta, Token};
use syn::punctuated::Punctuated;

pub fn strip_attributes(items: &mut Vec<Item>, enabled: &HashSet<String>, encountered: &mut HashSet<String>) {
    items.retain_mut(|item| match item {
        Item::Struct(item) => {
            process_attribute_list(&mut item.attrs, enabled, encountered)
                .then(|| process_fields(&mut item.fields, enabled, encountered))
                .is_some()
        },
        Item::Impl(item) => {
            process_attribute_list(&mut item.attrs, enabled, encountered)
                .then(|| item.items.retain_mut(|item| strip_item_attribute(item, enabled, encountered)))
                .is_some()
        },
        Item::Mod(item) => {
            process_attribute_list(&mut item.attrs, enabled, encountered)
        },
        Item::Macro(item) => {
            process_attribute_list(&mut item.attrs, enabled, encountered)
        }
        Item::Trait(item) => {
            process_attribute_list(&mut item.attrs, enabled, encountered)
        }
        _ => true
    })
}

fn strip_item_attribute(item: &mut ImplItem, enabled: &HashSet<String>, encountered: &mut HashSet<String>) -> bool {
    match item {
        ImplItem::Const(item) => process_attribute_list(&mut item.attrs, enabled, encountered),
        ImplItem::Fn(item) => process_attribute_list(&mut item.attrs, enabled, encountered),
        ImplItem::Type(item) => process_attribute_list(&mut item.attrs, enabled, encountered),
        ImplItem::Macro(item) => process_attribute_list(&mut item.attrs, enabled, encountered),
        _ => true,
    }
}

fn process_fields(field: &mut Fields, enabled: &HashSet<String>, encountered: &mut HashSet<String>) {
    match field {
        Fields::Named(field) => {
            field.named = std::mem::take(&mut field.named)
                .into_pairs()
                .filter_map(|mut p| process_attribute_list(&mut p.value_mut().attrs, enabled, encountered)
                    .then_some(p))
                .collect();
        }
        Fields::Unnamed(field) => {
            field.unnamed = std::mem::take(&mut field.unnamed)
                .into_pairs()
                .filter_map(|mut p| process_attribute_list(&mut p.value_mut().attrs, enabled, encountered)
                    .then_some(p))
                .collect();
        }
        Fields::Unit => {}
    }
}

fn process_attribute_list(attributes: &mut Vec<Attribute>, enabled: &HashSet<String>, encountered: &mut HashSet<String>) -> bool {
    let mut disabled = false;
    attributes.retain_mut(|attrib|
        !is_doc_attrib(attrib) &&
            is_enabled(attrib, enabled, encountered)
                .map(|i| disabled |= !i)
                .is_none());
    !disabled
}

fn is_doc_attrib(attrib: &Attribute) -> bool {
    match &attrib.meta {
        Meta::NameValue(meta) => meta.path.is_ident("doc"),
        _ => false
    }
}

fn is_enabled(attrib: &Attribute, enabled: &HashSet<String>, encountered: &mut HashSet<String>) -> Option<bool> {
    attrib
        .path()
        .is_ident("cfg")
        .then(|| parse_cfg(attrib.parse_args_with(Punctuated::parse_terminated).unwrap(), enabled, encountered))
}

pub fn parse_cfg(parsed: Punctuated::<Meta, Token![,]>, enabled: &HashSet<String>, encountered: &mut HashSet<String>) -> bool {
    parsed
        .into_iter()
        .all(|meta| match meta {
            Meta::List(list) if list.path.is_ident("all") => parse_cfg(list.parse_args_with(Punctuated::parse_terminated).unwrap(), enabled, encountered),
            Meta::List(list) if list.path.is_ident("not") => !parse_cfg(list.parse_args_with(Punctuated::parse_terminated).unwrap(), enabled, encountered),
            Meta::NameValue(value) if value.path.is_ident("feature") => {
                match value.value {
                    Expr::Lit(ExprLit{ lit: Lit::Str(arg), .. }) => {
                        let val = arg.value();
                        let result = enabled.contains(&val);
                        encountered.insert(val);
                        result

                    },
                    e => panic!("unexspected expr: {:?}", e)
                }
            }
            _ => panic!("unexpected meta: {:#?}", meta)
        })
}

