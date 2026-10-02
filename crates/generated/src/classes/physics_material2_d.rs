//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PhysicsMaterial2D {
    pub m_name: String,
    pub friction: f32,
    pub bounciness: f32,
    pub m_friction_combine: Option<i32>,
    pub m_bounce_combine: Option<i32>,
}

impl crate::Parse for PhysicsMaterial2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let friction = rdr.read_f32()?;
        let bounciness = rdr.read_f32()?;
        let m_friction_combine = if *ver >= crate::UnityVersion::new(6000, 0, 14) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_bounce_combine = if *ver >= crate::UnityVersion::new(6000, 0, 14) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(PhysicsMaterial2D {
            m_name,
            friction,
            bounciness,
            m_friction_combine,
            m_bounce_combine,
        })
    }
}

impl crate::ToValue for PhysicsMaterial2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("friction".to_string(), self.friction.to_value())),
            Some(("bounciness".to_string(), self.bounciness.to_value())),
            crate::opt_field("m_FrictionCombine", &self.m_friction_combine),
            crate::opt_field("m_BounceCombine", &self.m_bounce_combine),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "friction" => (self.friction).inspect_value(path, offset, limit),
                "bounciness" => (self.bounciness).inspect_value(path, offset, limit),
                "m_FrictionCombine" => (self.m_friction_combine).inspect_value(path, offset, limit),
                "m_BounceCombine" => (self.m_bounce_combine).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.friction).inspect_value(path, offset, limit))
                .map(|v| ("friction".to_string(), v)),
            ((self.bounciness).inspect_value(path, offset, limit))
                .map(|v| ("bounciness".to_string(), v)),
            ((self.m_friction_combine).inspect_value(path, offset, limit))
                .map(|v| ("m_FrictionCombine".to_string(), v)),
            ((self.m_bounce_combine).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceCombine".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
