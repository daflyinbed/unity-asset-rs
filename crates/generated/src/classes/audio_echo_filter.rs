//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：AudioEchoFilterMDelayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioEchoFilterMDelayVariant {
    Default(u32),
    V2022_2(f32),
}

impl crate::Parse for AudioEchoFilterMDelayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioEchoFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_delay: AudioEchoFilterMDelayVariant,
    pub m_decay_ratio: f32,
    pub m_wet_mix: f32,
    pub m_dry_mix: f32,
}

impl crate::Parse for AudioEchoFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_delay = AudioEchoFilterMDelayVariant::parse(rdr, ver)?;
        let m_decay_ratio = rdr.read_f32()?;
        let m_wet_mix = rdr.read_f32()?;
        let m_dry_mix = rdr.read_f32()?;

        Ok(AudioEchoFilter {
            m_game_object,
            m_enabled,
            m_delay,
            m_decay_ratio,
            m_wet_mix,
            m_dry_mix,
        })
    }
}
