//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderBuiltinSampler {
    pub sampler: u32,
    pub bind_point: i32,
}

impl crate::Parse for RayTracingShaderBuiltinSampler {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sampler = rdr.read_u32()?;
        let bind_point = rdr.read_i32()?;

        Ok(RayTracingShaderBuiltinSampler {
            sampler,
            bind_point,
        })
    }
}
