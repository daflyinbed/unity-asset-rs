//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SoftJointLimit {
    pub limit: f32,
    pub bounciness: f32,
    pub spring: Option<f32>,
    pub contact_distance: Option<f32>,
    pub damper: Option<f32>,
}

impl crate::Parse for SoftJointLimit {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut limit = None;
        let mut bounciness = None;
        let mut spring = None;
        let mut contact_distance = None;
        let mut damper = None;
        limit = Some(rdr.read_f32()?);
        bounciness = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            contact_distance = Some(rdr.read_f32()?);
        } else {
            spring = Some(rdr.read_f32()?);
            damper = Some(rdr.read_f32()?);
        }

        Ok(SoftJointLimit {
            limit: limit.unwrap(),
            bounciness: bounciness.unwrap(),
            spring,
            contact_distance,
            damper,
        })
    }
}

impl crate::ToValue for SoftJointLimit {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("limit".to_string(), self.limit.to_value())),
            Some(("bounciness".to_string(), self.bounciness.to_value())),
            crate::opt_field("spring", &self.spring),
            crate::opt_field("contactDistance", &self.contact_distance),
            crate::opt_field("damper", &self.damper),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "limit" => (self.limit).inspect_value(path, offset, limit),
                "bounciness" => (self.bounciness).inspect_value(path, offset, limit),
                "spring" => (self.spring).inspect_value(path, offset, limit),
                "contactDistance" => (self.contact_distance).inspect_value(path, offset, limit),
                "damper" => (self.damper).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.limit).inspect_value(path, offset, limit)).map(|v| ("limit".to_string(), v)),
            ((self.bounciness).inspect_value(path, offset, limit))
                .map(|v| ("bounciness".to_string(), v)),
            ((self.spring).inspect_value(path, offset, limit)).map(|v| ("spring".to_string(), v)),
            ((self.contact_distance).inspect_value(path, offset, limit))
                .map(|v| ("contactDistance".to_string(), v)),
            ((self.damper).inspect_value(path, offset, limit)).map(|v| ("damper".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
