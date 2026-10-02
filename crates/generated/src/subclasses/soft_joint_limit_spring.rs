//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SoftJointLimitSpring {
    pub spring: f32,
    pub damper: f32,
}

impl crate::Parse for SoftJointLimitSpring {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let spring = rdr.read_f32()?;
        let damper = rdr.read_f32()?;

        Ok(SoftJointLimitSpring { spring, damper })
    }
}

impl crate::ToValue for SoftJointLimitSpring {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("spring".to_string(), self.spring.to_value())),
            Some(("damper".to_string(), self.damper.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "spring" => (self.spring).inspect_value(path, offset, limit),
                "damper" => (self.damper).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.spring).inspect_value(path, offset, limit)).map(|v| ("spring".to_string(), v)),
            ((self.damper).inspect_value(path, offset, limit)).map(|v| ("damper".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
