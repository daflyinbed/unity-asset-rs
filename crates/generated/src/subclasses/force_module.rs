//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct ForceModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub in_world_space: bool,
    pub randomize_per_frame: bool,
}

impl crate::Parse for ForceModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let x = MinMaxCurve::parse(rdr, ver)?;
        let y = MinMaxCurve::parse(rdr, ver)?;
        let z = MinMaxCurve::parse(rdr, ver)?;
        let in_world_space = rdr.read_bool()?;
        let randomize_per_frame = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ForceModule {
            enabled,
            x,
            y,
            z,
            in_world_space,
            randomize_per_frame,
        })
    }
}

impl crate::ToValue for ForceModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("x".to_string(), self.x.to_value())),
            Some(("y".to_string(), self.y.to_value())),
            Some(("z".to_string(), self.z.to_value())),
            Some(("inWorldSpace".to_string(), self.in_world_space.to_value())),
            Some((
                "randomizePerFrame".to_string(),
                self.randomize_per_frame.to_value(),
            )),
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
                "randomizePerFrame" => {
                    (self.randomize_per_frame).inspect_value(path, offset, limit)
                }
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
            ((self.randomize_per_frame).inspect_value(path, offset, limit))
                .map(|v| ("randomizePerFrame".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
