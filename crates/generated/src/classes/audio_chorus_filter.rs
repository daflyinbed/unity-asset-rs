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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_dry_mix = None;
        let mut m_wet_mix1 = None;
        let mut m_wet_mix2 = None;
        let mut m_wet_mix3 = None;
        let mut m_delay = None;
        let mut m_rate = None;
        let mut m_depth = None;
        let mut m_feed_back = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_dry_mix = Some(rdr.read_f32()?);
        m_wet_mix1 = Some(rdr.read_f32()?);
        m_wet_mix2 = Some(rdr.read_f32()?);
        m_wet_mix3 = Some(rdr.read_f32()?);
        m_delay = Some(rdr.read_f32()?);
        m_rate = Some(rdr.read_f32()?);
        m_depth = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
        } else {
            m_feed_back = Some(rdr.read_f32()?);
        }

        Ok(AudioChorusFilter {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_dry_mix: m_dry_mix.unwrap(),
            m_wet_mix1: m_wet_mix1.unwrap(),
            m_wet_mix2: m_wet_mix2.unwrap(),
            m_wet_mix3: m_wet_mix3.unwrap(),
            m_delay: m_delay.unwrap(),
            m_rate: m_rate.unwrap(),
            m_depth: m_depth.unwrap(),
            m_feed_back,
        })
    }
}
