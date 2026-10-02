//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointMotor {
    pub target_velocity: f32,
    pub force: f32,
    pub free_spin: i32,
}

impl crate::Parse for JointMotor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_velocity = rdr.read_f32()?;
        let force = rdr.read_f32()?;
        let free_spin = rdr.read_i32()?;

        Ok(JointMotor {
            target_velocity,
            force,
            free_spin,
        })
    }
}

impl crate::ToValue for JointMotor {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "targetVelocity".to_string(),
                self.target_velocity.to_value(),
            )),
            Some(("force".to_string(), self.force.to_value())),
            Some(("freeSpin".to_string(), self.free_spin.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "targetVelocity" => (self.target_velocity).inspect_value(path, offset, limit),
                "force" => (self.force).inspect_value(path, offset, limit),
                "freeSpin" => (self.free_spin).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.target_velocity).inspect_value(path, offset, limit))
                .map(|v| ("targetVelocity".to_string(), v)),
            ((self.force).inspect_value(path, offset, limit)).map(|v| ("force".to_string(), v)),
            ((self.free_spin).inspect_value(path, offset, limit))
                .map(|v| ("freeSpin".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
