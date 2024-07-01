#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IBindableIterable(::windows_core::IUnknown);
impl IBindableIterable {}
::windows_core::imp::interface_hierarchy!(
    IBindableIterable, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IBindableIterable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{036d2c08-df29-41af-8aa2-d774be62ba6f}",
    );
}
unsafe impl ::windows_core::Interface for IBindableIterable {
    type Vtable = IBindableIterable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableIterable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x036d2c08_df29_41af_8aa2_d774be62ba6f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterable_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
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
pub struct IBindableIterator(::windows_core::IUnknown);
impl IBindableIterator {}
::windows_core::imp::interface_hierarchy!(
    IBindableIterator, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IBindableIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{6a1d6c07-076d-49f2-8314-f52c9c9a8331}",
    );
}
unsafe impl ::windows_core::Interface for IBindableIterator {
    type Vtable = IBindableIterator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableIterator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6a1d6c07_076d_49f2_8314_f52c9c9a8331,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IBindableObservableVector(::windows_core::IUnknown);
impl IBindableObservableVector {}
::windows_core::imp::interface_hierarchy!(
    IBindableObservableVector, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IBindableIterable> for IBindableObservableVector {}
impl ::windows_core::CanTryInto<IBindableVector> for IBindableObservableVector {}
impl ::windows_core::RuntimeType for IBindableObservableVector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{fe1eb536-7e7f-4f90-ac9a-474984aae512}",
    );
}
unsafe impl ::windows_core::Interface for IBindableObservableVector {
    type Vtable = IBindableObservableVector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableObservableVector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfe1eb536_7e7f_4f90_ac9a_474984aae512,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableObservableVector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VectorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct IBindableVector(::windows_core::IUnknown);
impl IBindableVector {}
::windows_core::imp::interface_hierarchy!(
    IBindableVector, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IBindableIterable> for IBindableVector {}
impl ::windows_core::RuntimeType for IBindableVector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{393de7de-6fd0-4c0d-bb71-47244a113e93}",
    );
}
unsafe impl ::windows_core::Interface for IBindableVector {
    type Vtable = IBindableVector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableVector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x393de7de_6fd0_4c0d_bb71_47244a113e93,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut u32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
    ) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(
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
pub struct IBindableVectorView(::windows_core::IUnknown);
impl IBindableVectorView {}
::windows_core::imp::interface_hierarchy!(
    IBindableVectorView, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IBindableIterable> for IBindableVectorView {}
impl ::windows_core::RuntimeType for IBindableVectorView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{346dd6e7-976e-4bc3-815d-ece243bc0f33}",
    );
}
unsafe impl ::windows_core::Interface for IBindableVectorView {
    type Vtable = IBindableVectorView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableVectorView {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x346dd6e7_976e_4bc3_815d_ece243bc0f33,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVectorView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut u32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::fmt::Debug,
    ::core::clone::Clone
)]
pub struct INotifyCollectionChanged(::windows_core::IUnknown);
impl INotifyCollectionChanged {}
::windows_core::imp::interface_hierarchy!(
    INotifyCollectionChanged, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for INotifyCollectionChanged {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{28b167d5-1a31-465b-9b25-d5c3ae686c40}",
    );
}
unsafe impl ::windows_core::Interface for INotifyCollectionChanged {
    type Vtable = INotifyCollectionChanged_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyCollectionChanged {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x28b167d5_1a31_465b_9b25_d5c3ae686c40,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChanged_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CollectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveCollectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
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
pub struct INotifyCollectionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyCollectionChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4cf68d33_e3f2_4964_b85e_945b4f7e2f21,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut NotifyCollectionChangedAction,
    ) -> ::windows_core::HRESULT,
    pub NewItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OldItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NewStartingIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub OldStartingIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
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
pub struct INotifyCollectionChangedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotifyCollectionChangedEventArgsFactory {
    type Vtable = INotifyCollectionChangedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyCollectionChangedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb30c3e3a_df8d_44a5_9a38_7ac0d08ce63d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithAllParameters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        action: NotifyCollectionChangedAction,
        newitems: *mut ::core::ffi::c_void,
        olditems: *mut ::core::ffi::c_void,
        newindex: i32,
        oldindex: i32,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
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
pub struct NotifyCollectionChangedEventArgs(::windows_core::IUnknown);
impl NotifyCollectionChangedEventArgs {}
impl ::windows_core::RuntimeType for NotifyCollectionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs;{4cf68d33-e3f2-4964-b85e-945b4f7e2f21})",
    );
}
unsafe impl ::windows_core::Interface for NotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotifyCollectionChangedEventArgs {
    const IID: ::windows_core::GUID = <INotifyCollectionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NotifyCollectionChangedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NotifyCollectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for NotifyCollectionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const Replace: Self = Self(2i32);
    pub const Move: Self = Self(3i32);
    pub const Reset: Self = Self(4i32);
}
impl ::core::marker::Copy for NotifyCollectionChangedAction {}
impl ::core::clone::Clone for NotifyCollectionChangedAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotifyCollectionChangedAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NotifyCollectionChangedAction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NotifyCollectionChangedAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedAction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NotifyCollectionChangedAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Interop.NotifyCollectionChangedAction;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0i32);
    pub const Metadata: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for TypeKind {}
impl ::core::clone::Clone for TypeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TypeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TypeKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TypeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TypeKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TypeKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)",
    );
}
#[repr(C)]
pub struct TypeName {
    pub Name: ::windows_core::HSTRING,
    pub Kind: TypeKind,
}
impl ::core::clone::Clone for TypeName {
    fn clone(&self) -> Self {
        Self {
            Name: self.Name.clone(),
            Kind: self.Kind,
        }
    }
}
impl ::core::fmt::Debug for TypeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TypeName")
            .field("Name", &self.Name)
            .field("Kind", &self.Kind)
            .finish()
    }
}
impl ::windows_core::TypeKind for TypeName {
    type TypeKind = ::windows_core::ValueType;
}
impl ::windows_core::RuntimeType for TypeName {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))",
    );
}
impl ::core::cmp::PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Kind == other.Kind
    }
}
impl ::core::cmp::Eq for TypeName {}
impl ::core::default::Default for TypeName {
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
pub struct BindableVectorChangedEventHandler(pub ::windows_core::IUnknown);
impl BindableVectorChangedEventHandler {}
#[repr(C)]
struct BindableVectorChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&IBindableObservableVector>,
            ::core::option::Option<&::windows_core::IInspectable>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const BindableVectorChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&IBindableObservableVector>,
            ::core::option::Option<&::windows_core::IInspectable>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> BindableVectorChangedEventHandlerBox<F> {
    const VTABLE: BindableVectorChangedEventHandler_Vtbl = BindableVectorChangedEventHandler_Vtbl {
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
            == <BindableVectorChangedEventHandler as ::windows_core::ComInterface>::IID
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
        vector: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this)
            .invoke)(
                ::windows_core::from_raw_borrowed(&vector),
                ::windows_core::from_raw_borrowed(&e),
            )
            .into()
    }
}
unsafe impl ::windows_core::Interface for BindableVectorChangedEventHandler {
    type Vtable = BindableVectorChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindableVectorChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x624cd4e1_d007_43b1_9c03_af4d3e6258c4,
    );
}
impl ::windows_core::RuntimeType for BindableVectorChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{624cd4e1-d007-43b1-9c03-af4d3e6258c4}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct BindableVectorChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        vector: *mut ::core::ffi::c_void,
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
pub struct NotifyCollectionChangedEventHandler(pub ::windows_core::IUnknown);
impl NotifyCollectionChangedEventHandler {}
#[repr(C)]
struct NotifyCollectionChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NotifyCollectionChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const NotifyCollectionChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NotifyCollectionChangedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> NotifyCollectionChangedEventHandlerBox<F> {
    const VTABLE: NotifyCollectionChangedEventHandler_Vtbl = NotifyCollectionChangedEventHandler_Vtbl {
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
            == <NotifyCollectionChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for NotifyCollectionChangedEventHandler {
    type Vtable = NotifyCollectionChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotifyCollectionChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xca10b37c_f382_4591_8557_5e24965279b0,
    );
}
impl ::windows_core::RuntimeType for NotifyCollectionChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{ca10b37c-f382-4591-8557-5e24965279b0}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct NotifyCollectionChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
