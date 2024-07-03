use windows::core::{HSTRING, RuntimeName};

use windows_ext::UI::Xaml::Interop::{TypeKind, TypeName};

pub trait GetTypeName {
    fn type_name() -> TypeName;
}

impl<T: RuntimeName> GetTypeName for T {
    fn type_name() -> TypeName {
        TypeName {
            Name: HSTRING::from(Self::NAME),
            Kind: TypeKind::Metadata
        }
    }
}
