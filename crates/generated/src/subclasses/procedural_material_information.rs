//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ProceduralMaterialInformation {
    pub m_offset: Vector2f,
    pub m_scale: Vector2f,
    pub m_generated_at_loading: Option<i32>,
    pub m_generate_all_outputs: Option<i32>,
    pub m_generate_mipmaps: Option<bool>,
    pub m_animation_update_rate: Option<i32>,
}

impl crate::Parse for ProceduralMaterialInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_offset = None;
        let mut m_scale = None;
        let mut m_generated_at_loading = None;
        let mut m_generate_all_outputs = None;
        let mut m_generate_mipmaps = None;
        let mut m_animation_update_rate = None;
        m_offset = Some(Vector2f::parse(rdr, ver)?);
        m_scale = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_generate_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_generate_all_outputs = Some(rdr.read_i32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_generate_all_outputs = Some(rdr.read_i32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_generated_at_loading = Some(rdr.read_i32()?);
            m_generate_all_outputs = Some(rdr.read_i32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else {
            m_generated_at_loading = Some(rdr.read_i32()?);
        }

        Ok(ProceduralMaterialInformation {
            m_offset: m_offset.unwrap(),
            m_scale: m_scale.unwrap(),
            m_generated_at_loading,
            m_generate_all_outputs,
            m_generate_mipmaps,
            m_animation_update_rate,
        })
    }
}
