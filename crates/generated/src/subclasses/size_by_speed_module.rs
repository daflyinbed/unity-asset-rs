//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct SizeBySpeedModule {
    pub enabled: bool,
    pub curve: MinMaxCurve,
    pub range: Vector2f,
    pub y: Option<MinMaxCurve>,
    pub z: Option<MinMaxCurve>,
    pub separate_axes: Option<bool>,
}

impl crate::Parse for SizeBySpeedModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut curve = None;
        let mut range = None;
        let mut y = None;
        let mut z = None;
        let mut separate_axes = None;
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            curve = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            y = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            z = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
        }
            range = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            separate_axes = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(SizeBySpeedModule {
            enabled: enabled.unwrap(),
            curve: curve.unwrap(),
            range: range.unwrap(),
            y,
            z,
            separate_axes,
        })
    }
}
