//! 自动生成。请勿手动修改。

use super::base_video_texture::BaseVideoTexture;
use super::texture::Texture;

#[derive(Debug, Clone)]
pub enum WebCamTextureParent {
    Texture(Texture),
    BaseVideoTexture(BaseVideoTexture),
}

#[derive(Debug, Clone)]
pub struct WebCamTexture {
    pub parent: WebCamTextureParent,
    pub m_name: String,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_downscale_fallback: Option<bool>,
}

impl crate::Parse for WebCamTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            WebCamTextureParent::Texture(Texture::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            WebCamTextureParent::BaseVideoTexture(BaseVideoTexture::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
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
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(WebCamTexture {
            parent,
            m_name,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_downscale_fallback,
        })
    }
}
