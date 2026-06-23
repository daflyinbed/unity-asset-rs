//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_texture_property::SerializedTextureProperty;

#[derive(Debug, Clone)]
pub struct SerializedProperty {
    pub m_name: String,
    pub m_description: String,
    pub m_attributes: Vec<String>,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_description = r.read_aligned_string()?;
        r.align(4)?;
        let m_attributes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_flags = r.read_u32()?;
        let m_def_value_0 = r.read_f32()?;
        let m_def_value_1 = r.read_f32()?;
        let m_def_value_2 = r.read_f32()?;
        let m_def_value_3 = r.read_f32()?;
        let m_def_texture = SerializedTextureProperty::parse(r, ver)?;
        r.align(4)?;

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
