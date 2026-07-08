//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct VelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub in_world_space: bool,
    pub speed_modifier: Option<MinMaxCurve>,
    pub orbital_x: Option<MinMaxCurve>,
    pub orbital_y: Option<MinMaxCurve>,
    pub orbital_z: Option<MinMaxCurve>,
    pub orbital_offset_x: Option<MinMaxCurve>,
    pub orbital_offset_y: Option<MinMaxCurve>,
    pub orbital_offset_z: Option<MinMaxCurve>,
    pub radial: Option<MinMaxCurve>,
}

impl crate::Parse for VelocityModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut in_world_space = None;
        let mut speed_modifier = None;
        let mut orbital_x = None;
        let mut orbital_y = None;
        let mut orbital_z = None;
        let mut orbital_offset_x = None;
        let mut orbital_offset_y = None;
        let mut orbital_offset_z = None;
        let mut radial = None;
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            x = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            y = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            z = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            orbital_x = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            orbital_y = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            orbital_z = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            orbital_offset_x = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            orbital_offset_y = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            orbital_offset_z = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            radial = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            speed_modifier = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            speed_modifier = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
        }
            in_world_space = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(VelocityModule {
            enabled: enabled.unwrap(),
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
            in_world_space: in_world_space.unwrap(),
            speed_modifier,
            orbital_x,
            orbital_y,
            orbital_z,
            orbital_offset_x,
            orbital_offset_y,
            orbital_offset_z,
            radial,
        })
    }
}
