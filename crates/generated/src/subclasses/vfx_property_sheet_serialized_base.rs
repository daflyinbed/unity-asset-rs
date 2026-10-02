//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_field_vfx_entry_exposed_animation_curve::VfxField_VfxEntryExposed_AnimationCurve;
use crate::subclasses::vfx_field_vfx_entry_exposed_bool::VfxField_VfxEntryExposed_bool;
use crate::subclasses::vfx_field_vfx_entry_exposed_float::VfxField_VfxEntryExposed_float;
use crate::subclasses::vfx_field_vfx_entry_exposed_gradient::VfxField_VfxEntryExposed_Gradient;
use crate::subclasses::vfx_field_vfx_entry_exposed_matrix4x4f::VfxField_VfxEntryExposed_Matrix4x4f;
use crate::subclasses::vfx_field_vfx_entry_exposed_p_ptr_named_object_::VfxField_VfxEntryExposed_PPtr_NamedObject_;
use crate::subclasses::vfx_field_vfx_entry_exposed_p_ptr_object_::VfxField_VfxEntryExposed_PPtr_Object_;
use crate::subclasses::vfx_field_vfx_entry_exposed_s_int32::VfxField_VfxEntryExposed_SInt32;
use crate::subclasses::vfx_field_vfx_entry_exposed_u_int32::VfxField_VfxEntryExposed_UInt32;
use crate::subclasses::vfx_field_vfx_entry_exposed_vector2f::VfxField_VfxEntryExposed_Vector2f;
use crate::subclasses::vfx_field_vfx_entry_exposed_vector3f::VfxField_VfxEntryExposed_Vector3f;
use crate::subclasses::vfx_field_vfx_entry_exposed_vector4f::VfxField_VfxEntryExposed_Vector4f;
use crate::subclasses::vfx_field_vfx_entry_expression_value_animation_curve::VfxField_VfxEntryExpressionValue_AnimationCurve;
use crate::subclasses::vfx_field_vfx_entry_expression_value_bool::VfxField_VfxEntryExpressionValue_bool;
use crate::subclasses::vfx_field_vfx_entry_expression_value_float::VfxField_VfxEntryExpressionValue_float;
use crate::subclasses::vfx_field_vfx_entry_expression_value_gradient::VfxField_VfxEntryExpressionValue_Gradient;
use crate::subclasses::vfx_field_vfx_entry_expression_value_matrix4x4f::VfxField_VfxEntryExpressionValue_Matrix4x4f;
use crate::subclasses::vfx_field_vfx_entry_expression_value_p_ptr_named_object_::VfxField_VfxEntryExpressionValue_PPtr_NamedObject_;
use crate::subclasses::vfx_field_vfx_entry_expression_value_p_ptr_object_::VfxField_VfxEntryExpressionValue_PPtr_Object_;
use crate::subclasses::vfx_field_vfx_entry_expression_value_s_int32::VfxField_VfxEntryExpressionValue_SInt32;
use crate::subclasses::vfx_field_vfx_entry_expression_value_u_int32::VfxField_VfxEntryExpressionValue_UInt32;
use crate::subclasses::vfx_field_vfx_entry_expression_value_vector2f::VfxField_VfxEntryExpressionValue_Vector2f;
use crate::subclasses::vfx_field_vfx_entry_expression_value_vector3f::VfxField_VfxEntryExpressionValue_Vector3f;
use crate::subclasses::vfx_field_vfx_entry_expression_value_vector4f::VfxField_VfxEntryExpressionValue_Vector4f;

/// 版本枚举：VfxPropertySheetSerializedBaseMFloatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMFloatVariant {
    Default(Box<VfxField_VfxEntryExposed_float>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_float>),
    V2019_3(Box<VfxField_VfxEntryExposed_float>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_float>),
    V2020_2(Box<VfxField_VfxEntryExposed_float>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_float>),
    V2022_2(Box<VfxField_VfxEntryExposed_float>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_float>),
    V6000_5(Box<VfxField_VfxEntryExposed_float>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_float>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMFloatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_float::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_float::parse(rdr, ver)?)) })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_float::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_float::parse(rdr, ver)?)) })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_float::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_float::parse(rdr, ver)?)) })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_float::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_float::parse(rdr, ver)?)) })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_float::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_float::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMFloatVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMVector2fVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMVector2fVariant {
    Default(Box<VfxField_VfxEntryExposed_Vector2f>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_Vector2f>),
    V2019_3(Box<VfxField_VfxEntryExposed_Vector2f>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_Vector2f>),
    V2020_2(Box<VfxField_VfxEntryExposed_Vector2f>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_Vector2f>),
    V2022_2(Box<VfxField_VfxEntryExposed_Vector2f>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_Vector2f>),
    V6000_5(Box<VfxField_VfxEntryExposed_Vector2f>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_Vector2f>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMVector2fVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector2f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMVector2fVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMVector3fVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMVector3fVariant {
    Default(Box<VfxField_VfxEntryExposed_Vector3f>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_Vector3f>),
    V2019_3(Box<VfxField_VfxEntryExposed_Vector3f>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_Vector3f>),
    V2020_2(Box<VfxField_VfxEntryExposed_Vector3f>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_Vector3f>),
    V2022_2(Box<VfxField_VfxEntryExposed_Vector3f>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_Vector3f>),
    V6000_5(Box<VfxField_VfxEntryExposed_Vector3f>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_Vector3f>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMVector3fVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector3f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMVector3fVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMVector4fVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMVector4fVariant {
    Default(Box<VfxField_VfxEntryExposed_Vector4f>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_Vector4f>),
    V2019_3(Box<VfxField_VfxEntryExposed_Vector4f>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_Vector4f>),
    V2020_2(Box<VfxField_VfxEntryExposed_Vector4f>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_Vector4f>),
    V2022_2(Box<VfxField_VfxEntryExposed_Vector4f>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_Vector4f>),
    V6000_5(Box<VfxField_VfxEntryExposed_Vector4f>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_Vector4f>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMVector4fVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Vector4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMVector4fVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMUintVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMUintVariant {
    Default(Box<VfxField_VfxEntryExposed_UInt32>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_UInt32>),
    V2019_3(Box<VfxField_VfxEntryExposed_UInt32>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_UInt32>),
    V2020_2(Box<VfxField_VfxEntryExposed_UInt32>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_UInt32>),
    V2022_2(Box<VfxField_VfxEntryExposed_UInt32>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_UInt32>),
    V6000_5(Box<VfxField_VfxEntryExposed_UInt32>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_UInt32>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMUintVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_UInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_UInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_UInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_UInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_UInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_UInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_UInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_UInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_UInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_UInt32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMUintVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMIntVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMIntVariant {
    Default(Box<VfxField_VfxEntryExposed_SInt32>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_SInt32>),
    V2019_3(Box<VfxField_VfxEntryExposed_SInt32>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_SInt32>),
    V2020_2(Box<VfxField_VfxEntryExposed_SInt32>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_SInt32>),
    V2022_2(Box<VfxField_VfxEntryExposed_SInt32>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_SInt32>),
    V6000_5(Box<VfxField_VfxEntryExposed_SInt32>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_SInt32>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMIntVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_SInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_SInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_SInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_SInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_SInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_SInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_SInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_SInt32::parse(rdr, ver)?)) })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_SInt32::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_SInt32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMIntVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMMatrix4x4fVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMMatrix4x4fVariant {
    Default(Box<VfxField_VfxEntryExposed_Matrix4x4f>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_Matrix4x4f>),
    V2019_3(Box<VfxField_VfxEntryExposed_Matrix4x4f>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_Matrix4x4f>),
    V2020_2(Box<VfxField_VfxEntryExposed_Matrix4x4f>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_Matrix4x4f>),
    V2022_2(Box<VfxField_VfxEntryExposed_Matrix4x4f>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_Matrix4x4f>),
    V6000_5(Box<VfxField_VfxEntryExposed_Matrix4x4f>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_Matrix4x4f>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMMatrix4x4fVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_Matrix4x4f::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Matrix4x4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_Matrix4x4f::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Matrix4x4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_Matrix4x4f::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Matrix4x4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_Matrix4x4f::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Matrix4x4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_Matrix4x4f::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Matrix4x4f::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMMatrix4x4fVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMAnimationCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMAnimationCurveVariant {
    Default(Box<VfxField_VfxEntryExposed_AnimationCurve>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_AnimationCurve>),
    V2019_3(Box<VfxField_VfxEntryExposed_AnimationCurve>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_AnimationCurve>),
    V2020_2(Box<VfxField_VfxEntryExposed_AnimationCurve>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_AnimationCurve>),
    V2022_2(Box<VfxField_VfxEntryExposed_AnimationCurve>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_AnimationCurve>),
    V6000_5(Box<VfxField_VfxEntryExposed_AnimationCurve>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_AnimationCurve>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMAnimationCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_AnimationCurve::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_AnimationCurve::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_AnimationCurve::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_AnimationCurve::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_AnimationCurve::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_AnimationCurve::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_AnimationCurve::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_AnimationCurve::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_AnimationCurve::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_AnimationCurve::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMAnimationCurveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMGradientVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMGradientVariant {
    Default(Box<VfxField_VfxEntryExposed_Gradient>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_Gradient>),
    V2019_3(Box<VfxField_VfxEntryExposed_Gradient>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_Gradient>),
    V2020_2(Box<VfxField_VfxEntryExposed_Gradient>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_Gradient>),
    V2022_2(Box<VfxField_VfxEntryExposed_Gradient>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_Gradient>),
    V6000_5(Box<VfxField_VfxEntryExposed_Gradient>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_Gradient>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMGradientVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_Gradient::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMGradientVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMNamedObjectVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMNamedObjectVariant {
    Default(Box<VfxField_VfxEntryExposed_PPtr_NamedObject_>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_PPtr_NamedObject_>),
    V2019_3(Box<VfxField_VfxEntryExposed_PPtr_NamedObject_>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_PPtr_NamedObject_>),
    V2020_2(Box<VfxField_VfxEntryExposed_PPtr_NamedObject_>),
    V2021_1(Box<VfxField_VfxEntryExposed_PPtr_Object_>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_PPtr_Object_>),
    V2022_2(Box<VfxField_VfxEntryExposed_PPtr_Object_>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_PPtr_Object_>),
    V6000_5(Box<VfxField_VfxEntryExposed_PPtr_Object_>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_PPtr_Object_>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMNamedObjectVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_PPtr_Object_::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_PPtr_Object_::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_PPtr_Object_::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_PPtr_Object_::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_PPtr_Object_::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_PPtr_Object_::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_PPtr_NamedObject_::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_PPtr_NamedObject_::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_PPtr_NamedObject_::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(
                    VfxField_VfxEntryExpressionValue_PPtr_NamedObject_::parse(rdr, ver)?,
                ))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExposed_PPtr_NamedObject_::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMNamedObjectVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VfxPropertySheetSerializedBaseMBoolVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxPropertySheetSerializedBaseMBoolVariant {
    Default(Box<VfxField_VfxEntryExposed_bool>),
    V2018_4_13(Box<VfxField_VfxEntryExpressionValue_bool>),
    V2019_3(Box<VfxField_VfxEntryExposed_bool>),
    V2020_1(Box<VfxField_VfxEntryExpressionValue_bool>),
    V2020_2(Box<VfxField_VfxEntryExposed_bool>),
    V2021_2(Box<VfxField_VfxEntryExpressionValue_bool>),
    V2022_2(Box<VfxField_VfxEntryExposed_bool>),
    V2022_3_6(Box<VfxField_VfxEntryExpressionValue_bool>),
    V6000_5(Box<VfxField_VfxEntryExposed_bool>),
    V6000_6(Box<VfxField_VfxEntryExpressionValue_bool>),
}

impl crate::Parse for VfxPropertySheetSerializedBaseMBoolVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_bool::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_bool::parse(rdr, ver)?)) })?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_bool::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_bool::parse(rdr, ver)?)) })?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_bool::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_bool::parse(rdr, ver)?)) })?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_bool::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_bool::parse(rdr, ver)?)) })?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ({
                Ok(Box::new(VfxField_VfxEntryExpressionValue_bool::parse(
                    rdr, ver,
                )?))
            })?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ({ Ok(Box::new(VfxField_VfxEntryExposed_bool::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBaseMBoolVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VfxPropertySheetSerializedBase {
    pub m_float: VfxPropertySheetSerializedBaseMFloatVariant,
    pub m_vector2f: VfxPropertySheetSerializedBaseMVector2fVariant,
    pub m_vector3f: VfxPropertySheetSerializedBaseMVector3fVariant,
    pub m_vector4f: VfxPropertySheetSerializedBaseMVector4fVariant,
    pub m_uint: VfxPropertySheetSerializedBaseMUintVariant,
    pub m_int: VfxPropertySheetSerializedBaseMIntVariant,
    pub m_matrix4x4f: VfxPropertySheetSerializedBaseMMatrix4x4fVariant,
    pub m_animation_curve: VfxPropertySheetSerializedBaseMAnimationCurveVariant,
    pub m_gradient: VfxPropertySheetSerializedBaseMGradientVariant,
    pub m_named_object: VfxPropertySheetSerializedBaseMNamedObjectVariant,
    pub m_bool: VfxPropertySheetSerializedBaseMBoolVariant,
}

impl crate::Parse for VfxPropertySheetSerializedBase {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_float = VfxPropertySheetSerializedBaseMFloatVariant::parse(rdr, ver)?;
        let m_vector2f = VfxPropertySheetSerializedBaseMVector2fVariant::parse(rdr, ver)?;
        let m_vector3f = VfxPropertySheetSerializedBaseMVector3fVariant::parse(rdr, ver)?;
        let m_vector4f = VfxPropertySheetSerializedBaseMVector4fVariant::parse(rdr, ver)?;
        let m_uint = VfxPropertySheetSerializedBaseMUintVariant::parse(rdr, ver)?;
        let m_int = VfxPropertySheetSerializedBaseMIntVariant::parse(rdr, ver)?;
        let m_matrix4x4f = VfxPropertySheetSerializedBaseMMatrix4x4fVariant::parse(rdr, ver)?;
        let m_animation_curve =
            VfxPropertySheetSerializedBaseMAnimationCurveVariant::parse(rdr, ver)?;
        let m_gradient = VfxPropertySheetSerializedBaseMGradientVariant::parse(rdr, ver)?;
        let m_named_object = VfxPropertySheetSerializedBaseMNamedObjectVariant::parse(rdr, ver)?;
        let m_bool = VfxPropertySheetSerializedBaseMBoolVariant::parse(rdr, ver)?;

        Ok(VfxPropertySheetSerializedBase {
            m_float,
            m_vector2f,
            m_vector3f,
            m_vector4f,
            m_uint,
            m_int,
            m_matrix4x4f,
            m_animation_curve,
            m_gradient,
            m_named_object,
            m_bool,
        })
    }
}

impl crate::ToValue for VfxPropertySheetSerializedBase {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Float".to_string(), self.m_float.to_value())),
            Some(("m_Vector2f".to_string(), self.m_vector2f.to_value())),
            Some(("m_Vector3f".to_string(), self.m_vector3f.to_value())),
            Some(("m_Vector4f".to_string(), self.m_vector4f.to_value())),
            Some(("m_Uint".to_string(), self.m_uint.to_value())),
            Some(("m_Int".to_string(), self.m_int.to_value())),
            Some(("m_Matrix4x4f".to_string(), self.m_matrix4x4f.to_value())),
            Some((
                "m_AnimationCurve".to_string(),
                self.m_animation_curve.to_value(),
            )),
            Some(("m_Gradient".to_string(), self.m_gradient.to_value())),
            Some(("m_NamedObject".to_string(), self.m_named_object.to_value())),
            Some(("m_Bool".to_string(), self.m_bool.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Float" => (self.m_float).inspect_value(path, offset, limit),
                "m_Vector2f" => (self.m_vector2f).inspect_value(path, offset, limit),
                "m_Vector3f" => (self.m_vector3f).inspect_value(path, offset, limit),
                "m_Vector4f" => (self.m_vector4f).inspect_value(path, offset, limit),
                "m_Uint" => (self.m_uint).inspect_value(path, offset, limit),
                "m_Int" => (self.m_int).inspect_value(path, offset, limit),
                "m_Matrix4x4f" => (self.m_matrix4x4f).inspect_value(path, offset, limit),
                "m_AnimationCurve" => (self.m_animation_curve).inspect_value(path, offset, limit),
                "m_Gradient" => (self.m_gradient).inspect_value(path, offset, limit),
                "m_NamedObject" => (self.m_named_object).inspect_value(path, offset, limit),
                "m_Bool" => (self.m_bool).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_float).inspect_value(path, offset, limit)).map(|v| ("m_Float".to_string(), v)),
            ((self.m_vector2f).inspect_value(path, offset, limit))
                .map(|v| ("m_Vector2f".to_string(), v)),
            ((self.m_vector3f).inspect_value(path, offset, limit))
                .map(|v| ("m_Vector3f".to_string(), v)),
            ((self.m_vector4f).inspect_value(path, offset, limit))
                .map(|v| ("m_Vector4f".to_string(), v)),
            ((self.m_uint).inspect_value(path, offset, limit)).map(|v| ("m_Uint".to_string(), v)),
            ((self.m_int).inspect_value(path, offset, limit)).map(|v| ("m_Int".to_string(), v)),
            ((self.m_matrix4x4f).inspect_value(path, offset, limit))
                .map(|v| ("m_Matrix4x4f".to_string(), v)),
            ((self.m_animation_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationCurve".to_string(), v)),
            ((self.m_gradient).inspect_value(path, offset, limit))
                .map(|v| ("m_Gradient".to_string(), v)),
            ((self.m_named_object).inspect_value(path, offset, limit))
                .map(|v| ("m_NamedObject".to_string(), v)),
            ((self.m_bool).inspect_value(path, offset, limit)).map(|v| ("m_Bool".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
