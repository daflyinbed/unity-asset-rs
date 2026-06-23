//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;

#[derive(Debug, Clone)]
pub struct Texture2D {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_complete_image_size: i32,
    pub m_texture_format: i32,
    pub m_mip_map: bool,
    pub m_is_readable: bool,
    pub m_read_allowed: bool,
    pub m_image_count: i32,
    pub m_texture_dimension: i32,
    pub m_texture_settings: GlTextureSettings,
    pub m_lightmap_format: i32,
    pub image_data: Vec<u8>,
}

impl crate::Parse for Texture2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_complete_image_size = r.read_i32()?;
        let m_texture_format = r.read_i32()?;
        let m_mip_map = r.read_bool()?;
        let m_is_readable = r.read_bool()?;
        let m_read_allowed = r.read_bool()?;
        r.align(4)?;
        let m_image_count = r.read_i32()?;
        let m_texture_dimension = r.read_i32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_lightmap_format = r.read_i32()?;
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(Texture2D {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_width,
            m_height,
            m_complete_image_size,
            m_texture_format,
            m_mip_map,
            m_is_readable,
            m_read_allowed,
            m_image_count,
            m_texture_dimension,
            m_texture_settings,
            m_lightmap_format,
            image_data,
        })
    }
}
