//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioDistortionFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_distortion_level: f32,
}

impl crate::Parse for AudioDistortionFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_distortion_level = rdr.read_f32()?;

        Ok(AudioDistortionFilter {
            m_game_object,
            m_enabled,
            m_distortion_level,
        })
    }
}
