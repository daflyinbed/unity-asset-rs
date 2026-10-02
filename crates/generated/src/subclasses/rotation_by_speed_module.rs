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

impl crate::ToValue for RotationBySpeedModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("curve".to_string(), self.curve.to_value())),
            crate::opt_field("x", &self.x),
            Some(("range".to_string(), self.range.to_value())),
            crate::opt_field("y", &self.y),
            crate::opt_field("separateAxes", &self.separate_axes),
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
                "curve" => (self.curve).inspect_value(path, offset, limit),
                "x" => (self.x).inspect_value(path, offset, limit),
                "range" => (self.range).inspect_value(path, offset, limit),
                "y" => (self.y).inspect_value(path, offset, limit),
                "separateAxes" => (self.separate_axes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.curve).inspect_value(path, offset, limit)).map(|v| ("curve".to_string(), v)),
            ((self.x).inspect_value(path, offset, limit)).map(|v| ("x".to_string(), v)),
            ((self.range).inspect_value(path, offset, limit)).map(|v| ("range".to_string(), v)),
            ((self.y).inspect_value(path, offset, limit)).map(|v| ("y".to_string(), v)),
            ((self.separate_axes).inspect_value(path, offset, limit))
                .map(|v| ("separateAxes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
