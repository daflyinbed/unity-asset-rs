//! 自动生成。请勿手动修改。

use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::streaming_info::StreamingInfo;
use super::texture2_d::Texture2D;
use super::texture::Texture;

#[derive(Debug, Clone)]
pub enum Texture3DParent {
    Texture2D(Texture2D),
    Texture(Texture),
}

#[derive(Debug, Clone)]
pub struct Texture3D {
    pub parent: Texture3DParent,
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_color_space: Option<i32>,
    pub m_downscale_fallback: Option<bool>,
    pub m_complete_image_size: Option<i32>,
    pub m_depth: Option<i32>,
    pub m_format: Option<u32>,
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
        let parent = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Texture3DParent::Texture2D(Texture2D::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Texture3DParent::Texture(Texture::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
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
        let m_color_space = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_complete_image_size = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_depth = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_format = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_texture_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mip_map = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mip_count = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_readable = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_data_size = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_read_allowed = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_image_count = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;
        let m_texture_dimension = if *ver >= crate::UnityVersion::new(3, 4, 0) {
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
        let m_usage_mode = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmap_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(Texture3D {
            parent,
            m_name,
            m_width,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height,
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
            m_texture_settings,
            m_image_count,
            image_data,
            m_texture_dimension,
            m_stream_data,
            m_usage_mode,
            m_lightmap_format,
        })
    }
}
