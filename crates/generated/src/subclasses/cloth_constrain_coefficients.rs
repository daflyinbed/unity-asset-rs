//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClothConstrainCoefficients {
    pub max_distance: f32,
    pub max_distance_bias: Option<f32>,
    pub collision_sphere_distance: f32,
    pub collision_sphere_radius: Option<f32>,
}

impl crate::Parse for ClothConstrainCoefficients {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut max_distance = None;
        let mut max_distance_bias = None;
        let mut collision_sphere_distance = None;
        let mut collision_sphere_radius = None;
        max_distance = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            max_distance_bias = Some(rdr.read_f32()?);
            collision_sphere_radius = Some(rdr.read_f32()?);
        }
        collision_sphere_distance = Some(rdr.read_f32()?);

        Ok(ClothConstrainCoefficients {
            max_distance: max_distance.unwrap(),
            max_distance_bias,
            collision_sphere_distance: collision_sphere_distance.unwrap(),
            collision_sphere_radius,
        })
    }
}

impl crate::ToValue for ClothConstrainCoefficients {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("maxDistance".to_string(), self.max_distance.to_value())),
            crate::opt_field("maxDistanceBias", &self.max_distance_bias),
            Some((
                "collisionSphereDistance".to_string(),
                self.collision_sphere_distance.to_value(),
            )),
            crate::opt_field("collisionSphereRadius", &self.collision_sphere_radius),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "maxDistance" => (self.max_distance).inspect_value(path, offset, limit),
                "maxDistanceBias" => (self.max_distance_bias).inspect_value(path, offset, limit),
                "collisionSphereDistance" => {
                    (self.collision_sphere_distance).inspect_value(path, offset, limit)
                }
                "collisionSphereRadius" => {
                    (self.collision_sphere_radius).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.max_distance).inspect_value(path, offset, limit))
                .map(|v| ("maxDistance".to_string(), v)),
            ((self.max_distance_bias).inspect_value(path, offset, limit))
                .map(|v| ("maxDistanceBias".to_string(), v)),
            ((self.collision_sphere_distance).inspect_value(path, offset, limit))
                .map(|v| ("collisionSphereDistance".to_string(), v)),
            ((self.collision_sphere_radius).inspect_value(path, offset, limit))
                .map(|v| ("collisionSphereRadius".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
