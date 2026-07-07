//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;

#[derive(Debug, Clone)]
pub struct Texture3D {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_color_space: Option<i32>,
    pub m_downscale_fallback: Option<bool>,
    pub m_complete_image_size: Option<i32>,
    pub m_depth: Option<i32>,
    pub m_format: Option<i32>,
    pub m_texture_format: Option<i32>,
    pub m_mip_map: Option<bool>,
    pub m_mip_count: Option<i32>,
    pub m_is_readable: Option<bool>,
    pub m_data_size: Option<u32>,
    pub m_read_allowed: Option<bool>,
    pub m_texture_settings: GlTextureSettings,
    pub m_image_count: Option<i32>,
    pub image_data: Vec<u8>,
    pub m_texture_dimension: Option<i32>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_usage_mode: Option<i32>,
    pub m_lightmap_format: Option<i32>,
}

impl crate::Parse for Texture3D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_height = None;
        let mut m_color_space = None;
        let mut m_downscale_fallback = None;
        let mut m_complete_image_size = None;
        let mut m_depth = None;
        let mut m_format = None;
        let mut m_texture_format = None;
        let mut m_mip_map = None;
        let mut m_mip_count = None;
        let mut m_is_readable = None;
        let mut m_data_size = None;
        let mut m_read_allowed = None;
        let mut m_texture_settings = None;
        let mut m_image_count = None;
        let mut image_data = None;
        let mut m_texture_dimension = None;
        let mut m_stream_data = None;
        let mut m_usage_mode = None;
        let mut m_lightmap_format = None;
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
            m_height = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_depth = Some(r.read_i32()?);
            m_mip_count = Some(r.read_i32()?);
            r.align(4)?;
            m_data_size = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_depth = Some(r.read_i32()?);
            m_format = Some(r.read_i32()?);
            m_mip_count = Some(r.read_i32()?);
            r.align(4)?;
            m_data_size = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_depth = Some(r.read_i32()?);
            m_format = Some(r.read_i32()?);
            m_mip_map = Some(r.read_bool()?);
            r.align(4)?;
            m_data_size = Some(r.read_u32()?);
        } else {
            m_complete_image_size = Some(r.read_i32()?);
            m_texture_format = Some(r.read_i32()?);
            m_mip_map = Some(r.read_bool()?);
            r.align(4)?;
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_read_allowed = Some(r.read_bool()?);
            r.align(4)?;
            m_image_count = Some(r.read_i32()?);
            m_texture_dimension = Some(r.read_i32()?);
        }
            m_texture_settings = Some(GlTextureSettings::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_usage_mode = Some(r.read_i32()?);
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_lightmap_format = Some(r.read_i32()?);
            m_color_space = Some(r.read_i32()?);
        } else {
            m_lightmap_format = Some(r.read_i32()?);
        }
            image_data = Some(( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_stream_data = Some(StreamingInfo::parse(r, ver)?);
            r.align(4)?;
        } else {
        }

        Ok(Texture3D {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_color_space,
            m_downscale_fallback,
            m_complete_image_size,
            m_depth,
            m_format,
            m_texture_format,
            m_mip_map,
            m_mip_count,
            m_is_readable,
            m_data_size,
            m_read_allowed,
            m_texture_settings: m_texture_settings.unwrap(),
            m_image_count,
            image_data: image_data.unwrap(),
            m_texture_dimension,
            m_stream_data,
            m_usage_mode,
            m_lightmap_format,
        })
    }
}
