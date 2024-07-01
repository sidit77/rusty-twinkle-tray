use proc_macro2::Ident;
use syn::{Path, Type};

pub trait IterExpect<T> {
    fn continue_if(self, func: impl FnOnce(T) -> bool) -> Option<Self>
    where
        Self: Sized;
}

impl<T, I: Iterator<Item = T>> IterExpect<T> for I {
    fn continue_if(mut self, func: impl FnOnce(T) -> bool) -> Option<Self>
    where
        Self: Sized
    {
        self.next().and_then(|i| func(i).then_some(self))
    }
}

pub fn get_indent(ty: &Type) -> String {
    match ty {
        Type::Path(path) => simple_ident(&path.path)
            .unwrap_or_else(|| panic!("Not an identifier: {:#?}", path))
            .to_string(),
        _ => panic!("Not a path")
    }
}

pub fn simple_ident(path: &Path) -> Option<&Ident> {
    if path.leading_colon.is_none() && path.segments.len() == 1 {
        Some(&path.segments[0].ident)
    } else {
        None
    }
}
