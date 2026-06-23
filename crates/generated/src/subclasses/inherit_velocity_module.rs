//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct InheritVelocityModule {
    pub enabled: bool,
    pub m_mode: i32,
    pub m_curve: MinMaxCurve,
}

impl crate::Parse for InheritVelocityModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let m_mode = r.read_i32()?;
        let m_curve = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;

        Ok(InheritVelocityModule {
            enabled,
            m_mode,
            m_curve,
        })
    }
}
