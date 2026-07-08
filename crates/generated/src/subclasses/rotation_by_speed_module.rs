//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct RotationBySpeedModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub x: Option<MinMaxCurve>,
    pub range: Vector2f,
    pub y: Option<MinMaxCurve>,
    pub separate_axes: Option<bool>,
}

impl crate::Parse for RotationBySpeedModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut curve = None;
        let mut x = None;
        let mut range = None;
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
            range = Some(Vector2f::parse(rdr, ver)?);

        Ok(RotationBySpeedModule {
            enabled: enabled.unwrap(),
            curve: curve.unwrap(),
            x,
            range: range.unwrap(),
            y,
            separate_axes,
        })
    }
}
