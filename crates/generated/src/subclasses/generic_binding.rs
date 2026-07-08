//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：GenericBindingIsPPtrCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GenericBindingIsPPtrCurveVariant {
    Default(u8),
    V5_6(u8),
    V2022_1(u8),
}

impl crate::Parse for GenericBindingIsPPtrCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2022_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GenericBindingIsIntCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GenericBindingIsIntCurveVariant {
    Default(u8),
    V2022_2(u8),
}

impl crate::Parse for GenericBindingIsIntCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GenericBindingIsSerializeReferenceCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GenericBindingIsSerializeReferenceCurveVariant {
    Default(u8),
    V6000_5(u8),
}

impl crate::Parse for GenericBindingIsSerializeReferenceCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct GenericBinding {
    pub path: u32,
    pub attribute: u32,
    pub script: PPtr,
    pub class_id: Option<u16>,
    pub type_id: Option<i32>,
    pub custom_type: u8,
    pub is_p_ptr_curve: GenericBindingIsPPtrCurveVariant,
    pub is_int_curve: Option<GenericBindingIsIntCurveVariant>,
    pub is_serialize_reference_curve: Option<GenericBindingIsSerializeReferenceCurveVariant>,
    pub meta_data: Option<u16>,
}

impl crate::Parse for GenericBinding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = rdr.read_u32()?;
        let attribute = rdr.read_u32()?;
        let script = crate::PPtr::parse(rdr, ver)?;
        let class_id = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(rdr.read_u16()?)
        } else {
            None
        };
        let type_id = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let custom_type = rdr.read_u8()?;
        let is_p_ptr_curve = GenericBindingIsPPtrCurveVariant::parse(rdr, ver)?;
        let is_int_curve = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(GenericBindingIsIntCurveVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let is_serialize_reference_curve = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(GenericBindingIsSerializeReferenceCurveVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let meta_data = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_u16()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(GenericBinding {
            path,
            attribute,
            script,
            class_id,
            type_id,
            custom_type,
            is_p_ptr_curve,
            is_int_curve,
            is_serialize_reference_curve,
            meta_data,
        })
    }
}
