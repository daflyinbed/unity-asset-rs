//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointDrive {
    pub mode: Option<i32>,
    pub position_spring: f32,
    pub position_damper: f32,
    pub maximum_force: f32,
    pub use_acceleration: Option<i32>,
}

impl crate::Parse for JointDrive {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut mode = None;
        let mut position_spring = None;
        let mut position_damper = None;
        let mut maximum_force = None;
        let mut use_acceleration = None;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            mode = Some(rdr.read_i32()?);
        }
        position_spring = Some(rdr.read_f32()?);
        position_damper = Some(rdr.read_f32()?);
        maximum_force = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            use_acceleration = Some(rdr.read_i32()?);
        } else {
        }

        Ok(JointDrive {
            mode,
            position_spring: position_spring.unwrap(),
            position_damper: position_damper.unwrap(),
            maximum_force: maximum_force.unwrap(),
            use_acceleration,
        })
    }
}

impl crate::ToValue for JointDrive {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("mode", &self.mode),
            Some((
                "positionSpring".to_string(),
                self.position_spring.to_value(),
            )),
            Some((
                "positionDamper".to_string(),
                self.position_damper.to_value(),
            )),
            Some(("maximumForce".to_string(), self.maximum_force.to_value())),
            crate::opt_field("useAcceleration", &self.use_acceleration),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "mode" => (self.mode).inspect_value(path, offset, limit),
                "positionSpring" => (self.position_spring).inspect_value(path, offset, limit),
                "positionDamper" => (self.position_damper).inspect_value(path, offset, limit),
                "maximumForce" => (self.maximum_force).inspect_value(path, offset, limit),
                "useAcceleration" => (self.use_acceleration).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.mode).inspect_value(path, offset, limit)).map(|v| ("mode".to_string(), v)),
            ((self.position_spring).inspect_value(path, offset, limit))
                .map(|v| ("positionSpring".to_string(), v)),
            ((self.position_damper).inspect_value(path, offset, limit))
                .map(|v| ("positionDamper".to_string(), v)),
            ((self.maximum_force).inspect_value(path, offset, limit))
                .map(|v| ("maximumForce".to_string(), v)),
            ((self.use_acceleration).inspect_value(path, offset, limit))
                .map(|v| ("useAcceleration".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
