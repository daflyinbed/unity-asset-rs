//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderResource {
    pub name: String,
    pub bind_point: i32,
    pub sampler_bind_point: i32,
    pub tex_dimension: i32,
    pub ray_gen_mask: u64,
}

impl crate::Parse for RayTracingShaderResource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let bind_point = r.read_i32()?;
        let sampler_bind_point = r.read_i32()?;
        let tex_dimension = r.read_i32()?;
        let ray_gen_mask = r.read_u64()?;

        Ok(RayTracingShaderResource {
            name,
            bind_point,
            sampler_bind_point,
            tex_dimension,
            ray_gen_mask,
        })
    }
}
