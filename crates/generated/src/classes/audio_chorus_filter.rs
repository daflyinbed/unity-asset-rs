//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioChorusFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_dry_mix: f32,
    pub m_wet_mix1: f32,
    pub m_wet_mix2: f32,
    pub m_wet_mix3: f32,
    pub m_delay: f32,
    pub m_rate: f32,
    pub m_depth: f32,
    pub m_feed_back: Option<f32>,
}

impl crate::Parse for AudioChorusFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_dry_mix = rdr.read_f32()?;
        let m_wet_mix1 = rdr.read_f32()?;
        let m_wet_mix2 = rdr.read_f32()?;
        let m_wet_mix3 = rdr.read_f32()?;
        let m_delay = rdr.read_f32()?;
        let m_rate = rdr.read_f32()?;
        let m_depth = rdr.read_f32()?;
        let m_feed_back = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(AudioChorusFilter {
            m_game_object,
            m_enabled,
            m_dry_mix,
            m_wet_mix1,
            m_wet_mix2,
            m_wet_mix3,
            m_delay,
            m_rate,
            m_depth,
            m_feed_back,
        })
    }
}
