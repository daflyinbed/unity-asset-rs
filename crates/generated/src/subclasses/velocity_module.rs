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
        y = Some(MinMaxCurve::parse(rdr, ver)?);
        z = Some(MinMaxCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            orbital_x = Some(MinMaxCurve::parse(rdr, ver)?);
            orbital_y = Some(MinMaxCurve::parse(rdr, ver)?);
            orbital_z = Some(MinMaxCurve::parse(rdr, ver)?);
            orbital_offset_x = Some(MinMaxCurve::parse(rdr, ver)?);
            orbital_offset_y = Some(MinMaxCurve::parse(rdr, ver)?);
            orbital_offset_z = Some(MinMaxCurve::parse(rdr, ver)?);
            radial = Some(MinMaxCurve::parse(rdr, ver)?);
            speed_modifier = Some(MinMaxCurve::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            speed_modifier = Some(MinMaxCurve::parse(rdr, ver)?);
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

impl crate::ToValue for VelocityModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("x".to_string(), self.x.to_value())),
            Some(("y".to_string(), self.y.to_value())),
            Some(("z".to_string(), self.z.to_value())),
            Some(("inWorldSpace".to_string(), self.in_world_space.to_value())),
            crate::opt_field("speedModifier", &self.speed_modifier),
            crate::opt_field("orbitalX", &self.orbital_x),
            crate::opt_field("orbitalY", &self.orbital_y),
            crate::opt_field("orbitalZ", &self.orbital_z),
            crate::opt_field("orbitalOffsetX", &self.orbital_offset_x),
            crate::opt_field("orbitalOffsetY", &self.orbital_offset_y),
            crate::opt_field("orbitalOffsetZ", &self.orbital_offset_z),
            crate::opt_field("radial", &self.radial),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "x" => (self.x).inspect_value(path, offset, limit),
                "y" => (self.y).inspect_value(path, offset, limit),
                "z" => (self.z).inspect_value(path, offset, limit),
                "inWorldSpace" => (self.in_world_space).inspect_value(path, offset, limit),
                "speedModifier" => (self.speed_modifier).inspect_value(path, offset, limit),
                "orbitalX" => (self.orbital_x).inspect_value(path, offset, limit),
                "orbitalY" => (self.orbital_y).inspect_value(path, offset, limit),
                "orbitalZ" => (self.orbital_z).inspect_value(path, offset, limit),
                "orbitalOffsetX" => (self.orbital_offset_x).inspect_value(path, offset, limit),
                "orbitalOffsetY" => (self.orbital_offset_y).inspect_value(path, offset, limit),
                "orbitalOffsetZ" => (self.orbital_offset_z).inspect_value(path, offset, limit),
                "radial" => (self.radial).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.x).inspect_value(path, offset, limit)).map(|v| ("x".to_string(), v)),
            ((self.y).inspect_value(path, offset, limit)).map(|v| ("y".to_string(), v)),
            ((self.z).inspect_value(path, offset, limit)).map(|v| ("z".to_string(), v)),
            ((self.in_world_space).inspect_value(path, offset, limit))
                .map(|v| ("inWorldSpace".to_string(), v)),
            ((self.speed_modifier).inspect_value(path, offset, limit))
                .map(|v| ("speedModifier".to_string(), v)),
            ((self.orbital_x).inspect_value(path, offset, limit))
                .map(|v| ("orbitalX".to_string(), v)),
            ((self.orbital_y).inspect_value(path, offset, limit))
                .map(|v| ("orbitalY".to_string(), v)),
            ((self.orbital_z).inspect_value(path, offset, limit))
                .map(|v| ("orbitalZ".to_string(), v)),
            ((self.orbital_offset_x).inspect_value(path, offset, limit))
                .map(|v| ("orbitalOffsetX".to_string(), v)),
            ((self.orbital_offset_y).inspect_value(path, offset, limit))
                .map(|v| ("orbitalOffsetY".to_string(), v)),
            ((self.orbital_offset_z).inspect_value(path, offset, limit))
                .map(|v| ("orbitalOffsetZ".to_string(), v)),
            ((self.radial).inspect_value(path, offset, limit)).map(|v| ("radial".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
