//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct PhysicsMaterial {
    pub m_name: String,
    pub dynamic_friction: Option<f32>,
    pub m_dynamic_friction: Option<f32>,
    pub static_friction: Option<f32>,
    pub m_static_friction: Option<f32>,
    pub bounciness: Option<f32>,
    pub m_bounciness: Option<f32>,
    pub friction_combine: Option<i32>,
    pub m_friction_combine: Option<i32>,
    pub bounce_combine: Option<i32>,
    pub m_bounce_combine: Option<i32>,
    pub friction_direction2: Option<Vector3f>,
    pub dynamic_friction2: Option<f32>,
    pub static_friction2: Option<f32>,
}

impl crate::Parse for PhysicsMaterial {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut dynamic_friction = None;
        let mut m_dynamic_friction = None;
        let mut static_friction = None;
        let mut m_static_friction = None;
        let mut bounciness = None;
        let mut m_bounciness = None;
        let mut friction_combine = None;
        let mut m_friction_combine = None;
        let mut bounce_combine = None;
        let mut m_bounce_combine = None;
        let mut friction_direction2 = None;
        let mut dynamic_friction2 = None;
        let mut static_friction2 = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_dynamic_friction = Some(rdr.read_f32()?);
            m_static_friction = Some(rdr.read_f32()?);
            m_bounciness = Some(rdr.read_f32()?);
            m_friction_combine = Some(rdr.read_i32()?);
            m_bounce_combine = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            dynamic_friction = Some(rdr.read_f32()?);
            static_friction = Some(rdr.read_f32()?);
            bounciness = Some(rdr.read_f32()?);
            friction_combine = Some(rdr.read_i32()?);
            bounce_combine = Some(rdr.read_i32()?);
        } else {
            dynamic_friction = Some(rdr.read_f32()?);
            static_friction = Some(rdr.read_f32()?);
            bounciness = Some(rdr.read_f32()?);
            friction_combine = Some(rdr.read_i32()?);
            bounce_combine = Some(rdr.read_i32()?);
            friction_direction2 = Some(Vector3f::parse(rdr, ver)?);
            dynamic_friction2 = Some(rdr.read_f32()?);
            static_friction2 = Some(rdr.read_f32()?);
        }

        Ok(PhysicsMaterial {
            m_name: m_name.unwrap(),
            dynamic_friction,
            m_dynamic_friction,
            static_friction,
            m_static_friction,
            bounciness,
            m_bounciness,
            friction_combine,
            m_friction_combine,
            bounce_combine,
            m_bounce_combine,
            friction_direction2,
            dynamic_friction2,
            static_friction2,
        })
    }
}

impl crate::ToValue for PhysicsMaterial {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("dynamicFriction", &self.dynamic_friction),
            crate::opt_field("m_DynamicFriction", &self.m_dynamic_friction),
            crate::opt_field("staticFriction", &self.static_friction),
            crate::opt_field("m_StaticFriction", &self.m_static_friction),
            crate::opt_field("bounciness", &self.bounciness),
            crate::opt_field("m_Bounciness", &self.m_bounciness),
            crate::opt_field("frictionCombine", &self.friction_combine),
            crate::opt_field("m_FrictionCombine", &self.m_friction_combine),
            crate::opt_field("bounceCombine", &self.bounce_combine),
            crate::opt_field("m_BounceCombine", &self.m_bounce_combine),
            crate::opt_field("frictionDirection2", &self.friction_direction2),
            crate::opt_field("dynamicFriction2", &self.dynamic_friction2),
            crate::opt_field("staticFriction2", &self.static_friction2),
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
                "dynamicFriction" => (self.dynamic_friction).inspect_value(path, offset, limit),
                "m_DynamicFriction" => (self.m_dynamic_friction).inspect_value(path, offset, limit),
                "staticFriction" => (self.static_friction).inspect_value(path, offset, limit),
                "m_StaticFriction" => (self.m_static_friction).inspect_value(path, offset, limit),
                "bounciness" => (self.bounciness).inspect_value(path, offset, limit),
                "m_Bounciness" => (self.m_bounciness).inspect_value(path, offset, limit),
                "frictionCombine" => (self.friction_combine).inspect_value(path, offset, limit),
                "m_FrictionCombine" => (self.m_friction_combine).inspect_value(path, offset, limit),
                "bounceCombine" => (self.bounce_combine).inspect_value(path, offset, limit),
                "m_BounceCombine" => (self.m_bounce_combine).inspect_value(path, offset, limit),
                "frictionDirection2" => {
                    (self.friction_direction2).inspect_value(path, offset, limit)
                }
                "dynamicFriction2" => (self.dynamic_friction2).inspect_value(path, offset, limit),
                "staticFriction2" => (self.static_friction2).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.dynamic_friction).inspect_value(path, offset, limit))
                .map(|v| ("dynamicFriction".to_string(), v)),
            ((self.m_dynamic_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicFriction".to_string(), v)),
            ((self.static_friction).inspect_value(path, offset, limit))
                .map(|v| ("staticFriction".to_string(), v)),
            ((self.m_static_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticFriction".to_string(), v)),
            ((self.bounciness).inspect_value(path, offset, limit))
                .map(|v| ("bounciness".to_string(), v)),
            ((self.m_bounciness).inspect_value(path, offset, limit))
                .map(|v| ("m_Bounciness".to_string(), v)),
            ((self.friction_combine).inspect_value(path, offset, limit))
                .map(|v| ("frictionCombine".to_string(), v)),
            ((self.m_friction_combine).inspect_value(path, offset, limit))
                .map(|v| ("m_FrictionCombine".to_string(), v)),
            ((self.bounce_combine).inspect_value(path, offset, limit))
                .map(|v| ("bounceCombine".to_string(), v)),
            ((self.m_bounce_combine).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceCombine".to_string(), v)),
            ((self.friction_direction2).inspect_value(path, offset, limit))
                .map(|v| ("frictionDirection2".to_string(), v)),
            ((self.dynamic_friction2).inspect_value(path, offset, limit))
                .map(|v| ("dynamicFriction2".to_string(), v)),
            ((self.static_friction2).inspect_value(path, offset, limit))
                .map(|v| ("staticFriction2".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
