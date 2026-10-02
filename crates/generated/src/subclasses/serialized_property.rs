//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_texture_property::SerializedTextureProperty;

/// 版本枚举：SerializedPropertyMAttributesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedPropertyMAttributesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for SerializedPropertyMAttributesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SerializedPropertyMAttributesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedProperty {
    pub m_name: String,
    pub m_description: String,
    pub m_attributes: SerializedPropertyMAttributesVariant,
    pub m_type: i32,
    pub m_flags: u32,
    pub m_def_value_0: f32,
    pub m_def_value_1: f32,
    pub m_def_value_2: f32,
    pub m_def_value_3: f32,
    pub m_def_texture: SerializedTextureProperty,
}

impl crate::Parse for SerializedProperty {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_description = rdr.read_aligned_string()?;
        let m_attributes = SerializedPropertyMAttributesVariant::parse(rdr, ver)?;
        let m_type = rdr.read_i32()?;
        let m_flags = rdr.read_u32()?;
        let m_def_value_0 = rdr.read_f32()?;
        let m_def_value_1 = rdr.read_f32()?;
        let m_def_value_2 = rdr.read_f32()?;
        let m_def_value_3 = rdr.read_f32()?;
        let m_def_texture = SerializedTextureProperty::parse(rdr, ver)?;

        Ok(SerializedProperty {
            m_name,
            m_description,
            m_attributes,
            m_type,
            m_flags,
            m_def_value_0,
            m_def_value_1,
            m_def_value_2,
            m_def_value_3,
            m_def_texture,
        })
    }
}

impl crate::ToValue for SerializedProperty {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Description".to_string(), self.m_description.to_value())),
            Some(("m_Attributes".to_string(), self.m_attributes.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_Flags".to_string(), self.m_flags.to_value())),
            Some(("m_DefValue[0]".to_string(), self.m_def_value_0.to_value())),
            Some(("m_DefValue[1]".to_string(), self.m_def_value_1.to_value())),
            Some(("m_DefValue[2]".to_string(), self.m_def_value_2.to_value())),
            Some(("m_DefValue[3]".to_string(), self.m_def_value_3.to_value())),
            Some(("m_DefTexture".to_string(), self.m_def_texture.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Description" => (self.m_description).inspect_value(path, offset, limit),
                "m_Attributes" => (self.m_attributes).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Flags" => (self.m_flags).inspect_value(path, offset, limit),
                "m_DefValue[0]" => (self.m_def_value_0).inspect_value(path, offset, limit),
                "m_DefValue[1]" => (self.m_def_value_1).inspect_value(path, offset, limit),
                "m_DefValue[2]" => (self.m_def_value_2).inspect_value(path, offset, limit),
                "m_DefValue[3]" => (self.m_def_value_3).inspect_value(path, offset, limit),
                "m_DefTexture" => (self.m_def_texture).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_description).inspect_value(path, offset, limit))
                .map(|v| ("m_Description".to_string(), v)),
            ((self.m_attributes).inspect_value(path, offset, limit))
                .map(|v| ("m_Attributes".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_flags).inspect_value(path, offset, limit)).map(|v| ("m_Flags".to_string(), v)),
            ((self.m_def_value_0).inspect_value(path, offset, limit))
                .map(|v| ("m_DefValue[0]".to_string(), v)),
            ((self.m_def_value_1).inspect_value(path, offset, limit))
                .map(|v| ("m_DefValue[1]".to_string(), v)),
            ((self.m_def_value_2).inspect_value(path, offset, limit))
                .map(|v| ("m_DefValue[2]".to_string(), v)),
            ((self.m_def_value_3).inspect_value(path, offset, limit))
                .map(|v| ("m_DefValue[3]".to_string(), v)),
            ((self.m_def_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_DefTexture".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
