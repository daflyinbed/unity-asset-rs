//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct LifetimeByEmitterSpeedModule {
    pub enabled: bool,
    pub m_curve: MinMaxCurve,
    pub m_range: Vector2f,
}

impl crate::Parse for LifetimeByEmitterSpeedModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let m_curve = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let m_range = Vector2f::parse(r, ver)?;

        Ok(LifetimeByEmitterSpeedModule {
            enabled,
            m_curve,
            m_range,
        })
    }
}
