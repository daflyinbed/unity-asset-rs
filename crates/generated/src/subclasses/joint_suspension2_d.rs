//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointSuspension2D {
    pub m_damping_ratio: f32,
    pub m_frequency: f32,
    pub m_angle: f32,
}

impl crate::Parse for JointSuspension2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_damping_ratio = rdr.read_f32()?;
        let m_frequency = rdr.read_f32()?;
        let m_angle = rdr.read_f32()?;

        Ok(JointSuspension2D {
            m_damping_ratio,
            m_frequency,
            m_angle,
        })
    }
}

impl crate::ToValue for JointSuspension2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_DampingRatio".to_string(),
                self.m_damping_ratio.to_value(),
            )),
            Some(("m_Frequency".to_string(), self.m_frequency.to_value())),
            Some(("m_Angle".to_string(), self.m_angle.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_DampingRatio" => (self.m_damping_ratio).inspect_value(path, offset, limit),
                "m_Frequency" => (self.m_frequency).inspect_value(path, offset, limit),
                "m_Angle" => (self.m_angle).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_damping_ratio).inspect_value(path, offset, limit))
                .map(|v| ("m_DampingRatio".to_string(), v)),
            ((self.m_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_Frequency".to_string(), v)),
            ((self.m_angle).inspect_value(path, offset, limit)).map(|v| ("m_Angle".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
