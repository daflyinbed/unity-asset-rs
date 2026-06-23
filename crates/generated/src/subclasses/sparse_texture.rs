//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;

#[derive(Debug, Clone)]
pub struct SparseTexture {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_mip_count: i32,
    pub m_format: u32,
    pub m_color_space: i32,
    pub m_texture_settings: GlTextureSettings,
}

impl crate::Parse for SparseTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_mip_count = r.read_i32()?;
        let m_format = r.read_u32()?;
        let m_color_space = r.read_i32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;

        Ok(SparseTexture {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_width,
            m_height,
            m_mip_count,
            m_format,
            m_color_space,
            m_texture_settings,
        })
    }
}
