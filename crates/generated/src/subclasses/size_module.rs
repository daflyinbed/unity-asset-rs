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

impl crate::ToValue for SizeModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("curve".to_string(), self.curve.to_value())),
            crate::opt_field("y", &self.y),
            crate::opt_field("z", &self.z),
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
                "y" => (self.y).inspect_value(path, offset, limit),
                "z" => (self.z).inspect_value(path, offset, limit),
                "separateAxes" => (self.separate_axes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.curve).inspect_value(path, offset, limit)).map(|v| ("curve".to_string(), v)),
            ((self.y).inspect_value(path, offset, limit)).map(|v| ("y".to_string(), v)),
            ((self.z).inspect_value(path, offset, limit)).map(|v| ("z".to_string(), v)),
            ((self.separate_axes).inspect_value(path, offset, limit))
                .map(|v| ("separateAxes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
