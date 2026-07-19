//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct RotationModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub x: Option<MinMaxCurve>,
    pub y: Option<MinMaxCurve>,
    pub separate_axes: Option<bool>,
}

impl crate::Parse for RotationModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut curve = None;
        let mut x = None;
        let mut y = None;
        let mut separate_axes = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            x = Some(MinMaxCurve::parse(rdr, ver)?);
            y = Some(MinMaxCurve::parse(rdr, ver)?);
        } else {
        }
        curve = Some(MinMaxCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            separate_axes = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(RotationModule {
            enabled: enabled.unwrap(),
            curve: curve.unwrap(),
            x,
            y,
            separate_axes,
        })
    }
}
