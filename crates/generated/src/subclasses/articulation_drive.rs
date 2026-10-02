//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ArticulationDrive {
    pub lower_limit: f32,
    pub upper_limit: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub force_limit: f32,
    pub target: f32,
    pub target_velocity: f32,
    pub drive_type: Option<i32>,
}

impl crate::Parse for ArticulationDrive {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let lower_limit = rdr.read_f32()?;
        let upper_limit = rdr.read_f32()?;
        let stiffness = rdr.read_f32()?;
        let damping = rdr.read_f32()?;
        let force_limit = rdr.read_f32()?;
        let target = rdr.read_f32()?;
        let target_velocity = rdr.read_f32()?;
        let drive_type = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(ArticulationDrive {
            lower_limit,
            upper_limit,
            stiffness,
            damping,
            force_limit,
            target,
            target_velocity,
            drive_type,
        })
    }
}

impl crate::ToValue for ArticulationDrive {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("lowerLimit".to_string(), self.lower_limit.to_value())),
            Some(("upperLimit".to_string(), self.upper_limit.to_value())),
            Some(("stiffness".to_string(), self.stiffness.to_value())),
            Some(("damping".to_string(), self.damping.to_value())),
            Some(("forceLimit".to_string(), self.force_limit.to_value())),
            Some(("target".to_string(), self.target.to_value())),
            Some((
                "targetVelocity".to_string(),
                self.target_velocity.to_value(),
            )),
            crate::opt_field("driveType", &self.drive_type),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "lowerLimit" => (self.lower_limit).inspect_value(path, offset, limit),
                "upperLimit" => (self.upper_limit).inspect_value(path, offset, limit),
                "stiffness" => (self.stiffness).inspect_value(path, offset, limit),
                "damping" => (self.damping).inspect_value(path, offset, limit),
                "forceLimit" => (self.force_limit).inspect_value(path, offset, limit),
                "target" => (self.target).inspect_value(path, offset, limit),
                "targetVelocity" => (self.target_velocity).inspect_value(path, offset, limit),
                "driveType" => (self.drive_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.lower_limit).inspect_value(path, offset, limit))
                .map(|v| ("lowerLimit".to_string(), v)),
            ((self.upper_limit).inspect_value(path, offset, limit))
                .map(|v| ("upperLimit".to_string(), v)),
            ((self.stiffness).inspect_value(path, offset, limit))
                .map(|v| ("stiffness".to_string(), v)),
            ((self.damping).inspect_value(path, offset, limit)).map(|v| ("damping".to_string(), v)),
            ((self.force_limit).inspect_value(path, offset, limit))
                .map(|v| ("forceLimit".to_string(), v)),
            ((self.target).inspect_value(path, offset, limit)).map(|v| ("target".to_string(), v)),
            ((self.target_velocity).inspect_value(path, offset, limit))
                .map(|v| ("targetVelocity".to_string(), v)),
            ((self.drive_type).inspect_value(path, offset, limit))
                .map(|v| ("driveType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
