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
        let m_animated_sprites = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_animation_speed = rdr.read_f32()?;
        let m_animation_time_offset = rdr.read_f32()?;
        let m_is_looping = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_flags = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_u32()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TileAnimationData {
            m_animated_sprites,
            m_animation_speed,
            m_animation_time_offset,
            m_is_looping,
            m_flags,
        })
    }
}
