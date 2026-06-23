//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;
use super::texture2_d::Texture2D;

#[derive(Debug, Clone)]
pub struct Cubemap {
    pub parent: Texture2D,
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<bool>,
    pub m_complete_image_size: i32,
    pub m_texture_format: i32,
    pub m_mip_map: Option<bool>,
    pub m_mip_count: Option<i32>,
    pub m_mips_stripped: Option<i32>,
    pub m_is_readable: bool,
    pub m_read_allowed: Option<bool>,
    pub m_image_count: i32,
    pub m_texture_dimension: i32,
    pub m_texture_settings: GlTextureSettings,
    pub m_streaming_mipmaps: Option<bool>,
    pub m_ignore_master_texture_limit: Option<bool>,
    pub m_is_pre_processed: Option<bool>,
    pub m_lightmap_format: i32,
    pub m_streaming_mipmaps_priority: Option<i32>,
    pub m_ignore_mipmap_limit: Option<bool>,
    pub m_color_space: Option<i32>,
    pub m_mipmap_limit_group_name: Option<String>,
    pub image_data: Vec<u8>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_source_textures: Option<Vec<PPtr>>,
    pub m_platform_blob: Option<Vec<u8>>,
}

impl crate::Parse for Cubemap {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Texture2D::parse(r, ver)?;
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
        let m_height = r.read_i32()?;
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_complete_image_size = r.read_i32()?;
        let m_texture_format = r.read_i32()?;
        let m_mip_map = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mip_count = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mips_stripped = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_readable = r.read_bool()?;
        let m_read_allowed = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_image_count = r.read_i32()?;
        let m_texture_dimension = r.read_i32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_streaming_mipmaps = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ignore_master_texture_limit = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_is_pre_processed = if *ver >= crate::UnityVersion::new(2019, 4, 9) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_lightmap_format = r.read_i32()?;
        let m_streaming_mipmaps_priority = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ignore_mipmap_limit = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_color_space = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mipmap_limit_group_name = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;
        let m_stream_data = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = StreamingInfo::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_source_textures = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_platform_blob = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Cubemap {
            parent,
            m_name,
            m_width,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height,
            m_downscale_fallback,
            m_complete_image_size,
            m_texture_format,
            m_mip_map,
            m_mip_count,
            m_mips_stripped,
            m_is_readable,
            m_read_allowed,
            m_image_count,
            m_texture_dimension,
            m_texture_settings,
            m_streaming_mipmaps,
            m_ignore_master_texture_limit,
            m_is_pre_processed,
            m_lightmap_format,
            m_streaming_mipmaps_priority,
            m_ignore_mipmap_limit,
            m_color_space,
            m_mipmap_limit_group_name,
            image_data,
            m_stream_data,
            m_source_textures,
            m_platform_blob,
        })
    }
}
