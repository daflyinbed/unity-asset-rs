//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;
use super::texture::Texture;

#[derive(Debug, Clone)]
pub struct CubemapArray {
    pub parent: Texture,
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_cubemap_count: i32,
    pub m_color_space: i32,
    pub m_downscale_fallback: Option<bool>,
    pub m_format: i32,
    pub m_mip_count: i32,
    pub m_data_size: u32,
    pub m_texture_settings: GlTextureSettings,
    pub m_is_readable: bool,
    pub image_data: Vec<u8>,
    pub m_usage_mode: Option<i32>,
    pub m_stream_data: Option<StreamingInfo>,
}

impl crate::Parse for CubemapArray {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Texture::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
        let m_forced_fallback_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_alpha_channel_optional = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cubemap_count = r.read_i32()?;
        let m_color_space = r.read_i32()?;
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_format = r.read_i32()?;
        let m_mip_count = r.read_i32()?;
        let m_data_size = r.read_u32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_is_readable = r.read_bool()?;
        r.align(4)?;
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;
        let m_usage_mode = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_stream_data = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = StreamingInfo::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(CubemapArray {
            parent,
            m_name,
            m_width,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_cubemap_count,
            m_color_space,
            m_downscale_fallback,
            m_format,
            m_mip_count,
            m_data_size,
            m_texture_settings,
            m_is_readable,
            image_data,
            m_usage_mode,
            m_stream_data,
        })
    }
}
