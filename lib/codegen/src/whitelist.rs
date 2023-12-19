use std::collections::{HashMap, HashSet};
use syn::{Expr, ImplItem, Item, parse_quote};
use crate::utils::get_indent;
use crate::WhiteList;

pub fn apply_whitelist(items: &mut Vec<Item>, white_list: &HashMap<String, WhiteList>) {
    items
        .iter_mut()
        .for_each(|item| match item {
            Item::Impl(item) if item.trait_.is_none() => {
                let ty = get_indent(&item.self_ty);
                let mut white_list = white_list.get(&ty).cloned().unwrap_or_default();
                if !white_list.all_enabled() {
                    let required = find_required_methods(&item.items);
                    if !required.is_empty() {
                        item.attrs.push(parse_quote!(#[allow(dead_code)]));
                    }
                    white_list.add(required);
                }
                clean_impl(&mut item.items, &white_list);
            }
            _ => { }
        });
}

fn find_required_methods(items: &Vec<ImplItem>) -> HashSet<String> {
    let mut required = HashSet::new();
    for item in items {
        match item {
            ImplItem::Const(item) if item.ident == "VTABLE" => {
                extract_required(&item.expr, &mut required);
            },
            _ => {  }
        }
    }
    required
}

fn extract_required(expr: &Expr, result: &mut HashSet<String>) {
    match expr {
        Expr::Struct(expr) => {
            expr.fields.iter().for_each(|e| extract_required(&e.expr, result));
        },
        Expr::Path(path) if path.path.segments[0].ident == "Self" => {
            result.insert(path.path.segments[1].ident.to_string());
        }
        _ => unreachable!()
    }
}

fn clean_impl(items: &mut Vec<ImplItem>, white_list: &WhiteList) {
    items.retain_mut(|item| match item {
        ImplItem::Fn(item) if !white_list.is_enabled(&item.sig.ident.to_string()) => false,
        _ => true
    })
}

