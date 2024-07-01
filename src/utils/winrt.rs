use windows::core::{implement, Array, IInspectable, RuntimeName, RuntimeType, GUID, HSTRING};
use windows::Foundation::{DateTime, IPropertyValue_Impl, IReference, IReference_Impl, Point, PropertyType, Rect, Size, TimeSpan};
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

#[implement(IReference<T>)]
pub struct Reference<T>(T)
where
    T: RuntimeType + 'static;

impl<T: RuntimeType> Reference<T> {
    pub fn box_value(value: T) -> IReference<T> {
        Self(value).into()
    }
}

#[allow(non_snake_case, unused_variables)]
impl<T: RuntimeType + 'static> IPropertyValue_Impl for Reference<T> {
    fn Type(&self) -> windows::core::Result<PropertyType> {
        todo!()
    }

    fn IsNumericScalar(&self) -> windows::core::Result<bool> {
        todo!()
    }

    fn GetUInt8(&self) -> windows::core::Result<u8> {
        todo!()
    }

    fn GetInt16(&self) -> windows::core::Result<i16> {
        todo!()
    }

    fn GetUInt16(&self) -> windows::core::Result<u16> {
        todo!()
    }

    fn GetInt32(&self) -> windows::core::Result<i32> {
        todo!()
    }

    fn GetUInt32(&self) -> windows::core::Result<u32> {
        todo!()
    }

    fn GetInt64(&self) -> windows::core::Result<i64> {
        todo!()
    }

    fn GetUInt64(&self) -> windows::core::Result<u64> {
        todo!()
    }

    fn GetSingle(&self) -> windows::core::Result<f32> {
        todo!()
    }

    fn GetDouble(&self) -> windows::core::Result<f64> {
        todo!()
    }

    fn GetChar16(&self) -> windows::core::Result<u16> {
        todo!()
    }

    fn GetBoolean(&self) -> windows::core::Result<bool> {
        todo!()
    }

    fn GetString(&self) -> windows::core::Result<HSTRING> {
        todo!()
    }

    fn GetGuid(&self) -> windows::core::Result<GUID> {
        todo!()
    }

    fn GetDateTime(&self) -> windows::core::Result<DateTime> {
        todo!()
    }

    fn GetTimeSpan(&self) -> windows::core::Result<TimeSpan> {
        todo!()
    }

    fn GetPoint(&self) -> windows::core::Result<Point> {
        todo!()
    }

    fn GetSize(&self) -> windows::core::Result<Size> {
        todo!()
    }

    fn GetRect(&self) -> windows::core::Result<Rect> {
        todo!()
    }

    fn GetUInt8Array(&self, value: &mut Array<u8>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetInt16Array(&self, value: &mut Array<i16>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetUInt16Array(&self, value: &mut Array<u16>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetInt32Array(&self, value: &mut Array<i32>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetUInt32Array(&self, value: &mut Array<u32>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetInt64Array(&self, value: &mut Array<i64>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetUInt64Array(&self, value: &mut Array<u64>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetSingleArray(&self, value: &mut Array<f32>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetDoubleArray(&self, value: &mut Array<f64>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetChar16Array(&self, value: &mut Array<u16>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetBooleanArray(&self, value: &mut Array<bool>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetStringArray(&self, value: &mut Array<HSTRING>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetInspectableArray(&self, value: &mut Array<IInspectable>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetGuidArray(&self, value: &mut Array<GUID>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetDateTimeArray(&self, value: &mut Array<DateTime>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetTimeSpanArray(&self, value: &mut Array<TimeSpan>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetPointArray(&self, value: &mut Array<Point>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetSizeArray(&self, value: &mut Array<Size>) -> windows::core::Result<()> {
        todo!()
    }

    fn GetRectArray(&self, value: &mut Array<Rect>) -> windows::core::Result<()> {
        todo!()
    }
}

impl<T: RuntimeType + 'static> IReference_Impl<T> for Reference<T> {
    fn Value(&self) -> windows::core::Result<T> {
        Ok(self.0.clone())
    }
}
