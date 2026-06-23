//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct CubemapArray {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_image_contents_hash: Hash128,
    pub m_width: i32,
    pub m_cubemap_count: i32,
    pub m_format: i32,
    pub m_mip_count: i32,
    pub m_data_size: u32,
    pub m_texture_settings: GlTextureSettings,
    pub m_color_space: i32,
    pub m_is_readable: bool,
    pub image_data: Vec<u8>,
}

impl crate::Parse for CubemapArray {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_image_contents_hash = Hash128::parse(r, ver)?;
        let m_width = r.read_i32()?;
        let m_cubemap_count = r.read_i32()?;
        let m_format = r.read_i32()?;
        let m_mip_count = r.read_i32()?;
        let m_data_size = r.read_u32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_color_space = r.read_i32()?;
        let m_is_readable = r.read_bool()?;
        r.align(4)?;
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(CubemapArray {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_image_contents_hash,
            m_width,
            m_cubemap_count,
            m_format,
            m_mip_count,
            m_data_size,
            m_texture_settings,
            m_color_space,
            m_is_readable,
            image_data,
        })
    }
}
