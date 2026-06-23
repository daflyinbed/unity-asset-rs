//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct HumanGoal {
    pub m_x: xform,
    pub m_weight_t: f32,
    pub m_weight_r: f32,
}

impl crate::Parse for HumanGoal {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(r, ver)?;
        let m_weight_t = r.read_f32()?;
        let m_weight_r = r.read_f32()?;

        Ok(HumanGoal {
            m_x,
            m_weight_t,
            m_weight_r,
        })
    }
}
