//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct SizeModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
}

impl crate::Parse for SizeModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let curve = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;

        Ok(SizeModule {
            enabled,
            curve,
        })
    }
}
