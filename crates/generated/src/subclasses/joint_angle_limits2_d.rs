//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointAngleLimits2D {
    pub m_lower_angle: f32,
    pub m_upper_angle: f32,
}

impl crate::Parse for JointAngleLimits2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lower_angle = rdr.read_f32()?;
        let m_upper_angle = rdr.read_f32()?;

        Ok(JointAngleLimits2D {
            m_lower_angle,
            m_upper_angle,
        })
    }
}

impl crate::ToValue for JointAngleLimits2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_LowerAngle".to_string(), self.m_lower_angle.to_value())),
            Some(("m_UpperAngle".to_string(), self.m_upper_angle.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_LowerAngle" => (self.m_lower_angle).inspect_value(path, offset, limit),
                "m_UpperAngle" => (self.m_upper_angle).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_lower_angle).inspect_value(path, offset, limit))
                .map(|v| ("m_LowerAngle".to_string(), v)),
            ((self.m_upper_angle).inspect_value(path, offset, limit))
                .map(|v| ("m_UpperAngle".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
