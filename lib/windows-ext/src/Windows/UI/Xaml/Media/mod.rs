#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrush {
    type Vtable = IAcrylicBrush_Vtbl;
}
impl ::core::clone::Clone for IAcrylicBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x79bbcf4e_cd66_4f1b_a8b6_cd6d2977c18d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BackgroundSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AcrylicBackgroundSource,
    ) -> ::windows_core::HRESULT,
    pub SetBackgroundSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AcrylicBackgroundSource,
    ) -> ::windows_core::HRESULT,
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub TintTransitionDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub SetTintTransitionDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub AlwaysUseFallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrush2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrush2 {
    type Vtable = IAcrylicBrush2_Vtbl;
}
impl ::core::clone::Clone for IAcrylicBrush2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrush2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc9645383_b19e_5ac0_86ff_3d90506dbcda,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TintLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTintLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushFactory {
    type Vtable = IAcrylicBrushFactory_Vtbl;
}
impl ::core::clone::Clone for IAcrylicBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x81a32568_f6cc_4013_8363_928ae23b7a61,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushStatics {
    type Vtable = IAcrylicBrushStatics_Vtbl;
}
impl ::core::clone::Clone for IAcrylicBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2787fd79_a3da_423f_b81a_599147971523,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BackgroundSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TintColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TintOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TintTransitionDurationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrushStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushStatics2 {
    type Vtable = IAcrylicBrushStatics2_Vtbl;
}
impl ::core::clone::Clone for IAcrylicBrushStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrushStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x129188a8_bf11_5bbc_8445_8c510e5926c0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TintLuminosityOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcSegment {
    type Vtable = IArcSegment_Vtbl;
}
impl ::core::clone::Clone for IArcSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IArcSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x07940c5f_63fb_4469_91be_f1097c168052,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub RotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub IsLargeArc: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsLargeArc: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub SweepDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SweepDirection,
    ) -> ::windows_core::HRESULT,
    pub SetSweepDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SweepDirection,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcSegmentStatics {
    type Vtable = IArcSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for IArcSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IArcSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x82348f6e_8a69_4204_9c12_7207df317643,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationAngleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsLargeArcProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SweepDirectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBezierSegment {
    type Vtable = IBezierSegment_Vtbl;
}
impl ::core::clone::Clone for IBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBezierSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xaf4bb9ee_8984_49b7_81df_3f35994b95eb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub Point2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub Point3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBezierSegmentStatics {
    type Vtable = IBezierSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for IBezierSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBezierSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc0287bac_1410_4530_8452_1c9d0ad1f341,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Point3Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapCache(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapCache {
    type Vtable = IBitmapCache_Vtbl;
}
impl ::core::clone::Clone for IBitmapCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapCache {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x79c2219e_44d2_4610_9735_9bec83809ecf,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCache_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrush {
    type Vtable = IBrush_Vtbl;
}
impl ::core::clone::Clone for IBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8806a321_1e06_422c_a1cc_01696559e021,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Opacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RelativeTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRelativeTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushFactory {
    type Vtable = IBrushFactory_Vtbl;
}
impl ::core::clone::Clone for IBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x399658a2_14fb_4b8f_83e6_6e3dab12069b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrushOverrides2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushOverrides2 {
    type Vtable = IBrushOverrides2_Vtbl;
}
impl ::core::clone::Clone for IBrushOverrides2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBrushOverrides2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd092b151_d83b_5a81_a71e_a1c7f8ad6963,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushOverrides2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    PopulatePropertyInfoOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushStatics {
    type Vtable = IBrushStatics_Vtbl;
}
impl ::core::clone::Clone for IBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe70c3102_0225_47f5_b22e_0467619f6a22,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RelativeTransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICacheMode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICacheMode {
    type Vtable = ICacheMode_Vtbl;
}
impl ::core::clone::Clone for ICacheMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICacheMode {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x98dc8b11_c6f9_4dab_b838_5fd5ec8c7350,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheMode_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICacheModeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICacheModeFactory {
    type Vtable = ICacheModeFactory_Vtbl;
}
impl ::core::clone::Clone for ICacheModeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICacheModeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xeb1f8c5b_0abb_4e70_b8a8_620d0d953ab2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheModeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransform {
    type Vtable = ICompositeTransform_Vtbl;
}
impl ::core::clone::Clone for ICompositeTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositeTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc8a4385b_f24a_4701_a265_a78846f142b9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub SkewX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetSkewX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub SkewY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetSkewY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub TranslateX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTranslateX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub TranslateY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTranslateY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransformStatics {
    type Vtable = ICompositeTransformStatics_Vtbl;
}
impl ::core::clone::Clone for ICompositeTransformStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositeTransformStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2f190c08_8266_496f_9653_a18bd4f836aa,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SkewXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SkewYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TranslateXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TranslateYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTarget {
    type Vtable = ICompositionTarget_Vtbl;
}
impl ::core::clone::Clone for ICompositionTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x26cfbff0_713c_4bec_8803_e101f7b14ed3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionTargetStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTargetStatics {
    type Vtable = ICompositionTargetStatics_Vtbl;
}
impl ::core::clone::Clone for ICompositionTargetStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionTargetStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2b1af03d_1ed2_4b59_bd00_7594ee92832b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Rendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub SurfaceContentsLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveSurfaceContentsLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionTargetStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTargetStatics3 {
    type Vtable = ICompositionTargetStatics3_Vtbl;
}
impl ::core::clone::Clone for ICompositionTargetStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionTargetStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xbc0a7cd9_6750_4708_994c_2028e0312ac8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Rendered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveRendered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEllipseGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEllipseGeometry {
    type Vtable = IEllipseGeometry_Vtbl;
}
impl ::core::clone::Clone for IEllipseGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEllipseGeometry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd4f61bba_4ea2_40d6_aa6c_8d38aa87651f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Center: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub RadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEllipseGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEllipseGeometryStatics {
    type Vtable = IEllipseGeometryStatics_Vtbl;
}
impl ::core::clone::Clone for IEllipseGeometryStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEllipseGeometryStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1744db47_f635_4b16_aee6_e052a65defb2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RadiusXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontFamily(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamily {
    type Vtable = IFontFamily_Vtbl;
}
impl ::core::clone::Clone for IFontFamily {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFontFamily {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x92467e64_d66a_4cf4_9322_3d23b3c0c361,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamily_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontFamilyFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamilyFactory {
    type Vtable = IFontFamilyFactory_Vtbl;
}
impl ::core::clone::Clone for IFontFamilyFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFontFamilyFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd5603377_3dae_4dcd_af09_f9498e9ec659,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        familyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontFamilyStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamilyStatics2 {
    type Vtable = IFontFamilyStatics2_Vtbl;
}
impl ::core::clone::Clone for IFontFamilyStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFontFamilyStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x52ad7af9_37e6_4297_a238_97fb6a408d9e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub XamlAutoFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneralTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransform {
    type Vtable = IGeneralTransform_Vtbl;
}
impl ::core::clone::Clone for IGeneralTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeneralTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa06798b7_a2ec_415f_ade2_eade9333f2c7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Inverse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TransformPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: super::super::super::Foundation::Point,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inpoint: super::super::super::Foundation::Point,
        outpoint: *mut super::super::super::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TransformBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: super::super::super::Foundation::Rect,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneralTransformFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransformFactory {
    type Vtable = IGeneralTransformFactory_Vtbl;
}
impl ::core::clone::Clone for IGeneralTransformFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeneralTransformFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7a25c930_29c4_4e31_b6f9_dedd52e4df1b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneralTransformOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransformOverrides {
    type Vtable = IGeneralTransformOverrides_Vtbl;
}
impl ::core::clone::Clone for IGeneralTransformOverrides {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeneralTransformOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4f121083_24cf_4524_90ad_8a42b1c12783,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InverseCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryTransformCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inpoint: super::super::super::Foundation::Point,
        outpoint: *mut super::super::super::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TransformBoundsCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: super::super::super::Foundation::Rect,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometry {
    type Vtable = IGeometry_Vtbl;
}
impl ::core::clone::Clone for IGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeometry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfa123889_0acd_417b_b62d_5ca1bf4dfc0e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryFactory {
    type Vtable = IGeometryFactory_Vtbl;
}
impl ::core::clone::Clone for IGeometryFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeometryFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf65daf23_d5fd_42f9_b32a_929c5a4b54e1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryGroup {
    type Vtable = IGeometryGroup_Vtbl;
}
impl ::core::clone::Clone for IGeometryGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeometryGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x55225a61_8677_4c8c_8e46_ee3dc355114b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FillRule,
    ) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FillRule,
    ) -> ::windows_core::HRESULT,
    pub Children: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryGroupStatics {
    type Vtable = IGeometryGroupStatics_Vtbl;
}
impl ::core::clone::Clone for IGeometryGroupStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeometryGroupStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x56c955f4_8496_4bb6_abf0_617b1fe78b45,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRuleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryStatics {
    type Vtable = IGeometryStatics_Vtbl;
}
impl ::core::clone::Clone for IGeometryStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeometryStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7a70aa8c_0b06_465f_b637_9a47e5a70111,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Empty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StandardFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrush {
    type Vtable = IGradientBrush_Vtbl;
}
impl ::core::clone::Clone for IGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGradientBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2166e69f_935a_4191_8e3c_1c8dfdfcdc78,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SpreadMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GradientSpreadMethod,
    ) -> ::windows_core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: GradientSpreadMethod,
    ) -> ::windows_core::HRESULT,
    pub MappingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BrushMappingMode,
    ) -> ::windows_core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: BrushMappingMode,
    ) -> ::windows_core::HRESULT,
    pub ColorInterpolationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ColorInterpolationMode,
    ) -> ::windows_core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ColorInterpolationMode,
    ) -> ::windows_core::HRESULT,
    pub GradientStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetGradientStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrushFactory {
    type Vtable = IGradientBrushFactory_Vtbl;
}
impl ::core::clone::Clone for IGradientBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGradientBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xed4779ca_45bd_4131_b625_be86e07c6112,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrushStatics {
    type Vtable = IGradientBrushStatics_Vtbl;
}
impl ::core::clone::Clone for IGradientBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGradientBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x961661f9_8bb4_4e6c_b923_b5d787e0f1a9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SpreadMethodProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MappingModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ColorInterpolationModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GradientStopsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientStop(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientStop {
    type Vtable = IGradientStop_Vtbl;
}
impl ::core::clone::Clone for IGradientStop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGradientStop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x665f44fe_2e59_4c4a_ab53_076a100ccd81,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientStopStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientStopStatics {
    type Vtable = IGradientStopStatics_Vtbl;
}
impl ::core::clone::Clone for IGradientStopStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGradientStopStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x602a6d75_6193_4fe5_8e82_c7c6f6febafd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStopStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageBrush {
    type Vtable = IImageBrush_Vtbl;
}
impl ::core::clone::Clone for IImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9fd11377_c12a_4493_bf7d_f3a8ad74b554,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ImageSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetImageSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ImageFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveImageFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub ImageOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveImageOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageBrushStatics {
    type Vtable = IImageBrushStatics_Vtbl;
}
impl ::core::clone::Clone for IImageBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1255b1b2_dd18_42e5_892c_eae30c305b8c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ImageSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageSource {
    type Vtable = IImageSource_Vtbl;
}
impl ::core::clone::Clone for IImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x737ef309_ea41_4d96_a71c_98e98efcab07,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageSourceFactory {
    type Vtable = IImageSourceFactory_Vtbl;
}
impl ::core::clone::Clone for IImageSourceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageSourceFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x297ec001_2540_4e5a_ab66_88035dd3ddb5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineGeometry {
    type Vtable = ILineGeometry_Vtbl;
}
impl ::core::clone::Clone for ILineGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILineGeometry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x30edd4a2_8fc5_40af_a7a2_c27fe7aa1363,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub EndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineGeometryStatics {
    type Vtable = ILineGeometryStatics_Vtbl;
}
impl ::core::clone::Clone for ILineGeometryStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILineGeometryStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x578ae763_5562_4ee4_8703_ea4036d891e3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineSegment {
    type Vtable = ILineSegment_Vtbl;
}
impl ::core::clone::Clone for ILineSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILineSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xef6a2e25_3ff0_4420_a411_7182a4cecb15,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineSegmentStatics {
    type Vtable = ILineSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for ILineSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILineSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9fcab141_04c0_4afb_87b3_e800b969b894,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrush {
    type Vtable = ILinearGradientBrush_Vtbl;
}
impl ::core::clone::Clone for ILinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILinearGradientBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8e96d16b_bb84_4c6f_9dbf_9d6c5c6d9c39,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub EndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrushFactory {
    type Vtable = ILinearGradientBrushFactory_Vtbl;
}
impl ::core::clone::Clone for ILinearGradientBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILinearGradientBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x0ae0861c_1e7a_4fed_9857_ea8caa798490,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithGradientStopCollectionAndAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        gradientstopcollection: *mut ::core::ffi::c_void,
        angle: f64,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrushStatics {
    type Vtable = ILinearGradientBrushStatics_Vtbl;
}
impl ::core::clone::Clone for ILinearGradientBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILinearGradientBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7af6e504_2dc3_40e3_be0b_b314c13cb991,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoadedImageSourceLoadCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILoadedImageSourceLoadCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoadedImageSourceLoadCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1ac60b1e_7837_4489_b3e5_d0d5ad0a56c4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSourceLoadCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut LoadedImageSourceLoadStatus,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoadedImageSurface(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSurface {
    type Vtable = ILoadedImageSurface_Vtbl;
}
impl ::core::clone::Clone for ILoadedImageSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoadedImageSurface {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x050c8313_6737_45ba_8531_33094febef55,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurface_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DecodedPhysicalSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub DecodedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub NaturalSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    pub LoadCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    pub RemoveLoadCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoadedImageSurfaceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSurfaceStatics {
    type Vtable = ILoadedImageSurfaceStatics_Vtbl;
}
impl ::core::clone::Clone for ILoadedImageSurfaceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoadedImageSurfaceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x22b8edf6_84ad_40ab_937d_4871613e765d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurfaceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartLoadFromUriWithSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        desiredmaxsize: super::super::super::Foundation::Size,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StartLoadFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    StartLoadFromStreamWithSize: usize,
    StartLoadFromStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DProjection {
    type Vtable = IMatrix3DProjection_Vtbl;
}
impl ::core::clone::Clone for IMatrix3DProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMatrix3DProjection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6f03e149_bfc9_4c01_b578_50338cec97fc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    ProjectionMatrix: usize,
    SetProjectionMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DProjectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DProjectionStatics {
    type Vtable = IMatrix3DProjectionStatics_Vtbl;
}
impl ::core::clone::Clone for IMatrix3DProjectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMatrix3DProjectionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xae9d5895_41ec_4e37_abaa_69f41d2f876b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixHelper {
    type Vtable = IMatrixHelper_Vtbl;
}
impl ::core::clone::Clone for IMatrixHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMatrixHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf3cf4882_06b5_48c8_9eb2_1763e9364038,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixHelperStatics {
    type Vtable = IMatrixHelperStatics_Vtbl;
}
impl ::core::clone::Clone for IMatrixHelperStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMatrixHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc18606a6_39f4_4b8a_8403_28e5e5f033b4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Identity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
    pub FromElements: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        m11: f64,
        m12: f64,
        m21: f64,
        m22: f64,
        offsetx: f64,
        offsety: f64,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
    pub GetIsIdentity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Matrix,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Matrix,
        point: super::super::super::Foundation::Point,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixTransform {
    type Vtable = IMatrixTransform_Vtbl;
}
impl ::core::clone::Clone for IMatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMatrixTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xedfdd551_5fed_45fc_ae62_92a4b6cf9707,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Matrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Matrix,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixTransformStatics {
    type Vtable = IMatrixTransformStatics_Vtbl;
}
impl ::core::clone::Clone for IMatrixTransformStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMatrixTransformStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x43e02e47_15b8_4758_bb97_7d52420acc5b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MatrixProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface
for IMediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaTransportControlsThumbnailRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface
for IMediaTransportControlsThumbnailRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe4a8b21c_e3c2_485c_ae69_f1537b76755a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    SetThumbnailImage: usize,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPartialMediaFailureDetectedEventArgs {
    type Vtable = IPartialMediaFailureDetectedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPartialMediaFailureDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPartialMediaFailureDetectedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x02b65a91_e5a1_442b_88d3_2dc127bfc59b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    StreamKind: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPartialMediaFailureDetectedEventArgs2 {
    type Vtable = IPartialMediaFailureDetectedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IPartialMediaFailureDetectedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPartialMediaFailureDetectedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x73074875_890d_416b_b9ae_e84dfd9c4b1b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::HRESULT,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathFigure(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathFigure {
    type Vtable = IPathFigure_Vtbl;
}
impl ::core::clone::Clone for IPathFigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPathFigure {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5d955c8c_5fa9_4dda_a3cc_10fcdcaa20d7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigure_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Segments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsFilled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathFigureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathFigureStatics {
    type Vtable = IPathFigureStatics_Vtbl;
}
impl ::core::clone::Clone for IPathFigureStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPathFigureStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb60591d9_2395_4317_9552_3a58526f8c7b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigureStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SegmentsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StartPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsClosedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsFilledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathGeometry {
    type Vtable = IPathGeometry_Vtbl;
}
impl ::core::clone::Clone for IPathGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPathGeometry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x081b9df8_bae6_4bcb_813c_bde0e46dc8b7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FillRule,
    ) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FillRule,
    ) -> ::windows_core::HRESULT,
    pub Figures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFigures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathGeometryStatics {
    type Vtable = IPathGeometryStatics_Vtbl;
}
impl ::core::clone::Clone for IPathGeometryStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPathGeometryStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd9e58bba_2cba_4741_8f8d_3198cf5186b9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRuleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FiguresProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathSegment {
    type Vtable = IPathSegment_Vtbl;
}
impl ::core::clone::Clone for IPathSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPathSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfcfa71cf_9ce3_474f_8157_10b6435a616b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathSegmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathSegmentFactory {
    type Vtable = IPathSegmentFactory_Vtbl;
}
impl ::core::clone::Clone for IPathSegmentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPathSegmentFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2a1c0aae_eccd_4464_a148_6ffdb3aa281f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaneProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaneProjection {
    type Vtable = IPlaneProjection_Vtbl;
}
impl ::core::clone::Clone for IPlaneProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaneProjection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe6f82bfa_6726_469a_b259_a5188347ca8f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLocalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLocalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLocalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    ProjectionMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaneProjectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaneProjectionStatics {
    type Vtable = IPlaneProjectionStatics_Vtbl;
}
impl ::core::clone::Clone for IPlaneProjectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaneProjectionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xad919c67_3bdc_4855_8969_d1f9a3adc27d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalOffsetXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyBezierSegment {
    type Vtable = IPolyBezierSegment_Vtbl;
}
impl ::core::clone::Clone for IPolyBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPolyBezierSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x36805271_38c4_4bcf_96cd_028a6d38af25,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyBezierSegmentStatics {
    type Vtable = IPolyBezierSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for IPolyBezierSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPolyBezierSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1d91a6da_1492_4acc_bd66_a496f3d829d6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyLineSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyLineSegment {
    type Vtable = IPolyLineSegment_Vtbl;
}
impl ::core::clone::Clone for IPolyLineSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPolyLineSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4b397f87_a2e6_479d_bdc8_6f4464646887,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyLineSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyLineSegmentStatics {
    type Vtable = IPolyLineSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for IPolyLineSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPolyLineSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd64a2c87_33f1_4e70_a47f_b4981ef648a2,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_Vtbl;
}
impl ::core::clone::Clone for IPolyQuadraticBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPolyQuadraticBezierSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xdd5ced7d_e6db_4c96_b6a1_3fce96e987a6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyQuadraticBezierSegmentStatics {
    type Vtable = IPolyQuadraticBezierSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for IPolyQuadraticBezierSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPolyQuadraticBezierSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xfdf5eb75_7ad5_4c89_8169_8c9786abd9eb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProjection {
    type Vtable = IProjection_Vtbl;
}
impl ::core::clone::Clone for IProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProjection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb3443557_7f39_4d04_a89c_844338cac897,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProjectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProjectionFactory {
    type Vtable = IProjectionFactory_Vtbl;
}
impl ::core::clone::Clone for IProjectionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProjectionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc4f29cab_60ad_4f24_bd27_9d69c3127c9a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuadraticBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_Vtbl;
}
impl ::core::clone::Clone for IQuadraticBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQuadraticBezierSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2c509a5b_bf18_455a_a078_914b5232d8af,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub Point2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    pub SetPoint2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Point,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuadraticBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticBezierSegmentStatics {
    type Vtable = IQuadraticBezierSegmentStatics_Vtbl;
}
impl ::core::clone::Clone for IQuadraticBezierSegmentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQuadraticBezierSegmentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x69c78278_3c0b_4b4f_b7a2_f003ded41bb0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRateChangedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRateChangedRoutedEventArgs {
    type Vtable = IRateChangedRoutedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRateChangedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRateChangedRoutedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9016aa6f_3ca8_4c80_8e2f_8851a68f131f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRateChangedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRectangleGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectangleGeometry {
    type Vtable = IRectangleGeometry_Vtbl;
}
impl ::core::clone::Clone for IRectangleGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRectangleGeometry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa25a1f58_c575_4196_91cf_9fdfb10445c3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Rect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    pub SetRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRectangleGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectangleGeometryStatics {
    type Vtable = IRectangleGeometryStatics_Vtbl;
}
impl ::core::clone::Clone for IRectangleGeometryStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRectangleGeometryStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x377f8dba_7902_48e3_83be_7c8002a6653c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RectProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRenderedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderedEventArgs {
    type Vtable = IRenderedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRenderedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe349817d_81c7_4938_828c_a7e2797b35a6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FrameDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRenderingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderingEventArgs {
    type Vtable = IRenderingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRenderingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRenderingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5bf7d30d_9748_4aed_8380_d7890eb776a0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RenderingTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBackgroundBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBackgroundBrush {
    type Vtable = IRevealBackgroundBrush_Vtbl;
}
impl ::core::clone::Clone for IRevealBackgroundBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBackgroundBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x261dcc0e_1991_4cdf_aee0_6350a3f90bb9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBackgroundBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBackgroundBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBackgroundBrushFactory {
    type Vtable = IRevealBackgroundBrushFactory_Vtbl;
}
impl ::core::clone::Clone for IRevealBackgroundBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBackgroundBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x8c56bcaa_02a5_4f45_8506_8d39228f5d3f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBackgroundBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBorderBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBorderBrush {
    type Vtable = IRevealBorderBrush_Vtbl;
}
impl ::core::clone::Clone for IRevealBorderBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBorderBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x060ba115_c542_483c_8202_5f03331866c9,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBorderBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBorderBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBorderBrushFactory {
    type Vtable = IRevealBorderBrushFactory_Vtbl;
}
impl ::core::clone::Clone for IRevealBorderBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBorderBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x94c25298_f5f8_4482_a25c_6758501a8626,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBorderBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBrush {
    type Vtable = IRevealBrush_Vtbl;
}
impl ::core::clone::Clone for IRevealBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2036a0ed_8271_4398_9019_25872093f13f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub TargetTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ApplicationTheme,
    ) -> ::windows_core::HRESULT,
    pub SetTargetTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ApplicationTheme,
    ) -> ::windows_core::HRESULT,
    pub AlwaysUseFallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBrushFactory {
    type Vtable = IRevealBrushFactory_Vtbl;
}
impl ::core::clone::Clone for IRevealBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9d9379ce_e3a0_4aaf_be37_ea9d9dd43105,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBrushStatics {
    type Vtable = IRevealBrushStatics_Vtbl;
}
impl ::core::clone::Clone for IRevealBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRevealBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x190f2625_7209_4d42_a847_1ac4bbbb3499,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TargetThemeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: RevealBrushState,
    ) -> ::windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut RevealBrushState,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRotateTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRotateTransform {
    type Vtable = IRotateTransform_Vtbl;
}
impl ::core::clone::Clone for IRotateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRotateTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x688ea9b9_1e4e_4596_86e3_428b27334faf,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Angle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRotateTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRotateTransformStatics {
    type Vtable = IRotateTransformStatics_Vtbl;
}
impl ::core::clone::Clone for IRotateTransformStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRotateTransformStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa131eb8a_51a3_41b6_b9d3_a10e429054ab,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AngleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScaleTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleTransform {
    type Vtable = IScaleTransform_Vtbl;
}
impl ::core::clone::Clone for IScaleTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IScaleTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xed67f18d_936e_43ab_929a_e9cd0a511e52,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScaleTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleTransformStatics {
    type Vtable = IScaleTransformStatics_Vtbl;
}
impl ::core::clone::Clone for IScaleTransformStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IScaleTransformStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9d9436f4_40a7_46dd_975a_07d337cd852e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShadow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShadow {
    type Vtable = IShadow_Vtbl;
}
impl ::core::clone::Clone for IShadow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IShadow {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x6813a583_f3b4_5fcf_8694_2cd0aefc2fad,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadow_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShadowFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShadowFactory {
    type Vtable = IShadowFactory_Vtbl;
}
impl ::core::clone::Clone for IShadowFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IShadowFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x19899f25_d28b_51e6_94b0_d7e709686305,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadowFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISkewTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISkewTransform {
    type Vtable = ISkewTransform_Vtbl;
}
impl ::core::clone::Clone for ISkewTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISkewTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4e8a3b15_7a0f_4617_9e98_1e65bdc92115,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub AngleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetAngleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub AngleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISkewTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISkewTransformStatics {
    type Vtable = ISkewTransformStatics_Vtbl;
}
impl ::core::clone::Clone for ISkewTransformStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISkewTransformStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xecd11d73_5614_4b31_b6af_beae10105624,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AngleXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AngleYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISolidColorBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrush {
    type Vtable = ISolidColorBrush_Vtbl;
}
impl ::core::clone::Clone for ISolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISolidColorBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x9d850850_66f3_48df_9a8f_824bd5e070af,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Color,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISolidColorBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrushFactory {
    type Vtable = ISolidColorBrushFactory_Vtbl;
}
impl ::core::clone::Clone for ISolidColorBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISolidColorBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd935ce0c_86f5_4da6_8a27_b1619ef7f92b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        color: super::super::Color,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISolidColorBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrushStatics {
    type Vtable = ISolidColorBrushStatics_Vtbl;
}
impl ::core::clone::Clone for ISolidColorBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISolidColorBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe1a65efa_2b23_41ba_b9ba_7094ec8e4e9f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThemeShadow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeShadow {
    type Vtable = IThemeShadow_Vtbl;
}
impl ::core::clone::Clone for IThemeShadow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IThemeShadow {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3eccad09_7985_5f39_8b62_6c10696dca6f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadow_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Receivers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThemeShadowFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeShadowFactory {
    type Vtable = IThemeShadowFactory_Vtbl;
}
impl ::core::clone::Clone for IThemeShadowFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IThemeShadowFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x2e71465d_0f67_590e_831b_7e5e2a32b778,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadowFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrush {
    type Vtable = ITileBrush_Vtbl;
}
impl ::core::clone::Clone for ITileBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITileBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc201cf06_cd84_48a5_9607_664d7361cd61,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlignmentX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AlignmentX,
    ) -> ::windows_core::HRESULT,
    pub SetAlignmentX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AlignmentX,
    ) -> ::windows_core::HRESULT,
    pub AlignmentY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AlignmentY,
    ) -> ::windows_core::HRESULT,
    pub SetAlignmentY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AlignmentY,
    ) -> ::windows_core::HRESULT,
    pub Stretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Stretch,
    ) -> ::windows_core::HRESULT,
    pub SetStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Stretch,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrushFactory {
    type Vtable = ITileBrushFactory_Vtbl;
}
impl ::core::clone::Clone for ITileBrushFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITileBrushFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xaa159f7c_ed6a_4fb3_b014_b5c7e379a4de,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrushStatics {
    type Vtable = ITileBrushStatics_Vtbl;
}
impl ::core::clone::Clone for ITileBrushStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITileBrushStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x3497c25b_b562_4e68_8435_2399f6eb94d5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlignmentXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AlignmentYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineMarker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineMarker {
    type Vtable = ITimelineMarker_Vtbl;
}
impl ::core::clone::Clone for ITimelineMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimelineMarker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xa68ef02d_45ba_4e50_8cad_aaea3a227af5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Time: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub SetTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineMarkerRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineMarkerRoutedEventArgs {
    type Vtable = ITimelineMarkerRoutedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITimelineMarkerRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimelineMarkerRoutedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7c3b3ef3_2c88_4d9c_99b6_46cdbd48d4c1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarkerRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Marker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetMarker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineMarkerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineMarkerStatics {
    type Vtable = ITimelineMarkerStatics_Vtbl;
}
impl ::core::clone::Clone for ITimelineMarkerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimelineMarkerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc4aef0c6_16a3_484b_87f5_6528b8f04a47,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarkerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TimeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransform {
    type Vtable = ITransform_Vtbl;
}
impl ::core::clone::Clone for ITransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4df74078_bfd6_4ed1_9682_d2fd8bf2fe6f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformFactory {
    type Vtable = ITransformFactory_Vtbl;
}
impl ::core::clone::Clone for ITransformFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITransformFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1a955a66_7cf4_4320_b416_6181192fcc6d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformGroup {
    type Vtable = ITransformGroup_Vtbl;
}
impl ::core::clone::Clone for ITransformGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITransformGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x63418ccc_8d2d_4737_b951_2afce1ddc4c4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Children: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformGroupStatics {
    type Vtable = ITransformGroupStatics_Vtbl;
}
impl ::core::clone::Clone for ITransformGroupStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITransformGroupStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x25312f2a_cfab_4b24_9713_5bdead1929c0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChildrenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITranslateTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITranslateTransform {
    type Vtable = ITranslateTransform_Vtbl;
}
impl ::core::clone::Clone for ITranslateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITranslateTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xc975905c_3c36_4229_817b_178f64c0e113,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub X: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Y: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITranslateTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITranslateTransformStatics {
    type Vtable = ITranslateTransformStatics_Vtbl;
}
impl ::core::clone::Clone for ITranslateTransformStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITranslateTransformStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xf419aa91_e042_4111_9c2f_d201304123dd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub XProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub YProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelper {
    type Vtable = IVisualTreeHelper_Vtbl;
}
impl ::core::clone::Clone for IVisualTreeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualTreeHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x24b935e3_52c7_4141_8bac_a73d06130569,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics {
    type Vtable = IVisualTreeHelperStatics_Vtbl;
}
impl ::core::clone::Clone for IVisualTreeHelperStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualTreeHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xe75758c4_d25d_4b1d_971f_596f17f12baa,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FindElementsInHostCoordinatesPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingpoint: super::super::super::Foundation::Point,
        subtree: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindElementsInHostCoordinatesRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingrect: super::super::super::Foundation::Rect,
        subtree: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindAllElementsInHostCoordinatesPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingpoint: super::super::super::Foundation::Point,
        subtree: *mut ::core::ffi::c_void,
        includeallelements: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindAllElementsInHostCoordinatesRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingrect: super::super::super::Foundation::Rect,
        subtree: *mut ::core::ffi::c_void,
        includeallelements: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: *mut ::core::ffi::c_void,
        childindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetChildrenCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisconnectChildrenRecursive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelperStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics2 {
    type Vtable = IVisualTreeHelperStatics2_Vtbl;
}
impl ::core::clone::Clone for IVisualTreeHelperStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualTreeHelperStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x07bcd176_869f_44a7_8797_2103a4c3e47a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetOpenPopups: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        window: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelperStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics3 {
    type Vtable = IVisualTreeHelperStatics3_Vtbl;
}
impl ::core::clone::Clone for IVisualTreeHelperStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualTreeHelperStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x40420d50_ca16_57da_8aac_944c8af577fd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetOpenPopupsForXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xamlroot: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_Vtbl;
}
impl ::core::clone::Clone for IXamlCompositionBrushBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x03e432d9_b35c_4a79_811c_c5652004da0e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Color,
    ) -> ::windows_core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Color,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseFactory {
    type Vtable = IXamlCompositionBrushBaseFactory_Vtbl;
}
impl ::core::clone::Clone for IXamlCompositionBrushBaseFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x394f0823_2451_4ed8_bd24_488149b3428d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseOverrides {
    type Vtable = IXamlCompositionBrushBaseOverrides_Vtbl;
}
impl ::core::clone::Clone for IXamlCompositionBrushBaseOverrides {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xd19127f1_38b4_4ea1_8f33_849629a4c9c1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseProtected {
    type Vtable = IXamlCompositionBrushBaseProtected_Vtbl;
}
impl ::core::clone::Clone for IXamlCompositionBrushBaseProtected {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseProtected {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x1513f3d8_0457_4e1c_ad77_11c1d9879743,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseProtected_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    CompositionBrush: usize,
    SetCompositionBrush: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseStatics {
    type Vtable = IXamlCompositionBrushBaseStatics_Vtbl;
}
impl ::core::clone::Clone for IXamlCompositionBrushBaseStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x4fd49b06_061a_441f_b97a_adfbd41ae681,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FallbackColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLight(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLight {
    type Vtable = IXamlLight_Vtbl;
}
impl ::core::clone::Clone for IXamlLight {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlLight {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x0cc3fc1f_b327_4a18_9648_7c84db26ce22,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLight_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightFactory {
    type Vtable = IXamlLightFactory_Vtbl;
}
impl ::core::clone::Clone for IXamlLightFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlLightFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x87ded768_3055_43b8_8ef6_798dc4c2329a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightOverrides {
    type Vtable = IXamlLightOverrides_Vtbl;
}
impl ::core::clone::Clone for IXamlLightOverrides {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlLightOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x7c6296c7_0173_48e1_b73d_7fa216a9ac28,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub OnConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newelement: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        oldelement: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightProtected {
    type Vtable = IXamlLightProtected_Vtbl;
}
impl ::core::clone::Clone for IXamlLightProtected {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlLightProtected {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x5ecf220b_1252_43d0_9729_6ea692046838,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightProtected_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    CompositionLight: usize,
    SetCompositionLight: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightStatics {
    type Vtable = IXamlLightStatics_Vtbl;
}
impl ::core::clone::Clone for IXamlLightStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IXamlLightStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0xb5ea9d69_b508_4e9c_bd27_6b044b5f78a0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddTargetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveTargetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddTargetBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveTargetBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct AcrylicBrush(::windows_core::IUnknown);
impl AcrylicBrush {}
impl ::core::cmp::PartialEq for AcrylicBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AcrylicBrush {}
impl ::core::fmt::Debug for AcrylicBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AcrylicBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AcrylicBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.AcrylicBrush;{79bbcf4e-cd66-4f1b-a8b6-cd6d2977c18d})",
    );
}
impl ::core::clone::Clone for AcrylicBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AcrylicBrush {
    type Vtable = IAcrylicBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AcrylicBrush {
    const IID: ::windows_core::GUID = <IAcrylicBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AcrylicBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.AcrylicBrush";
}
::windows_core::imp::interface_hierarchy!(
    AcrylicBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<XamlCompositionBrushBase> for AcrylicBrush {}
impl ::windows_core::CanTryInto<Brush> for AcrylicBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for AcrylicBrush {}
unsafe impl ::core::marker::Send for AcrylicBrush {}
unsafe impl ::core::marker::Sync for AcrylicBrush {}
#[repr(transparent)]
pub struct ArcSegment(::windows_core::IUnknown);
impl ArcSegment {}
impl ::core::cmp::PartialEq for ArcSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ArcSegment {}
impl ::core::fmt::Debug for ArcSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ArcSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ArcSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.ArcSegment;{07940c5f-63fb-4469-91be-f1097c168052})",
    );
}
impl ::core::clone::Clone for ArcSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ArcSegment {
    type Vtable = IArcSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ArcSegment {
    const IID: ::windows_core::GUID = <IArcSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ArcSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ArcSegment";
}
::windows_core::imp::interface_hierarchy!(
    ArcSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for ArcSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ArcSegment {}
unsafe impl ::core::marker::Send for ArcSegment {}
unsafe impl ::core::marker::Sync for ArcSegment {}
#[repr(transparent)]
pub struct BezierSegment(::windows_core::IUnknown);
impl BezierSegment {}
impl ::core::cmp::PartialEq for BezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BezierSegment {}
impl ::core::fmt::Debug for BezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BezierSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.BezierSegment;{af4bb9ee-8984-49b7-81df-3f35994b95eb})",
    );
}
impl ::core::clone::Clone for BezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BezierSegment {
    type Vtable = IBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BezierSegment {
    const IID: ::windows_core::GUID = <IBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    BezierSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for BezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for BezierSegment {}
unsafe impl ::core::marker::Send for BezierSegment {}
unsafe impl ::core::marker::Sync for BezierSegment {}
#[repr(transparent)]
pub struct BitmapCache(::windows_core::IUnknown);
impl BitmapCache {}
impl ::core::cmp::PartialEq for BitmapCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapCache {}
impl ::core::fmt::Debug for BitmapCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapCache").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapCache {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.BitmapCache;{79c2219e-44d2-4610-9735-9bec83809ecf})",
    );
}
impl ::core::clone::Clone for BitmapCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapCache {
    type Vtable = IBitmapCache_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapCache {
    const IID: ::windows_core::GUID = <IBitmapCache as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapCache {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BitmapCache";
}
::windows_core::imp::interface_hierarchy!(
    BitmapCache, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<CacheMode> for BitmapCache {}
impl ::windows_core::CanTryInto<super::DependencyObject> for BitmapCache {}
unsafe impl ::core::marker::Send for BitmapCache {}
unsafe impl ::core::marker::Sync for BitmapCache {}
#[repr(transparent)]
pub struct Brush(::windows_core::IUnknown);
impl Brush {}
impl ::core::cmp::PartialEq for Brush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Brush {}
impl ::core::fmt::Debug for Brush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Brush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Brush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.Brush;{8806a321-1e06-422c-a1cc-01696559e021})",
    );
}
impl ::core::clone::Clone for Brush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Brush {
    type Vtable = IBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Brush {
    const IID: ::windows_core::GUID = <IBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Brush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Brush";
}
::windows_core::imp::interface_hierarchy!(
    Brush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Brush {}
unsafe impl ::core::marker::Send for Brush {}
unsafe impl ::core::marker::Sync for Brush {}
#[repr(transparent)]
pub struct BrushCollection(::windows_core::IUnknown);
impl BrushCollection {}
impl ::core::cmp::PartialEq for BrushCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BrushCollection {}
impl ::core::fmt::Debug for BrushCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrushCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BrushCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.BrushCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Brush;{8806a321-1e06-422c-a1cc-01696559e021})))",
    );
}
impl ::core::clone::Clone for BrushCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BrushCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<Brush>;
}
unsafe impl ::windows_core::ComInterface for BrushCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        Brush,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BrushCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BrushCollection";
}
impl ::core::iter::IntoIterator for BrushCollection {
    type Item = Brush;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &BrushCollection {
    type Item = Brush;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    BrushCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<Brush>,
> for BrushCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<Brush>,
> for BrushCollection {}
unsafe impl ::core::marker::Send for BrushCollection {}
unsafe impl ::core::marker::Sync for BrushCollection {}
#[repr(transparent)]
pub struct CacheMode(::windows_core::IUnknown);
impl CacheMode {}
impl ::core::cmp::PartialEq for CacheMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CacheMode {}
impl ::core::fmt::Debug for CacheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CacheMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CacheMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.CacheMode;{98dc8b11-c6f9-4dab-b838-5fd5ec8c7350})",
    );
}
impl ::core::clone::Clone for CacheMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CacheMode {
    type Vtable = ICacheMode_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CacheMode {
    const IID: ::windows_core::GUID = <ICacheMode as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CacheMode {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CacheMode";
}
::windows_core::imp::interface_hierarchy!(
    CacheMode, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for CacheMode {}
unsafe impl ::core::marker::Send for CacheMode {}
unsafe impl ::core::marker::Sync for CacheMode {}
#[repr(transparent)]
pub struct CompositeTransform(::windows_core::IUnknown);
impl CompositeTransform {}
impl ::core::cmp::PartialEq for CompositeTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositeTransform {}
impl ::core::fmt::Debug for CompositeTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositeTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CompositeTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.CompositeTransform;{c8a4385b-f24a-4701-a265-a78846f142b9})",
    );
}
impl ::core::clone::Clone for CompositeTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CompositeTransform {
    type Vtable = ICompositeTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompositeTransform {
    const IID: ::windows_core::GUID = <ICompositeTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompositeTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositeTransform";
}
::windows_core::imp::interface_hierarchy!(
    CompositeTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for CompositeTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for CompositeTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for CompositeTransform {}
unsafe impl ::core::marker::Send for CompositeTransform {}
unsafe impl ::core::marker::Sync for CompositeTransform {}
#[repr(transparent)]
pub struct CompositionTarget(::windows_core::IUnknown);
impl CompositionTarget {}
impl ::core::cmp::PartialEq for CompositionTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionTarget {}
impl ::core::fmt::Debug for CompositionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionTarget").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.CompositionTarget;{26cfbff0-713c-4bec-8803-e101f7b14ed3})",
    );
}
impl ::core::clone::Clone for CompositionTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CompositionTarget {
    type Vtable = ICompositionTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompositionTarget {
    const IID: ::windows_core::GUID = <ICompositionTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositionTarget";
}
::windows_core::imp::interface_hierarchy!(
    CompositionTarget, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CompositionTarget {}
unsafe impl ::core::marker::Sync for CompositionTarget {}
#[repr(transparent)]
pub struct DoubleCollection(::windows_core::IUnknown);
impl DoubleCollection {}
impl ::core::cmp::PartialEq for DoubleCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleCollection {}
impl ::core::fmt::Debug for DoubleCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DoubleCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.DoubleCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};f8))",
    );
}
impl ::core::clone::Clone for DoubleCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DoubleCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<f64>;
}
unsafe impl ::windows_core::ComInterface for DoubleCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        f64,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DoubleCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.DoubleCollection";
}
impl ::core::iter::IntoIterator for DoubleCollection {
    type Item = f64;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &DoubleCollection {
    type Item = f64;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    DoubleCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<f64>,
> for DoubleCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<f64>,
> for DoubleCollection {}
unsafe impl ::core::marker::Send for DoubleCollection {}
unsafe impl ::core::marker::Sync for DoubleCollection {}
#[repr(transparent)]
pub struct EllipseGeometry(::windows_core::IUnknown);
impl EllipseGeometry {}
impl ::core::cmp::PartialEq for EllipseGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EllipseGeometry {}
impl ::core::fmt::Debug for EllipseGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EllipseGeometry").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EllipseGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.EllipseGeometry;{d4f61bba-4ea2-40d6-aa6c-8d38aa87651f})",
    );
}
impl ::core::clone::Clone for EllipseGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EllipseGeometry {
    type Vtable = IEllipseGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EllipseGeometry {
    const IID: ::windows_core::GUID = <IEllipseGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EllipseGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.EllipseGeometry";
}
::windows_core::imp::interface_hierarchy!(
    EllipseGeometry, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for EllipseGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for EllipseGeometry {}
unsafe impl ::core::marker::Send for EllipseGeometry {}
unsafe impl ::core::marker::Sync for EllipseGeometry {}
#[repr(transparent)]
pub struct FontFamily(::windows_core::IUnknown);
impl FontFamily {}
impl ::core::cmp::PartialEq for FontFamily {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FontFamily {}
impl ::core::fmt::Debug for FontFamily {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontFamily").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FontFamily {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.FontFamily;{92467e64-d66a-4cf4-9322-3d23b3c0c361})",
    );
}
impl ::core::clone::Clone for FontFamily {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FontFamily {
    type Vtable = IFontFamily_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FontFamily {
    const IID: ::windows_core::GUID = <IFontFamily as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FontFamily {
    const NAME: &'static str = "Windows.UI.Xaml.Media.FontFamily";
}
::windows_core::imp::interface_hierarchy!(
    FontFamily, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FontFamily {}
unsafe impl ::core::marker::Sync for FontFamily {}
#[repr(transparent)]
pub struct GeneralTransform(::windows_core::IUnknown);
impl GeneralTransform {}
impl ::core::cmp::PartialEq for GeneralTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeneralTransform {}
impl ::core::fmt::Debug for GeneralTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneralTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeneralTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.GeneralTransform;{a06798b7-a2ec-415f-ade2-eade9333f2c7})",
    );
}
impl ::core::clone::Clone for GeneralTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GeneralTransform {
    type Vtable = IGeneralTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeneralTransform {
    const IID: ::windows_core::GUID = <IGeneralTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeneralTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeneralTransform";
}
::windows_core::imp::interface_hierarchy!(
    GeneralTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for GeneralTransform {}
unsafe impl ::core::marker::Send for GeneralTransform {}
unsafe impl ::core::marker::Sync for GeneralTransform {}
#[repr(transparent)]
pub struct Geometry(::windows_core::IUnknown);
impl Geometry {}
impl ::core::cmp::PartialEq for Geometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geometry {}
impl ::core::fmt::Debug for Geometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geometry").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Geometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.Geometry;{fa123889-0acd-417b-b62d-5ca1bf4dfc0e})",
    );
}
impl ::core::clone::Clone for Geometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Geometry {
    type Vtable = IGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geometry {
    const IID: ::windows_core::GUID = <IGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Geometry";
}
::windows_core::imp::interface_hierarchy!(
    Geometry, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Geometry {}
unsafe impl ::core::marker::Send for Geometry {}
unsafe impl ::core::marker::Sync for Geometry {}
#[repr(transparent)]
pub struct GeometryCollection(::windows_core::IUnknown);
impl GeometryCollection {}
impl ::core::cmp::PartialEq for GeometryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeometryCollection {}
impl ::core::fmt::Debug for GeometryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeometryCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeometryCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.GeometryCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Geometry;{fa123889-0acd-417b-b62d-5ca1bf4dfc0e})))",
    );
}
impl ::core::clone::Clone for GeometryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GeometryCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<Geometry>;
}
unsafe impl ::windows_core::ComInterface for GeometryCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        Geometry,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeometryCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeometryCollection";
}
impl ::core::iter::IntoIterator for GeometryCollection {
    type Item = Geometry;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &GeometryCollection {
    type Item = Geometry;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    GeometryCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<Geometry>,
> for GeometryCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<Geometry>,
> for GeometryCollection {}
unsafe impl ::core::marker::Send for GeometryCollection {}
unsafe impl ::core::marker::Sync for GeometryCollection {}
#[repr(transparent)]
pub struct GeometryGroup(::windows_core::IUnknown);
impl GeometryGroup {}
impl ::core::cmp::PartialEq for GeometryGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeometryGroup {}
impl ::core::fmt::Debug for GeometryGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeometryGroup").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeometryGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.GeometryGroup;{55225a61-8677-4c8c-8e46-ee3dc355114b})",
    );
}
impl ::core::clone::Clone for GeometryGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GeometryGroup {
    type Vtable = IGeometryGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeometryGroup {
    const IID: ::windows_core::GUID = <IGeometryGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeometryGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeometryGroup";
}
::windows_core::imp::interface_hierarchy!(
    GeometryGroup, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for GeometryGroup {}
impl ::windows_core::CanTryInto<super::DependencyObject> for GeometryGroup {}
unsafe impl ::core::marker::Send for GeometryGroup {}
unsafe impl ::core::marker::Sync for GeometryGroup {}
#[repr(transparent)]
pub struct GradientBrush(::windows_core::IUnknown);
impl GradientBrush {}
impl ::core::cmp::PartialEq for GradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GradientBrush {}
impl ::core::fmt::Debug for GradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GradientBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.GradientBrush;{2166e69f-935a-4191-8e3c-1c8dfdfcdc78})",
    );
}
impl ::core::clone::Clone for GradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GradientBrush {
    type Vtable = IGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GradientBrush {
    const IID: ::windows_core::GUID = <IGradientBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientBrush";
}
::windows_core::imp::interface_hierarchy!(
    GradientBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Brush> for GradientBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for GradientBrush {}
unsafe impl ::core::marker::Send for GradientBrush {}
unsafe impl ::core::marker::Sync for GradientBrush {}
#[repr(transparent)]
pub struct GradientStop(::windows_core::IUnknown);
impl GradientStop {}
impl ::core::cmp::PartialEq for GradientStop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GradientStop {}
impl ::core::fmt::Debug for GradientStop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientStop").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GradientStop {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.GradientStop;{665f44fe-2e59-4c4a-ab53-076a100ccd81})",
    );
}
impl ::core::clone::Clone for GradientStop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GradientStop {
    type Vtable = IGradientStop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GradientStop {
    const IID: ::windows_core::GUID = <IGradientStop as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GradientStop {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientStop";
}
::windows_core::imp::interface_hierarchy!(
    GradientStop, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for GradientStop {}
unsafe impl ::core::marker::Send for GradientStop {}
unsafe impl ::core::marker::Sync for GradientStop {}
#[repr(transparent)]
pub struct GradientStopCollection(::windows_core::IUnknown);
impl GradientStopCollection {}
impl ::core::cmp::PartialEq for GradientStopCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GradientStopCollection {}
impl ::core::fmt::Debug for GradientStopCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientStopCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GradientStopCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.GradientStopCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.GradientStop;{665f44fe-2e59-4c4a-ab53-076a100ccd81})))",
    );
}
impl ::core::clone::Clone for GradientStopCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GradientStopCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<
        GradientStop,
    >;
}
unsafe impl ::windows_core::ComInterface for GradientStopCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        GradientStop,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GradientStopCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientStopCollection";
}
impl ::core::iter::IntoIterator for GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    GradientStopCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<GradientStop>,
> for GradientStopCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<GradientStop>,
> for GradientStopCollection {}
unsafe impl ::core::marker::Send for GradientStopCollection {}
unsafe impl ::core::marker::Sync for GradientStopCollection {}
#[repr(transparent)]
pub struct ImageBrush(::windows_core::IUnknown);
impl ImageBrush {}
impl ::core::cmp::PartialEq for ImageBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageBrush {}
impl ::core::fmt::Debug for ImageBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.ImageBrush;{9fd11377-c12a-4493-bf7d-f3a8ad74b554})",
    );
}
impl ::core::clone::Clone for ImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageBrush {
    type Vtable = IImageBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageBrush {
    const IID: ::windows_core::GUID = <IImageBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageBrush";
}
::windows_core::imp::interface_hierarchy!(
    ImageBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<TileBrush> for ImageBrush {}
impl ::windows_core::CanTryInto<Brush> for ImageBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ImageBrush {}
unsafe impl ::core::marker::Send for ImageBrush {}
unsafe impl ::core::marker::Sync for ImageBrush {}
#[repr(transparent)]
pub struct ImageSource(::windows_core::IUnknown);
impl ImageSource {}
impl ::core::cmp::PartialEq for ImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageSource {}
impl ::core::fmt::Debug for ImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.ImageSource;{737ef309-ea41-4d96-a71c-98e98efcab07})",
    );
}
impl ::core::clone::Clone for ImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageSource {
    type Vtable = IImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageSource {
    const IID: ::windows_core::GUID = <IImageSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageSource";
}
::windows_core::imp::interface_hierarchy!(
    ImageSource, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for ImageSource {}
unsafe impl ::core::marker::Send for ImageSource {}
unsafe impl ::core::marker::Sync for ImageSource {}
#[repr(transparent)]
pub struct LineGeometry(::windows_core::IUnknown);
impl LineGeometry {}
impl ::core::cmp::PartialEq for LineGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineGeometry {}
impl ::core::fmt::Debug for LineGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineGeometry").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LineGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.LineGeometry;{30edd4a2-8fc5-40af-a7a2-c27fe7aa1363})",
    );
}
impl ::core::clone::Clone for LineGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LineGeometry {
    type Vtable = ILineGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LineGeometry {
    const IID: ::windows_core::GUID = <ILineGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LineGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LineGeometry";
}
::windows_core::imp::interface_hierarchy!(
    LineGeometry, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for LineGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LineGeometry {}
unsafe impl ::core::marker::Send for LineGeometry {}
unsafe impl ::core::marker::Sync for LineGeometry {}
#[repr(transparent)]
pub struct LineSegment(::windows_core::IUnknown);
impl LineSegment {}
impl ::core::cmp::PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineSegment {}
impl ::core::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LineSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.LineSegment;{ef6a2e25-3ff0-4420-a411-7182a4cecb15})",
    );
}
impl ::core::clone::Clone for LineSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LineSegment {
    type Vtable = ILineSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LineSegment {
    const IID: ::windows_core::GUID = <ILineSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LineSegment";
}
::windows_core::imp::interface_hierarchy!(
    LineSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for LineSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LineSegment {}
unsafe impl ::core::marker::Send for LineSegment {}
unsafe impl ::core::marker::Sync for LineSegment {}
#[repr(transparent)]
pub struct LinearGradientBrush(::windows_core::IUnknown);
impl LinearGradientBrush {}
impl ::core::cmp::PartialEq for LinearGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearGradientBrush {}
impl ::core::fmt::Debug for LinearGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearGradientBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LinearGradientBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.LinearGradientBrush;{8e96d16b-bb84-4c6f-9dbf-9d6c5c6d9c39})",
    );
}
impl ::core::clone::Clone for LinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LinearGradientBrush {
    type Vtable = ILinearGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LinearGradientBrush {
    const IID: ::windows_core::GUID = <ILinearGradientBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LinearGradientBrush";
}
::windows_core::imp::interface_hierarchy!(
    LinearGradientBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<GradientBrush> for LinearGradientBrush {}
impl ::windows_core::CanTryInto<Brush> for LinearGradientBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LinearGradientBrush {}
unsafe impl ::core::marker::Send for LinearGradientBrush {}
unsafe impl ::core::marker::Sync for LinearGradientBrush {}
#[repr(transparent)]
pub struct LoadedImageSourceLoadCompletedEventArgs(::windows_core::IUnknown);
impl LoadedImageSourceLoadCompletedEventArgs {}
impl ::core::cmp::PartialEq for LoadedImageSourceLoadCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoadedImageSourceLoadCompletedEventArgs {}
impl ::core::fmt::Debug for LoadedImageSourceLoadCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSourceLoadCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoadedImageSourceLoadCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs;{1ac60b1e-7837-4489-b3e5-d0d5ad0a56c4})",
    );
}
impl ::core::clone::Clone for LoadedImageSourceLoadCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoadedImageSourceLoadCompletedEventArgs {
    const IID: ::windows_core::GUID = <ILoadedImageSourceLoadCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoadedImageSourceLoadCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    LoadedImageSourceLoadCompletedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for LoadedImageSourceLoadCompletedEventArgs {}
unsafe impl ::core::marker::Sync for LoadedImageSourceLoadCompletedEventArgs {}
#[repr(transparent)]
pub struct LoadedImageSurface(::windows_core::IUnknown);
impl LoadedImageSurface {}
impl ::core::cmp::PartialEq for LoadedImageSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoadedImageSurface {}
impl ::core::fmt::Debug for LoadedImageSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSurface").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoadedImageSurface {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.LoadedImageSurface;{050c8313-6737-45ba-8531-33094febef55})",
    );
}
impl ::core::clone::Clone for LoadedImageSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoadedImageSurface {
    type Vtable = ILoadedImageSurface_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoadedImageSurface {
    const IID: ::windows_core::GUID = <ILoadedImageSurface as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoadedImageSurface {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LoadedImageSurface";
}
::windows_core::imp::interface_hierarchy!(
    LoadedImageSurface, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable>
for LoadedImageSurface {}
unsafe impl ::core::marker::Send for LoadedImageSurface {}
unsafe impl ::core::marker::Sync for LoadedImageSurface {}
#[repr(transparent)]
pub struct Matrix3DProjection(::windows_core::IUnknown);
impl Matrix3DProjection {}
impl ::core::cmp::PartialEq for Matrix3DProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Matrix3DProjection {}
impl ::core::fmt::Debug for Matrix3DProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Matrix3DProjection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Matrix3DProjection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.Matrix3DProjection;{6f03e149-bfc9-4c01-b578-50338cec97fc})",
    );
}
impl ::core::clone::Clone for Matrix3DProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Matrix3DProjection {
    type Vtable = IMatrix3DProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Matrix3DProjection {
    const IID: ::windows_core::GUID = <IMatrix3DProjection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Matrix3DProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Matrix3DProjection";
}
::windows_core::imp::interface_hierarchy!(
    Matrix3DProjection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Projection> for Matrix3DProjection {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Matrix3DProjection {}
unsafe impl ::core::marker::Send for Matrix3DProjection {}
unsafe impl ::core::marker::Sync for Matrix3DProjection {}
#[repr(transparent)]
pub struct MatrixHelper(::windows_core::IUnknown);
impl MatrixHelper {}
impl ::core::cmp::PartialEq for MatrixHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MatrixHelper {}
impl ::core::fmt::Debug for MatrixHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MatrixHelper").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MatrixHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.MatrixHelper;{f3cf4882-06b5-48c8-9eb2-1763e9364038})",
    );
}
impl ::core::clone::Clone for MatrixHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MatrixHelper {
    type Vtable = IMatrixHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MatrixHelper {
    const IID: ::windows_core::GUID = <IMatrixHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MatrixHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MatrixHelper";
}
::windows_core::imp::interface_hierarchy!(
    MatrixHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MatrixHelper {}
unsafe impl ::core::marker::Sync for MatrixHelper {}
#[repr(transparent)]
pub struct MatrixTransform(::windows_core::IUnknown);
impl MatrixTransform {}
impl ::core::cmp::PartialEq for MatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MatrixTransform {}
impl ::core::fmt::Debug for MatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MatrixTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MatrixTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.MatrixTransform;{edfdd551-5fed-45fc-ae62-92a4b6cf9707})",
    );
}
impl ::core::clone::Clone for MatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MatrixTransform {
    type Vtable = IMatrixTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MatrixTransform {
    const IID: ::windows_core::GUID = <IMatrixTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MatrixTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MatrixTransform";
}
::windows_core::imp::interface_hierarchy!(
    MatrixTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for MatrixTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for MatrixTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for MatrixTransform {}
unsafe impl ::core::marker::Send for MatrixTransform {}
unsafe impl ::core::marker::Sync for MatrixTransform {}
#[repr(transparent)]
pub struct MediaTransportControlsThumbnailRequestedEventArgs(::windows_core::IUnknown);
impl MediaTransportControlsThumbnailRequestedEventArgs {}
impl ::core::cmp::PartialEq for MediaTransportControlsThumbnailRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTransportControlsThumbnailRequestedEventArgs {}
impl ::core::fmt::Debug for MediaTransportControlsThumbnailRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTransportControlsThumbnailRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
impl ::windows_core::RuntimeType for MediaTransportControlsThumbnailRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs;{e4a8b21c-e3c2-485c-ae69-f1537b76755a})",
    );
}
impl ::core::clone::Clone for MediaTransportControlsThumbnailRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface
for MediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface
for MediaTransportControlsThumbnailRequestedEventArgs {
    const IID: ::windows_core::GUID = <IMediaTransportControlsThumbnailRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaTransportControlsThumbnailRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    MediaTransportControlsThumbnailRequestedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MediaTransportControlsThumbnailRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTransportControlsThumbnailRequestedEventArgs {}
#[repr(transparent)]
pub struct PartialMediaFailureDetectedEventArgs(::windows_core::IUnknown);
impl PartialMediaFailureDetectedEventArgs {}
impl ::core::cmp::PartialEq for PartialMediaFailureDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PartialMediaFailureDetectedEventArgs {}
impl ::core::fmt::Debug for PartialMediaFailureDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PartialMediaFailureDetectedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PartialMediaFailureDetectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PartialMediaFailureDetectedEventArgs;{02b65a91-e5a1-442b-88d3-2dc127bfc59b})",
    );
}
impl ::core::clone::Clone for PartialMediaFailureDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PartialMediaFailureDetectedEventArgs {
    type Vtable = IPartialMediaFailureDetectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PartialMediaFailureDetectedEventArgs {
    const IID: ::windows_core::GUID = <IPartialMediaFailureDetectedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PartialMediaFailureDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PartialMediaFailureDetectedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    PartialMediaFailureDetectedEventArgs, ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PartialMediaFailureDetectedEventArgs {}
unsafe impl ::core::marker::Sync for PartialMediaFailureDetectedEventArgs {}
#[repr(transparent)]
pub struct PathFigure(::windows_core::IUnknown);
impl PathFigure {}
impl ::core::cmp::PartialEq for PathFigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathFigure {}
impl ::core::fmt::Debug for PathFigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathFigure").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PathFigure {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PathFigure;{5d955c8c-5fa9-4dda-a3cc-10fcdcaa20d7})",
    );
}
impl ::core::clone::Clone for PathFigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PathFigure {
    type Vtable = IPathFigure_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PathFigure {
    const IID: ::windows_core::GUID = <IPathFigure as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathFigure {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathFigure";
}
::windows_core::imp::interface_hierarchy!(
    PathFigure, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for PathFigure {}
unsafe impl ::core::marker::Send for PathFigure {}
unsafe impl ::core::marker::Sync for PathFigure {}
#[repr(transparent)]
pub struct PathFigureCollection(::windows_core::IUnknown);
impl PathFigureCollection {}
impl ::core::cmp::PartialEq for PathFigureCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathFigureCollection {}
impl ::core::fmt::Debug for PathFigureCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathFigureCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PathFigureCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PathFigureCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.PathFigure;{5d955c8c-5fa9-4dda-a3cc-10fcdcaa20d7})))",
    );
}
impl ::core::clone::Clone for PathFigureCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PathFigureCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<PathFigure>;
}
unsafe impl ::windows_core::ComInterface for PathFigureCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        PathFigure,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathFigureCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathFigureCollection";
}
impl ::core::iter::IntoIterator for PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    PathFigureCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<PathFigure>,
> for PathFigureCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<PathFigure>,
> for PathFigureCollection {}
unsafe impl ::core::marker::Send for PathFigureCollection {}
unsafe impl ::core::marker::Sync for PathFigureCollection {}
#[repr(transparent)]
pub struct PathGeometry(::windows_core::IUnknown);
impl PathGeometry {}
impl ::core::cmp::PartialEq for PathGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathGeometry {}
impl ::core::fmt::Debug for PathGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathGeometry").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PathGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PathGeometry;{081b9df8-bae6-4bcb-813c-bde0e46dc8b7})",
    );
}
impl ::core::clone::Clone for PathGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PathGeometry {
    type Vtable = IPathGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PathGeometry {
    const IID: ::windows_core::GUID = <IPathGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathGeometry";
}
::windows_core::imp::interface_hierarchy!(
    PathGeometry, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for PathGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PathGeometry {}
unsafe impl ::core::marker::Send for PathGeometry {}
unsafe impl ::core::marker::Sync for PathGeometry {}
#[repr(transparent)]
pub struct PathSegment(::windows_core::IUnknown);
impl PathSegment {}
impl ::core::cmp::PartialEq for PathSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathSegment {}
impl ::core::fmt::Debug for PathSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PathSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PathSegment;{fcfa71cf-9ce3-474f-8157-10b6435a616b})",
    );
}
impl ::core::clone::Clone for PathSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PathSegment {
    type Vtable = IPathSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PathSegment {
    const IID: ::windows_core::GUID = <IPathSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathSegment";
}
::windows_core::imp::interface_hierarchy!(
    PathSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for PathSegment {}
unsafe impl ::core::marker::Send for PathSegment {}
unsafe impl ::core::marker::Sync for PathSegment {}
#[repr(transparent)]
pub struct PathSegmentCollection(::windows_core::IUnknown);
impl PathSegmentCollection {}
impl ::core::cmp::PartialEq for PathSegmentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathSegmentCollection {}
impl ::core::fmt::Debug for PathSegmentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathSegmentCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PathSegmentCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PathSegmentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.PathSegment;{fcfa71cf-9ce3-474f-8157-10b6435a616b})))",
    );
}
impl ::core::clone::Clone for PathSegmentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PathSegmentCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<
        PathSegment,
    >;
}
unsafe impl ::windows_core::ComInterface for PathSegmentCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        PathSegment,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathSegmentCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathSegmentCollection";
}
impl ::core::iter::IntoIterator for PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    PathSegmentCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<PathSegment>,
> for PathSegmentCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<PathSegment>,
> for PathSegmentCollection {}
unsafe impl ::core::marker::Send for PathSegmentCollection {}
unsafe impl ::core::marker::Sync for PathSegmentCollection {}
#[repr(transparent)]
pub struct PlaneProjection(::windows_core::IUnknown);
impl PlaneProjection {}
impl ::core::cmp::PartialEq for PlaneProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaneProjection {}
impl ::core::fmt::Debug for PlaneProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaneProjection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlaneProjection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PlaneProjection;{e6f82bfa-6726-469a-b259-a5188347ca8f})",
    );
}
impl ::core::clone::Clone for PlaneProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PlaneProjection {
    type Vtable = IPlaneProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaneProjection {
    const IID: ::windows_core::GUID = <IPlaneProjection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaneProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PlaneProjection";
}
::windows_core::imp::interface_hierarchy!(
    PlaneProjection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Projection> for PlaneProjection {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PlaneProjection {}
unsafe impl ::core::marker::Send for PlaneProjection {}
unsafe impl ::core::marker::Sync for PlaneProjection {}
#[repr(transparent)]
pub struct PointCollection(::windows_core::IUnknown);
impl PointCollection {}
impl ::core::cmp::PartialEq for PointCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointCollection {}
impl ::core::fmt::Debug for PointCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PointCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PointCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};struct(Windows.Foundation.Point;f4;f4)))",
    );
}
impl ::core::clone::Clone for PointCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PointCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<
        super::super::super::Foundation::Point,
    >;
}
unsafe impl ::windows_core::ComInterface for PointCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        super::super::super::Foundation::Point,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PointCollection";
}
impl ::core::iter::IntoIterator for PointCollection {
    type Item = super::super::super::Foundation::Point;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &PointCollection {
    type Item = super::super::super::Foundation::Point;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    PointCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<
        super::super::super::Foundation::Point,
    >,
> for PointCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<
        super::super::super::Foundation::Point,
    >,
> for PointCollection {}
unsafe impl ::core::marker::Send for PointCollection {}
unsafe impl ::core::marker::Sync for PointCollection {}
#[repr(transparent)]
pub struct PolyBezierSegment(::windows_core::IUnknown);
impl PolyBezierSegment {}
impl ::core::cmp::PartialEq for PolyBezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PolyBezierSegment {}
impl ::core::fmt::Debug for PolyBezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolyBezierSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PolyBezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PolyBezierSegment;{36805271-38c4-4bcf-96cd-028a6d38af25})",
    );
}
impl ::core::clone::Clone for PolyBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PolyBezierSegment {
    type Vtable = IPolyBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PolyBezierSegment {
    const IID: ::windows_core::GUID = <IPolyBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PolyBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyBezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    PolyBezierSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for PolyBezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PolyBezierSegment {}
unsafe impl ::core::marker::Send for PolyBezierSegment {}
unsafe impl ::core::marker::Sync for PolyBezierSegment {}
#[repr(transparent)]
pub struct PolyLineSegment(::windows_core::IUnknown);
impl PolyLineSegment {}
impl ::core::cmp::PartialEq for PolyLineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PolyLineSegment {}
impl ::core::fmt::Debug for PolyLineSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolyLineSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PolyLineSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PolyLineSegment;{4b397f87-a2e6-479d-bdc8-6f4464646887})",
    );
}
impl ::core::clone::Clone for PolyLineSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PolyLineSegment {
    type Vtable = IPolyLineSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PolyLineSegment {
    const IID: ::windows_core::GUID = <IPolyLineSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PolyLineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyLineSegment";
}
::windows_core::imp::interface_hierarchy!(
    PolyLineSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for PolyLineSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PolyLineSegment {}
unsafe impl ::core::marker::Send for PolyLineSegment {}
unsafe impl ::core::marker::Sync for PolyLineSegment {}
#[repr(transparent)]
pub struct PolyQuadraticBezierSegment(::windows_core::IUnknown);
impl PolyQuadraticBezierSegment {}
impl ::core::cmp::PartialEq for PolyQuadraticBezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PolyQuadraticBezierSegment {}
impl ::core::fmt::Debug for PolyQuadraticBezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolyQuadraticBezierSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PolyQuadraticBezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.PolyQuadraticBezierSegment;{dd5ced7d-e6db-4c96-b6a1-3fce96e987a6})",
    );
}
impl ::core::clone::Clone for PolyQuadraticBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PolyQuadraticBezierSegment {
    const IID: ::windows_core::GUID = <IPolyQuadraticBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PolyQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyQuadraticBezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    PolyQuadraticBezierSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for PolyQuadraticBezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PolyQuadraticBezierSegment {}
unsafe impl ::core::marker::Send for PolyQuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for PolyQuadraticBezierSegment {}
#[repr(transparent)]
pub struct Projection(::windows_core::IUnknown);
impl Projection {}
impl ::core::cmp::PartialEq for Projection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Projection {}
impl ::core::fmt::Debug for Projection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Projection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Projection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.Projection;{b3443557-7f39-4d04-a89c-844338cac897})",
    );
}
impl ::core::clone::Clone for Projection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Projection {
    type Vtable = IProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Projection {
    const IID: ::windows_core::GUID = <IProjection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Projection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Projection";
}
::windows_core::imp::interface_hierarchy!(
    Projection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Projection {}
unsafe impl ::core::marker::Send for Projection {}
unsafe impl ::core::marker::Sync for Projection {}
#[repr(transparent)]
pub struct QuadraticBezierSegment(::windows_core::IUnknown);
impl QuadraticBezierSegment {}
impl ::core::cmp::PartialEq for QuadraticBezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuadraticBezierSegment {}
impl ::core::fmt::Debug for QuadraticBezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuadraticBezierSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for QuadraticBezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.QuadraticBezierSegment;{2c509a5b-bf18-455a-a078-914b5232d8af})",
    );
}
impl ::core::clone::Clone for QuadraticBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for QuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for QuadraticBezierSegment {
    const IID: ::windows_core::GUID = <IQuadraticBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for QuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.QuadraticBezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    QuadraticBezierSegment, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for QuadraticBezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for QuadraticBezierSegment {}
unsafe impl ::core::marker::Send for QuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for QuadraticBezierSegment {}
#[repr(transparent)]
pub struct RateChangedRoutedEventArgs(::windows_core::IUnknown);
impl RateChangedRoutedEventArgs {}
impl ::core::cmp::PartialEq for RateChangedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RateChangedRoutedEventArgs {}
impl ::core::fmt::Debug for RateChangedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RateChangedRoutedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RateChangedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RateChangedRoutedEventArgs;{9016aa6f-3ca8-4c80-8e2f-8851a68f131f})",
    );
}
impl ::core::clone::Clone for RateChangedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RateChangedRoutedEventArgs {
    type Vtable = IRateChangedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RateChangedRoutedEventArgs {
    const IID: ::windows_core::GUID = <IRateChangedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RateChangedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RateChangedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RateChangedRoutedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for RateChangedRoutedEventArgs {}
unsafe impl ::core::marker::Send for RateChangedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RateChangedRoutedEventArgs {}
#[repr(transparent)]
pub struct RectangleGeometry(::windows_core::IUnknown);
impl RectangleGeometry {}
impl ::core::cmp::PartialEq for RectangleGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RectangleGeometry {}
impl ::core::fmt::Debug for RectangleGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RectangleGeometry").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RectangleGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RectangleGeometry;{a25a1f58-c575-4196-91cf-9fdfb10445c3})",
    );
}
impl ::core::clone::Clone for RectangleGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RectangleGeometry {
    type Vtable = IRectangleGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RectangleGeometry {
    const IID: ::windows_core::GUID = <IRectangleGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RectangleGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RectangleGeometry";
}
::windows_core::imp::interface_hierarchy!(
    RectangleGeometry, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for RectangleGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RectangleGeometry {}
unsafe impl ::core::marker::Send for RectangleGeometry {}
unsafe impl ::core::marker::Sync for RectangleGeometry {}
#[repr(transparent)]
pub struct RenderedEventArgs(::windows_core::IUnknown);
impl RenderedEventArgs {}
impl ::core::cmp::PartialEq for RenderedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RenderedEventArgs {}
impl ::core::fmt::Debug for RenderedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenderedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RenderedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RenderedEventArgs;{e349817d-81c7-4938-828c-a7e2797b35a6})",
    );
}
impl ::core::clone::Clone for RenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RenderedEventArgs {
    type Vtable = IRenderedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RenderedEventArgs {
    const IID: ::windows_core::GUID = <IRenderedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RenderedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RenderedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RenderedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RenderedEventArgs {}
unsafe impl ::core::marker::Sync for RenderedEventArgs {}
#[repr(transparent)]
pub struct RenderingEventArgs(::windows_core::IUnknown);
impl RenderingEventArgs {}
impl ::core::cmp::PartialEq for RenderingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RenderingEventArgs {}
impl ::core::fmt::Debug for RenderingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenderingEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RenderingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RenderingEventArgs;{5bf7d30d-9748-4aed-8380-d7890eb776a0})",
    );
}
impl ::core::clone::Clone for RenderingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RenderingEventArgs {
    type Vtable = IRenderingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RenderingEventArgs {
    const IID: ::windows_core::GUID = <IRenderingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RenderingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RenderingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RenderingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RenderingEventArgs {}
unsafe impl ::core::marker::Sync for RenderingEventArgs {}
#[repr(transparent)]
pub struct RevealBackgroundBrush(::windows_core::IUnknown);
impl RevealBackgroundBrush {}
impl ::core::cmp::PartialEq for RevealBackgroundBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevealBackgroundBrush {}
impl ::core::fmt::Debug for RevealBackgroundBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBackgroundBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RevealBackgroundBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RevealBackgroundBrush;{261dcc0e-1991-4cdf-aee0-6350a3f90bb9})",
    );
}
impl ::core::clone::Clone for RevealBackgroundBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RevealBackgroundBrush {
    type Vtable = IRevealBackgroundBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RevealBackgroundBrush {
    const IID: ::windows_core::GUID = <IRevealBackgroundBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RevealBackgroundBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBackgroundBrush";
}
::windows_core::imp::interface_hierarchy!(
    RevealBackgroundBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RevealBrush> for RevealBackgroundBrush {}
impl ::windows_core::CanTryInto<XamlCompositionBrushBase> for RevealBackgroundBrush {}
impl ::windows_core::CanTryInto<Brush> for RevealBackgroundBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RevealBackgroundBrush {}
unsafe impl ::core::marker::Send for RevealBackgroundBrush {}
unsafe impl ::core::marker::Sync for RevealBackgroundBrush {}
#[repr(transparent)]
pub struct RevealBorderBrush(::windows_core::IUnknown);
impl RevealBorderBrush {}
impl ::core::cmp::PartialEq for RevealBorderBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevealBorderBrush {}
impl ::core::fmt::Debug for RevealBorderBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBorderBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RevealBorderBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RevealBorderBrush;{060ba115-c542-483c-8202-5f03331866c9})",
    );
}
impl ::core::clone::Clone for RevealBorderBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RevealBorderBrush {
    type Vtable = IRevealBorderBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RevealBorderBrush {
    const IID: ::windows_core::GUID = <IRevealBorderBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RevealBorderBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBorderBrush";
}
::windows_core::imp::interface_hierarchy!(
    RevealBorderBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<RevealBrush> for RevealBorderBrush {}
impl ::windows_core::CanTryInto<XamlCompositionBrushBase> for RevealBorderBrush {}
impl ::windows_core::CanTryInto<Brush> for RevealBorderBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RevealBorderBrush {}
unsafe impl ::core::marker::Send for RevealBorderBrush {}
unsafe impl ::core::marker::Sync for RevealBorderBrush {}
#[repr(transparent)]
pub struct RevealBrush(::windows_core::IUnknown);
impl RevealBrush {}
impl ::core::cmp::PartialEq for RevealBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevealBrush {}
impl ::core::fmt::Debug for RevealBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RevealBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RevealBrush;{2036a0ed-8271-4398-9019-25872093f13f})",
    );
}
impl ::core::clone::Clone for RevealBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RevealBrush {
    type Vtable = IRevealBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RevealBrush {
    const IID: ::windows_core::GUID = <IRevealBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RevealBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBrush";
}
::windows_core::imp::interface_hierarchy!(
    RevealBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<XamlCompositionBrushBase> for RevealBrush {}
impl ::windows_core::CanTryInto<Brush> for RevealBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RevealBrush {}
unsafe impl ::core::marker::Send for RevealBrush {}
unsafe impl ::core::marker::Sync for RevealBrush {}
#[repr(transparent)]
pub struct RotateTransform(::windows_core::IUnknown);
impl RotateTransform {}
impl ::core::cmp::PartialEq for RotateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RotateTransform {}
impl ::core::fmt::Debug for RotateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RotateTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RotateTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.RotateTransform;{688ea9b9-1e4e-4596-86e3-428b27334faf})",
    );
}
impl ::core::clone::Clone for RotateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RotateTransform {
    type Vtable = IRotateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RotateTransform {
    const IID: ::windows_core::GUID = <IRotateTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RotateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RotateTransform";
}
::windows_core::imp::interface_hierarchy!(
    RotateTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for RotateTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for RotateTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RotateTransform {}
unsafe impl ::core::marker::Send for RotateTransform {}
unsafe impl ::core::marker::Sync for RotateTransform {}
#[repr(transparent)]
pub struct ScaleTransform(::windows_core::IUnknown);
impl ScaleTransform {}
impl ::core::cmp::PartialEq for ScaleTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScaleTransform {}
impl ::core::fmt::Debug for ScaleTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScaleTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ScaleTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.ScaleTransform;{ed67f18d-936e-43ab-929a-e9cd0a511e52})",
    );
}
impl ::core::clone::Clone for ScaleTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ScaleTransform {
    type Vtable = IScaleTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScaleTransform {
    const IID: ::windows_core::GUID = <IScaleTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScaleTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ScaleTransform";
}
::windows_core::imp::interface_hierarchy!(
    ScaleTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for ScaleTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for ScaleTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ScaleTransform {}
unsafe impl ::core::marker::Send for ScaleTransform {}
unsafe impl ::core::marker::Sync for ScaleTransform {}
#[repr(transparent)]
pub struct Shadow(::windows_core::IUnknown);
impl Shadow {}
impl ::core::cmp::PartialEq for Shadow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Shadow {}
impl ::core::fmt::Debug for Shadow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Shadow").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Shadow {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.Shadow;{6813a583-f3b4-5fcf-8694-2cd0aefc2fad})",
    );
}
impl ::core::clone::Clone for Shadow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Shadow {
    type Vtable = IShadow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Shadow {
    const IID: ::windows_core::GUID = <IShadow as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Shadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Shadow";
}
::windows_core::imp::interface_hierarchy!(
    Shadow, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Shadow {}
unsafe impl ::core::marker::Send for Shadow {}
unsafe impl ::core::marker::Sync for Shadow {}
#[repr(transparent)]
pub struct SkewTransform(::windows_core::IUnknown);
impl SkewTransform {}
impl ::core::cmp::PartialEq for SkewTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SkewTransform {}
impl ::core::fmt::Debug for SkewTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SkewTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SkewTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.SkewTransform;{4e8a3b15-7a0f-4617-9e98-1e65bdc92115})",
    );
}
impl ::core::clone::Clone for SkewTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SkewTransform {
    type Vtable = ISkewTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SkewTransform {
    const IID: ::windows_core::GUID = <ISkewTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SkewTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SkewTransform";
}
::windows_core::imp::interface_hierarchy!(
    SkewTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for SkewTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for SkewTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for SkewTransform {}
unsafe impl ::core::marker::Send for SkewTransform {}
unsafe impl ::core::marker::Sync for SkewTransform {}
#[repr(transparent)]
pub struct SolidColorBrush(::windows_core::IUnknown);
impl SolidColorBrush {}
impl ::core::cmp::PartialEq for SolidColorBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SolidColorBrush {}
impl ::core::fmt::Debug for SolidColorBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SolidColorBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SolidColorBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.SolidColorBrush;{9d850850-66f3-48df-9a8f-824bd5e070af})",
    );
}
impl ::core::clone::Clone for SolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SolidColorBrush {
    type Vtable = ISolidColorBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SolidColorBrush {
    const IID: ::windows_core::GUID = <ISolidColorBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SolidColorBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SolidColorBrush";
}
::windows_core::imp::interface_hierarchy!(
    SolidColorBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Brush> for SolidColorBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for SolidColorBrush {}
unsafe impl ::core::marker::Send for SolidColorBrush {}
unsafe impl ::core::marker::Sync for SolidColorBrush {}
#[repr(transparent)]
pub struct ThemeShadow(::windows_core::IUnknown);
impl ThemeShadow {}
impl ::core::cmp::PartialEq for ThemeShadow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ThemeShadow {}
impl ::core::fmt::Debug for ThemeShadow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThemeShadow").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ThemeShadow {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.ThemeShadow;{3eccad09-7985-5f39-8b62-6c10696dca6f})",
    );
}
impl ::core::clone::Clone for ThemeShadow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ThemeShadow {
    type Vtable = IThemeShadow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ThemeShadow {
    const IID: ::windows_core::GUID = <IThemeShadow as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ThemeShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ThemeShadow";
}
::windows_core::imp::interface_hierarchy!(
    ThemeShadow, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Shadow> for ThemeShadow {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ThemeShadow {}
unsafe impl ::core::marker::Send for ThemeShadow {}
unsafe impl ::core::marker::Sync for ThemeShadow {}
#[repr(transparent)]
pub struct TileBrush(::windows_core::IUnknown);
impl TileBrush {}
impl ::core::cmp::PartialEq for TileBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileBrush {}
impl ::core::fmt::Debug for TileBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileBrush").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TileBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TileBrush;{c201cf06-cd84-48a5-9607-664d7361cd61})",
    );
}
impl ::core::clone::Clone for TileBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TileBrush {
    type Vtable = ITileBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TileBrush {
    const IID: ::windows_core::GUID = <ITileBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TileBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TileBrush";
}
::windows_core::imp::interface_hierarchy!(
    TileBrush, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Brush> for TileBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for TileBrush {}
unsafe impl ::core::marker::Send for TileBrush {}
unsafe impl ::core::marker::Sync for TileBrush {}
#[repr(transparent)]
pub struct TimelineMarker(::windows_core::IUnknown);
impl TimelineMarker {}
impl ::core::cmp::PartialEq for TimelineMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarker {}
impl ::core::fmt::Debug for TimelineMarker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarker").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimelineMarker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TimelineMarker;{a68ef02d-45ba-4e50-8cad-aaea3a227af5})",
    );
}
impl ::core::clone::Clone for TimelineMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimelineMarker {
    type Vtable = ITimelineMarker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimelineMarker {
    const IID: ::windows_core::GUID = <ITimelineMarker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimelineMarker {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarker";
}
::windows_core::imp::interface_hierarchy!(
    TimelineMarker, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for TimelineMarker {}
unsafe impl ::core::marker::Send for TimelineMarker {}
unsafe impl ::core::marker::Sync for TimelineMarker {}
#[repr(transparent)]
pub struct TimelineMarkerCollection(::windows_core::IUnknown);
impl TimelineMarkerCollection {}
impl ::core::cmp::PartialEq for TimelineMarkerCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarkerCollection {}
impl ::core::fmt::Debug for TimelineMarkerCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarkerCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimelineMarkerCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TimelineMarkerCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.TimelineMarker;{a68ef02d-45ba-4e50-8cad-aaea3a227af5})))",
    );
}
impl ::core::clone::Clone for TimelineMarkerCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimelineMarkerCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<
        TimelineMarker,
    >;
}
unsafe impl ::windows_core::ComInterface for TimelineMarkerCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        TimelineMarker,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimelineMarkerCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarkerCollection";
}
impl ::core::iter::IntoIterator for TimelineMarkerCollection {
    type Item = TimelineMarker;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &TimelineMarkerCollection {
    type Item = TimelineMarker;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    TimelineMarkerCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<TimelineMarker>,
> for TimelineMarkerCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<TimelineMarker>,
> for TimelineMarkerCollection {}
unsafe impl ::core::marker::Send for TimelineMarkerCollection {}
unsafe impl ::core::marker::Sync for TimelineMarkerCollection {}
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventArgs(::windows_core::IUnknown);
impl TimelineMarkerRoutedEventArgs {}
impl ::core::cmp::PartialEq for TimelineMarkerRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarkerRoutedEventArgs {}
impl ::core::fmt::Debug for TimelineMarkerRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarkerRoutedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimelineMarkerRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TimelineMarkerRoutedEventArgs;{7c3b3ef3-2c88-4d9c-99b6-46cdbd48d4c1})",
    );
}
impl ::core::clone::Clone for TimelineMarkerRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimelineMarkerRoutedEventArgs {
    type Vtable = ITimelineMarkerRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimelineMarkerRoutedEventArgs {
    const IID: ::windows_core::GUID = <ITimelineMarkerRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimelineMarkerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarkerRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    TimelineMarkerRoutedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs>
for TimelineMarkerRoutedEventArgs {}
unsafe impl ::core::marker::Send for TimelineMarkerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TimelineMarkerRoutedEventArgs {}
#[repr(transparent)]
pub struct Transform(::windows_core::IUnknown);
impl Transform {}
impl ::core::cmp::PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Transform {}
impl ::core::fmt::Debug for Transform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Transform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Transform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.Transform;{4df74078-bfd6-4ed1-9682-d2fd8bf2fe6f})",
    );
}
impl ::core::clone::Clone for Transform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Transform {
    type Vtable = ITransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Transform {
    const IID: ::windows_core::GUID = <ITransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Transform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Transform";
}
::windows_core::imp::interface_hierarchy!(
    Transform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<GeneralTransform> for Transform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Transform {}
unsafe impl ::core::marker::Send for Transform {}
unsafe impl ::core::marker::Sync for Transform {}
#[repr(transparent)]
pub struct TransformCollection(::windows_core::IUnknown);
impl TransformCollection {}
impl ::core::cmp::PartialEq for TransformCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformCollection {}
impl ::core::fmt::Debug for TransformCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TransformCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TransformCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Transform;{4df74078-bfd6-4ed1-9682-d2fd8bf2fe6f})))",
    );
}
impl ::core::clone::Clone for TransformCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TransformCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<Transform>;
}
unsafe impl ::windows_core::ComInterface for TransformCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<
        Transform,
    > as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TransformCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TransformCollection";
}
impl ::core::iter::IntoIterator for TransformCollection {
    type Item = Transform;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &TransformCollection {
    type Item = Transform;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<
        Self::Item,
    >;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    TransformCollection, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IIterable<Transform>,
> for TransformCollection {}
impl ::windows_core::CanTryInto<
    super::super::super::Foundation::Collections::IVector<Transform>,
> for TransformCollection {}
unsafe impl ::core::marker::Send for TransformCollection {}
unsafe impl ::core::marker::Sync for TransformCollection {}
#[repr(transparent)]
pub struct TransformGroup(::windows_core::IUnknown);
impl TransformGroup {}
impl ::core::cmp::PartialEq for TransformGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformGroup {}
impl ::core::fmt::Debug for TransformGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformGroup").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TransformGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TransformGroup;{63418ccc-8d2d-4737-b951-2afce1ddc4c4})",
    );
}
impl ::core::clone::Clone for TransformGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TransformGroup {
    type Vtable = ITransformGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TransformGroup {
    const IID: ::windows_core::GUID = <ITransformGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TransformGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TransformGroup";
}
::windows_core::imp::interface_hierarchy!(
    TransformGroup, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for TransformGroup {}
impl ::windows_core::CanTryInto<GeneralTransform> for TransformGroup {}
impl ::windows_core::CanTryInto<super::DependencyObject> for TransformGroup {}
unsafe impl ::core::marker::Send for TransformGroup {}
unsafe impl ::core::marker::Sync for TransformGroup {}
#[repr(transparent)]
pub struct TranslateTransform(::windows_core::IUnknown);
impl TranslateTransform {}
impl ::core::cmp::PartialEq for TranslateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TranslateTransform {}
impl ::core::fmt::Debug for TranslateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranslateTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TranslateTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.TranslateTransform;{c975905c-3c36-4229-817b-178f64c0e113})",
    );
}
impl ::core::clone::Clone for TranslateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TranslateTransform {
    type Vtable = ITranslateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TranslateTransform {
    const IID: ::windows_core::GUID = <ITranslateTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TranslateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TranslateTransform";
}
::windows_core::imp::interface_hierarchy!(
    TranslateTransform, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for TranslateTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for TranslateTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for TranslateTransform {}
unsafe impl ::core::marker::Send for TranslateTransform {}
unsafe impl ::core::marker::Sync for TranslateTransform {}
#[repr(transparent)]
pub struct VisualTreeHelper(::windows_core::IUnknown);
impl VisualTreeHelper {}
impl ::core::cmp::PartialEq for VisualTreeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualTreeHelper {}
impl ::core::fmt::Debug for VisualTreeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualTreeHelper").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VisualTreeHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.VisualTreeHelper;{24b935e3-52c7-4141-8bac-a73d06130569})",
    );
}
impl ::core::clone::Clone for VisualTreeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VisualTreeHelper {
    type Vtable = IVisualTreeHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualTreeHelper {
    const IID: ::windows_core::GUID = <IVisualTreeHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualTreeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.VisualTreeHelper";
}
::windows_core::imp::interface_hierarchy!(
    VisualTreeHelper, ::windows_core::IUnknown, ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for VisualTreeHelper {}
unsafe impl ::core::marker::Sync for VisualTreeHelper {}
#[repr(transparent)]
pub struct XamlCompositionBrushBase(::windows_core::IUnknown);
impl XamlCompositionBrushBase {}
impl ::core::cmp::PartialEq for XamlCompositionBrushBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlCompositionBrushBase {}
impl ::core::fmt::Debug for XamlCompositionBrushBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlCompositionBrushBase").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XamlCompositionBrushBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.XamlCompositionBrushBase;{03e432d9-b35c-4a79-811c-c5652004da0e})",
    );
}
impl ::core::clone::Clone for XamlCompositionBrushBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlCompositionBrushBase {
    const IID: ::windows_core::GUID = <IXamlCompositionBrushBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlCompositionBrushBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.XamlCompositionBrushBase";
}
::windows_core::imp::interface_hierarchy!(
    XamlCompositionBrushBase, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Brush> for XamlCompositionBrushBase {}
impl ::windows_core::CanTryInto<super::DependencyObject> for XamlCompositionBrushBase {}
unsafe impl ::core::marker::Send for XamlCompositionBrushBase {}
unsafe impl ::core::marker::Sync for XamlCompositionBrushBase {}
#[repr(transparent)]
pub struct XamlLight(::windows_core::IUnknown);
impl XamlLight {}
impl ::core::cmp::PartialEq for XamlLight {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlLight {}
impl ::core::fmt::Debug for XamlLight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlLight").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XamlLight {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"rc(Windows.UI.Xaml.Media.XamlLight;{0cc3fc1f-b327-4a18-9648-7c84db26ce22})",
    );
}
impl ::core::clone::Clone for XamlLight {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for XamlLight {
    type Vtable = IXamlLight_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlLight {
    const IID: ::windows_core::GUID = <IXamlLight as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlLight {
    const NAME: &'static str = "Windows.UI.Xaml.Media.XamlLight";
}
::windows_core::imp::interface_hierarchy!(
    XamlLight, ::windows_core::IUnknown, ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for XamlLight {}
unsafe impl ::core::marker::Send for XamlLight {}
unsafe impl ::core::marker::Sync for XamlLight {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AcrylicBackgroundSource(pub i32);
impl AcrylicBackgroundSource {
    pub const HostBackdrop: Self = Self(0i32);
    pub const Backdrop: Self = Self(1i32);
}
impl ::core::marker::Copy for AcrylicBackgroundSource {}
impl ::core::clone::Clone for AcrylicBackgroundSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AcrylicBackgroundSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AcrylicBackgroundSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AcrylicBackgroundSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AcrylicBackgroundSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AcrylicBackgroundSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.AcrylicBackgroundSource;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentX {}
impl ::core::clone::Clone for AlignmentX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlignmentX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AlignmentX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AlignmentX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlignmentX").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AlignmentX {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.AlignmentX;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentY {}
impl ::core::clone::Clone for AlignmentY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlignmentY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AlignmentY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AlignmentY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlignmentY").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AlignmentY {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.AlignmentY;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioCategory(pub i32);
impl AudioCategory {
    pub const Other: Self = Self(0i32);
    pub const ForegroundOnlyMedia: Self = Self(1i32);
    pub const BackgroundCapableMedia: Self = Self(2i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for AudioCategory {}
impl ::core::clone::Clone for AudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.AudioCategory;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioDeviceType(pub i32);
impl AudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDeviceType {}
impl ::core::clone::Clone for AudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioDeviceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDeviceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.AudioDeviceType;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const RelativeToBoundingBox: Self = Self(1i32);
}
impl ::core::marker::Copy for BrushMappingMode {}
impl ::core::clone::Clone for BrushMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BrushMappingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BrushMappingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BrushMappingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrushMappingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BrushMappingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.BrushMappingMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: Self = Self(0i32);
    pub const SRgbLinearInterpolation: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorInterpolationMode {}
impl ::core::clone::Clone for ColorInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ColorInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ColorInterpolationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ColorInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorInterpolationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ColorInterpolationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.ColorInterpolationMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const MinBlend: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementCompositeMode {}
impl ::core::clone::Clone for ElementCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementCompositeMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementCompositeMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementCompositeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositeMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementCompositeMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.ElementCompositeMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: Self = Self(0i32);
    pub const Hide: Self = Self(1i32);
    pub const Disable: Self = Self(2i32);
}
impl ::core::marker::Copy for FastPlayFallbackBehaviour {}
impl ::core::clone::Clone for FastPlayFallbackBehaviour {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FastPlayFallbackBehaviour {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FastPlayFallbackBehaviour {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FastPlayFallbackBehaviour {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FastPlayFallbackBehaviour").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FastPlayFallbackBehaviour {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.FastPlayFallbackBehaviour;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: Self = Self(0i32);
    pub const Nonzero: Self = Self(1i32);
}
impl ::core::marker::Copy for FillRule {}
impl ::core::clone::Clone for FillRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FillRule {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FillRule {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FillRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FillRule").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FillRule {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.FillRule;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: Self = Self(0i32);
    pub const Reflect: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for GradientSpreadMethod {}
impl ::core::clone::Clone for GradientSpreadMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GradientSpreadMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GradientSpreadMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GradientSpreadMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientSpreadMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GradientSpreadMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.GradientSpreadMethod;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for LoadedImageSourceLoadStatus {}
impl ::core::clone::Clone for LoadedImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoadedImageSourceLoadStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LoadedImageSourceLoadStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LoadedImageSourceLoadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSourceLoadStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoadedImageSourceLoadStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.LoadedImageSourceLoadStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCanPlayResponse(pub i32);
impl MediaCanPlayResponse {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCanPlayResponse {}
impl ::core::clone::Clone for MediaCanPlayResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCanPlayResponse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCanPlayResponse {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCanPlayResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCanPlayResponse").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCanPlayResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.MediaCanPlayResponse;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaElementState(pub i32);
impl MediaElementState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaElementState {}
impl ::core::clone::Clone for MediaElementState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaElementState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaElementState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaElementState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaElementState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaElementState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.MediaElementState;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for PenLineCap {}
impl ::core::clone::Clone for PenLineCap {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenLineCap {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PenLineCap {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PenLineCap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenLineCap").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PenLineCap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.PenLineCap;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
}
impl ::core::marker::Copy for PenLineJoin {}
impl ::core::clone::Clone for PenLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenLineJoin {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PenLineJoin {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PenLineJoin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenLineJoin").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PenLineJoin {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.PenLineJoin;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RevealBrushState(pub i32);
impl RevealBrushState {
    pub const Normal: Self = Self(0i32);
    pub const PointerOver: Self = Self(1i32);
    pub const Pressed: Self = Self(2i32);
}
impl ::core::marker::Copy for RevealBrushState {}
impl ::core::clone::Clone for RevealBrushState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RevealBrushState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RevealBrushState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RevealBrushState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBrushState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RevealBrushState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.RevealBrushState;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Stereo3DVideoPackingMode(pub i32);
impl Stereo3DVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for Stereo3DVideoPackingMode {}
impl ::core::clone::Clone for Stereo3DVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stereo3DVideoPackingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Stereo3DVideoPackingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Stereo3DVideoPackingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stereo3DVideoPackingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Stereo3DVideoPackingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.Stereo3DVideoPackingMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Stereo3DVideoRenderMode(pub i32);
impl Stereo3DVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for Stereo3DVideoRenderMode {}
impl ::core::clone::Clone for Stereo3DVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stereo3DVideoRenderMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Stereo3DVideoRenderMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Stereo3DVideoRenderMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stereo3DVideoRenderMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Stereo3DVideoRenderMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.Stereo3DVideoRenderMode;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for Stretch {}
impl ::core::clone::Clone for Stretch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stretch {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Stretch {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Stretch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stretch").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Stretch {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.Stretch;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: Self = Self(0i32);
    pub const BoldSimulation: Self = Self(1i32);
    pub const ItalicSimulation: Self = Self(2i32);
    pub const BoldItalicSimulation: Self = Self(3i32);
}
impl ::core::marker::Copy for StyleSimulations {}
impl ::core::clone::Clone for StyleSimulations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StyleSimulations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StyleSimulations {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StyleSimulations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StyleSimulations").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StyleSimulations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.StyleSimulations;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
impl ::core::marker::Copy for SweepDirection {}
impl ::core::clone::Clone for SweepDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SweepDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SweepDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SweepDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SweepDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SweepDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Media.SweepDirection;i4)",
    );
}
#[repr(C)]
pub struct Matrix {
    pub M11: f64,
    pub M12: f64,
    pub M21: f64,
    pub M22: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
}
impl ::core::marker::Copy for Matrix {}
impl ::core::clone::Clone for Matrix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Matrix")
            .field("M11", &self.M11)
            .field("M12", &self.M12)
            .field("M21", &self.M21)
            .field("M22", &self.M22)
            .field("OffsetX", &self.OffsetX)
            .field("OffsetY", &self.OffsetY)
            .finish()
    }
}
impl ::windows_core::TypeKind for Matrix {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for Matrix {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Media.Matrix;f8;f8;f8;f8;f8;f8)",
    );
}
impl ::core::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21
            && self.M22 == other.M22 && self.OffsetX == other.OffsetX
            && self.OffsetY == other.OffsetY
    }
}
impl ::core::cmp::Eq for Matrix {}
impl ::core::default::Default for Matrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct RateChangedRoutedEventHandler(pub ::windows_core::IUnknown);
impl RateChangedRoutedEventHandler {}
#[repr(C)]
struct RateChangedRoutedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RateChangedRoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const RateChangedRoutedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RateChangedRoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> RateChangedRoutedEventHandlerBox<F> {
    const VTABLE: RateChangedRoutedEventHandler_Vtbl = RateChangedRoutedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: &::windows_core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid
            == &<RateChangedRoutedEventHandler as ::windows_core::ComInterface>::IID
            || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || iid
                == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
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
impl ::core::cmp::PartialEq for RateChangedRoutedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RateChangedRoutedEventHandler {}
impl ::core::fmt::Debug for RateChangedRoutedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RateChangedRoutedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RateChangedRoutedEventHandler {
    type Vtable = RateChangedRoutedEventHandler_Vtbl;
}
impl ::core::clone::Clone for RateChangedRoutedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for RateChangedRoutedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x08e9a257_ae05_489b_8839_28c6225d2349,
    );
}
impl ::windows_core::RuntimeType for RateChangedRoutedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{08e9a257-ae05-489b-8839-28c6225d2349}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct RateChangedRoutedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventHandler(pub ::windows_core::IUnknown);
impl TimelineMarkerRoutedEventHandler {}
#[repr(C)]
struct TimelineMarkerRoutedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&TimelineMarkerRoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const TimelineMarkerRoutedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[allow(dead_code)]
impl<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&TimelineMarkerRoutedEventArgs>,
        ) -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> TimelineMarkerRoutedEventHandlerBox<F> {
    const VTABLE: TimelineMarkerRoutedEventHandler_Vtbl = TimelineMarkerRoutedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: &::windows_core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid
            == &<TimelineMarkerRoutedEventHandler as ::windows_core::ComInterface>::IID
            || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || iid
                == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
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
impl ::core::cmp::PartialEq for TimelineMarkerRoutedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarkerRoutedEventHandler {}
impl ::core::fmt::Debug for TimelineMarkerRoutedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarkerRoutedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for TimelineMarkerRoutedEventHandler {
    type Vtable = TimelineMarkerRoutedEventHandler_Vtbl;
}
impl ::core::clone::Clone for TimelineMarkerRoutedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for TimelineMarkerRoutedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(
        0x72e2fa9c_6dea_4cbe_a159_06ce95fbeced,
    );
}
impl ::windows_core::RuntimeType for TimelineMarkerRoutedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(
        b"{72e2fa9c-6dea-4cbe-a159-06ce95fbeced}",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct TimelineMarkerRoutedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
