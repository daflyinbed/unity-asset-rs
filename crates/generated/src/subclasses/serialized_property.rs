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
