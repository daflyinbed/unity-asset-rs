//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioChorusFilter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_dry_mix: f32,
    pub m_wet_mix1: f32,
    pub m_wet_mix2: f32,
    pub m_wet_mix3: f32,
    pub m_delay: f32,
    pub m_rate: f32,
    pub m_depth: f32,
    pub m_feed_back: f32,
}

impl crate::Parse for AudioChorusFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_dry_mix = r.read_f32()?;
        let m_wet_mix1 = r.read_f32()?;
        let m_wet_mix2 = r.read_f32()?;
        let m_wet_mix3 = r.read_f32()?;
        let m_delay = r.read_f32()?;
        let m_rate = r.read_f32()?;
        let m_depth = r.read_f32()?;
        let m_feed_back = r.read_f32()?;

        Ok(AudioChorusFilter {
            m_object_hide_flags,
            m_extension_ptr,
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
