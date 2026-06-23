//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClothConstrainCoefficients {
    pub max_distance: f32,
    pub max_distance_bias: f32,
    pub collision_sphere_radius: f32,
    pub collision_sphere_distance: f32,
}

impl crate::Parse for ClothConstrainCoefficients {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let max_distance = r.read_f32()?;
        let max_distance_bias = r.read_f32()?;
        let collision_sphere_radius = r.read_f32()?;
        let collision_sphere_distance = r.read_f32()?;

        Ok(ClothConstrainCoefficients {
            max_distance,
            max_distance_bias,
            collision_sphere_radius,
            collision_sphere_distance,
        })
    }
}
