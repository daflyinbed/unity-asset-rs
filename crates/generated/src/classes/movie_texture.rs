//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::base_video_texture::BaseVideoTexture;
use super::texture::Texture;

#[derive(Debug, Clone)]
pub enum MovieTextureParent {
    Texture(Texture),
    BaseVideoTexture(BaseVideoTexture),
}

#[derive(Debug, Clone)]
pub struct MovieTexture {
    pub parent: MovieTextureParent,
    pub m_name: String,
    pub m_loop: Option<bool>,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_audio_clip: Option<PPtr>,
    pub m_downscale_fallback: Option<bool>,
    pub m_movie_data: Option<Vec<u8>>,
    pub m_color_space: Option<i32>,
}

impl crate::Parse for MovieTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            MovieTextureParent::Texture(Texture::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            MovieTextureParent::BaseVideoTexture(BaseVideoTexture::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            MovieTextureParent::Texture(Texture::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_loop = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
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
        let m_audio_clip = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(r, ver)?)
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
        let m_movie_data = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
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

        Ok(MovieTexture {
            parent,
            m_name,
            m_loop,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_audio_clip,
            m_downscale_fallback,
            m_movie_data,
            m_color_space,
        })
    }
}
