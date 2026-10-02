//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointMotor2D {
    pub m_motor_speed: f32,
    pub m_maximum_motor_force: f32,
}

impl crate::Parse for JointMotor2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_motor_speed = rdr.read_f32()?;
        let m_maximum_motor_force = rdr.read_f32()?;

        Ok(JointMotor2D {
            m_motor_speed,
            m_maximum_motor_force,
        })
    }
}

impl crate::ToValue for JointMotor2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_MotorSpeed".to_string(), self.m_motor_speed.to_value())),
            Some((
                "m_MaximumMotorForce".to_string(),
                self.m_maximum_motor_force.to_value(),
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
                "m_MotorSpeed" => (self.m_motor_speed).inspect_value(path, offset, limit),
                "m_MaximumMotorForce" => {
                    (self.m_maximum_motor_force).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_motor_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_MotorSpeed".to_string(), v)),
            ((self.m_maximum_motor_force).inspect_value(path, offset, limit))
                .map(|v| ("m_MaximumMotorForce".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
