//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderResource {
    pub name: String,
    pub bind_point: i32,
    pub sampler_bind_point: i32,
    pub tex_dimension: i32,
    pub ray_gen_mask: u64,
    pub array_size: Option<i32>,
    pub multisampled: Option<bool>,
}

impl crate::Parse for RayTracingShaderResource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let bind_point = rdr.read_i32()?;
        let sampler_bind_point = rdr.read_i32()?;
        let tex_dimension = rdr.read_i32()?;
        let ray_gen_mask = rdr.read_u64()?;
        let array_size = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let multisampled = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(RayTracingShaderResource {
            name,
            bind_point,
            sampler_bind_point,
            tex_dimension,
            ray_gen_mask,
            array_size,
            multisampled,
        })
    }
}
