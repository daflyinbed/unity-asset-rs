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
