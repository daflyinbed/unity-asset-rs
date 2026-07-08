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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_mode = rdr.read_i32()?;
        let m_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(InheritVelocityModule {
            enabled,
            m_mode,
            m_curve,
        })
    }
}
