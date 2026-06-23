//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Child {
    pub m_motion: PPtr,
    pub m_threshold: f32,
    pub m_time_scale: f32,
    pub m_is_anim: bool,
}

impl crate::Parse for Child {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_motion = crate::PPtr::parse(r, ver)?;
        let m_threshold = r.read_f32()?;
        let m_time_scale = r.read_f32()?;
        let m_is_anim = r.read_bool()?;
        r.align(4)?;

        Ok(Child {
            m_motion,
            m_threshold,
            m_time_scale,
            m_is_anim,
        })
    }
}
