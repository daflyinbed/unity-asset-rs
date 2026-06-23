//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;

#[derive(Debug, Clone)]
pub struct RenderTexture {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_depth_format: i32,
    pub m_color_format: i32,
    pub m_is_power_of_two: bool,
    pub m_is_cubemap: bool,
    pub m_mip_map: bool,
    pub m_texture_settings: GlTextureSettings,
}

impl crate::Parse for RenderTexture {
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
        let m_depth_format = r.read_i32()?;
        let m_color_format = r.read_i32()?;
        let m_is_power_of_two = r.read_bool()?;
        let m_is_cubemap = r.read_bool()?;
        let m_mip_map = r.read_bool()?;
        r.align(4)?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;

        Ok(RenderTexture {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_width,
            m_height,
            m_depth_format,
            m_color_format,
            m_is_power_of_two,
            m_is_cubemap,
            m_mip_map,
            m_texture_settings,
        })
    }
}
