//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;

#[derive(Debug, Clone)]
pub struct CubemapArray {
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
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_cubemap_count = None;
        let mut m_color_space = None;
        let mut m_downscale_fallback = None;
        let mut m_format = None;
        let mut m_mip_count = None;
        let mut m_data_size = None;
        let mut m_texture_settings = None;
        let mut m_is_readable = None;
        let mut image_data = None;
        let mut m_usage_mode = None;
        let mut m_stream_data = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
            m_color_space = Some(r.read_i32()?);
            m_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(r.read_bool()?);
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
            m_color_space = Some(r.read_i32()?);
            m_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(r.read_bool()?);
            m_color_space = Some(r.read_i32()?);
            m_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(r.read_bool()?);
        } else {
        }
            m_width = Some(r.read_i32()?);
            m_cubemap_count = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_format = Some(r.read_i32()?);
        }
            m_mip_count = Some(r.read_i32()?);
            m_data_size = Some(r.read_u32()?);
            m_texture_settings = Some(GlTextureSettings::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_usage_mode = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_color_space = Some(r.read_i32()?);
        }
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            image_data = Some(( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_stream_data = Some(StreamingInfo::parse(r, ver)?);
            r.align(4)?;
        } else {
        }

        Ok(CubemapArray {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_cubemap_count: m_cubemap_count.unwrap(),
            m_color_space: m_color_space.unwrap(),
            m_downscale_fallback,
            m_format: m_format.unwrap(),
            m_mip_count: m_mip_count.unwrap(),
            m_data_size: m_data_size.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
            m_is_readable: m_is_readable.unwrap(),
            image_data: image_data.unwrap(),
            m_usage_mode,
            m_stream_data,
        })
    }
}
