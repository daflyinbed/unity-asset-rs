//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointLimits {
    pub min: f32,
    pub max: f32,
    pub min_bounce: Option<f32>,
    pub bounciness: Option<f32>,
    pub max_bounce: Option<f32>,
    pub bounce_min_velocity: Option<f32>,
    pub contact_distance: Option<f32>,
}

impl crate::Parse for JointLimits {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut min = None;
        let mut max = None;
        let mut min_bounce = None;
        let mut bounciness = None;
        let mut max_bounce = None;
        let mut bounce_min_velocity = None;
        let mut contact_distance = None;
        min = Some(rdr.read_f32()?);
        max = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            bounciness = Some(rdr.read_f32()?);
            bounce_min_velocity = Some(rdr.read_f32()?);
            contact_distance = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            min_bounce = Some(rdr.read_f32()?);
            max_bounce = Some(rdr.read_f32()?);
            contact_distance = Some(rdr.read_f32()?);
        } else {
            min_bounce = Some(rdr.read_f32()?);
            max_bounce = Some(rdr.read_f32()?);
        }

        Ok(JointLimits {
            min: min.unwrap(),
            max: max.unwrap(),
            min_bounce,
            bounciness,
            max_bounce,
            bounce_min_velocity,
            contact_distance,
        })
    }
}

impl crate::ToValue for JointLimits {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("min".to_string(), self.min.to_value())),
            Some(("max".to_string(), self.max.to_value())),
            crate::opt_field("minBounce", &self.min_bounce),
            crate::opt_field("bounciness", &self.bounciness),
            crate::opt_field("maxBounce", &self.max_bounce),
            crate::opt_field("bounceMinVelocity", &self.bounce_min_velocity),
            crate::opt_field("contactDistance", &self.contact_distance),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "min" => (self.min).inspect_value(path, offset, limit),
                "max" => (self.max).inspect_value(path, offset, limit),
                "minBounce" => (self.min_bounce).inspect_value(path, offset, limit),
                "bounciness" => (self.bounciness).inspect_value(path, offset, limit),
                "maxBounce" => (self.max_bounce).inspect_value(path, offset, limit),
                "bounceMinVelocity" => {
                    (self.bounce_min_velocity).inspect_value(path, offset, limit)
                }
                "contactDistance" => (self.contact_distance).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.min).inspect_value(path, offset, limit)).map(|v| ("min".to_string(), v)),
            ((self.max).inspect_value(path, offset, limit)).map(|v| ("max".to_string(), v)),
            ((self.min_bounce).inspect_value(path, offset, limit))
                .map(|v| ("minBounce".to_string(), v)),
            ((self.bounciness).inspect_value(path, offset, limit))
                .map(|v| ("bounciness".to_string(), v)),
            ((self.max_bounce).inspect_value(path, offset, limit))
                .map(|v| ("maxBounce".to_string(), v)),
            ((self.bounce_min_velocity).inspect_value(path, offset, limit))
                .map(|v| ("bounceMinVelocity".to_string(), v)),
            ((self.contact_distance).inspect_value(path, offset, limit))
                .map(|v| ("contactDistance".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
