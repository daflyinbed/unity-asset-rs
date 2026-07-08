//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：MovieTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MovieTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for MovieTextureMDownscaleFallbackVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MovieTexture {
    pub m_name: String,
    pub m_loop: Option<bool>,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_audio_clip: Option<PPtr>,
    pub m_downscale_fallback: Option<MovieTextureMDownscaleFallbackVariant>,
    pub m_movie_data: Option<Vec<u8>>,
    pub m_color_space: Option<i32>,
}

impl crate::Parse for MovieTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_loop = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_audio_clip = None;
        let mut m_downscale_fallback = None;
        let mut m_movie_data = None;
        let mut m_color_space = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(MovieTextureMDownscaleFallbackVariant::parse(r, ver)?);
            m_is_alpha_channel_optional = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(MovieTextureMDownscaleFallbackVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(r.read_i32()?);
            m_downscale_fallback = Some(MovieTextureMDownscaleFallbackVariant::parse(r, ver)?);
            m_loop = Some(r.read_bool()?);
            r.align(4)?;
            m_audio_clip = Some(crate::PPtr::parse(r, ver)?);
            m_movie_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_color_space = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_loop = Some(r.read_bool()?);
            r.align(4)?;
            m_audio_clip = Some(crate::PPtr::parse(r, ver)?);
            m_movie_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_color_space = Some(r.read_i32()?);
        } else {
            m_loop = Some(r.read_bool()?);
            r.align(4)?;
            m_audio_clip = Some(crate::PPtr::parse(r, ver)?);
            m_movie_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        }

        Ok(MovieTexture {
            m_name: m_name.unwrap(),
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
