//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct SizeModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub y: Option<MinMaxCurve>,
    pub z: Option<MinMaxCurve>,
    pub separate_axes: Option<bool>,
}

impl crate::Parse for SizeModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let curve = MinMaxCurve::parse(rdr, ver)?;
        let y = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(MinMaxCurve::parse(rdr, ver)?)
        } else {
            None
        };
        let z = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(MinMaxCurve::parse(rdr, ver)?)
        } else {
            None
        };
        let separate_axes = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SizeModule {
            enabled,
            curve,
            y,
            z,
            separate_axes,
        })
    }
}
