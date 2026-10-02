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

impl crate::ToValue for GenericBindingIsPPtrCurveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for GenericBindingIsIntCurveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for GenericBindingIsSerializeReferenceCurveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
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
        let mut path = None;
        let mut attribute = None;
        let mut script = None;
        let mut class_id = None;
        let mut type_id = None;
        let mut custom_type = None;
        let mut is_p_ptr_curve = None;
        let mut is_int_curve = None;
        let mut is_serialize_reference_curve = None;
        let mut meta_data = None;
        path = Some(rdr.read_u32()?);
        attribute = Some(rdr.read_u32()?);
        script = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            type_id = Some(rdr.read_i32()?);
        } else {
            class_id = Some(rdr.read_u16()?);
        }
        custom_type = Some(rdr.read_u8()?);
        is_p_ptr_curve = Some(GenericBindingIsPPtrCurveVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            is_int_curve = Some(GenericBindingIsIntCurveVariant::parse(rdr, ver)?);
            is_serialize_reference_curve = Some(
                GenericBindingIsSerializeReferenceCurveVariant::parse(rdr, ver)?,
            );
            meta_data = Some(rdr.read_u16()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            is_int_curve = Some(GenericBindingIsIntCurveVariant::parse(rdr, ver)?);
            is_serialize_reference_curve = Some(
                GenericBindingIsSerializeReferenceCurveVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            is_int_curve = Some(GenericBindingIsIntCurveVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(GenericBinding {
            path: path.unwrap(),
            attribute: attribute.unwrap(),
            script: script.unwrap(),
            class_id,
            type_id,
            custom_type: custom_type.unwrap(),
            is_p_ptr_curve: is_p_ptr_curve.unwrap(),
            is_int_curve,
            is_serialize_reference_curve,
            meta_data,
        })
    }
}

impl crate::ToValue for GenericBinding {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("path".to_string(), self.path.to_value())),
            Some(("attribute".to_string(), self.attribute.to_value())),
            Some(("script".to_string(), self.script.to_value())),
            crate::opt_field("classID", &self.class_id),
            crate::opt_field("typeID", &self.type_id),
            Some(("customType".to_string(), self.custom_type.to_value())),
            Some(("isPPtrCurve".to_string(), self.is_p_ptr_curve.to_value())),
            crate::opt_field("isIntCurve", &self.is_int_curve),
            crate::opt_field(
                "isSerializeReferenceCurve",
                &self.is_serialize_reference_curve,
            ),
            crate::opt_field("metaData", &self.meta_data),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "path" => (self.path).inspect_value(path, offset, limit),
                "attribute" => (self.attribute).inspect_value(path, offset, limit),
                "script" => (self.script).inspect_value(path, offset, limit),
                "classID" => (self.class_id).inspect_value(path, offset, limit),
                "typeID" => (self.type_id).inspect_value(path, offset, limit),
                "customType" => (self.custom_type).inspect_value(path, offset, limit),
                "isPPtrCurve" => (self.is_p_ptr_curve).inspect_value(path, offset, limit),
                "isIntCurve" => (self.is_int_curve).inspect_value(path, offset, limit),
                "isSerializeReferenceCurve" => {
                    (self.is_serialize_reference_curve).inspect_value(path, offset, limit)
                }
                "metaData" => (self.meta_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.path).inspect_value(path, offset, limit)).map(|v| ("path".to_string(), v)),
            ((self.attribute).inspect_value(path, offset, limit))
                .map(|v| ("attribute".to_string(), v)),
            ((self.script).inspect_value(path, offset, limit)).map(|v| ("script".to_string(), v)),
            ((self.class_id).inspect_value(path, offset, limit))
                .map(|v| ("classID".to_string(), v)),
            ((self.type_id).inspect_value(path, offset, limit)).map(|v| ("typeID".to_string(), v)),
            ((self.custom_type).inspect_value(path, offset, limit))
                .map(|v| ("customType".to_string(), v)),
            ((self.is_p_ptr_curve).inspect_value(path, offset, limit))
                .map(|v| ("isPPtrCurve".to_string(), v)),
            ((self.is_int_curve).inspect_value(path, offset, limit))
                .map(|v| ("isIntCurve".to_string(), v)),
            ((self.is_serialize_reference_curve).inspect_value(path, offset, limit))
                .map(|v| ("isSerializeReferenceCurve".to_string(), v)),
            ((self.meta_data).inspect_value(path, offset, limit))
                .map(|v| ("metaData".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
