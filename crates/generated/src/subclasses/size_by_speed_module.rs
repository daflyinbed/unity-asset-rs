//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SizeBySpeedModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub range: Vector2f,
}

impl crate::Parse for SizeBySpeedModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let curve = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let range = Vector2f::parse(r, ver)?;

        Ok(SizeBySpeedModule {
            enabled,
            curve,
            range,
        })
    }
}
