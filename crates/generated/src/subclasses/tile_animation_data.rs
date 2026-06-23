//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TileAnimationData {
    pub m_animated_sprites: Vec<PPtr>,
    pub m_animation_speed: f32,
    pub m_animation_time_offset: f32,
    pub m_is_looping: bool,
}

impl crate::Parse for TileAnimationData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_animated_sprites = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_animation_speed = r.read_f32()?;
        let m_animation_time_offset = r.read_f32()?;
        let m_is_looping = r.read_bool()?;
        r.align(4)?;

        Ok(TileAnimationData {
            m_animated_sprites,
            m_animation_speed,
            m_animation_time_offset,
            m_is_looping,
        })
    }
}
