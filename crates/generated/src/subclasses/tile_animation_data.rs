//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TileAnimationData {
    pub m_animated_sprites: Vec<PPtr>,
    pub m_animation_speed: f32,
    pub m_animation_time_offset: f32,
    pub m_is_looping: Option<bool>,
    pub m_flags: Option<u32>,
}

impl crate::Parse for TileAnimationData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_animated_sprites = None;
        let mut m_animation_speed = None;
        let mut m_animation_time_offset = None;
        let mut m_is_looping = None;
        let mut m_flags = None;
            m_animated_sprites = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_animation_speed = Some(rdr.read_f32()?);
            m_animation_time_offset = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_flags = Some(rdr.read_u32()?);
            rdr.align(4)?;
        } else {
            m_is_looping = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(TileAnimationData {
            m_animated_sprites: m_animated_sprites.unwrap(),
            m_animation_speed: m_animation_speed.unwrap(),
            m_animation_time_offset: m_animation_time_offset.unwrap(),
            m_is_looping,
            m_flags,
        })
    }
}
