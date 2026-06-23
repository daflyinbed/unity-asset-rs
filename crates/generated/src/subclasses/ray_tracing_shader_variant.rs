//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_reflection_data::RayTracingShaderReflectionData;

#[derive(Debug, Clone)]
pub struct RayTracingShaderVariant {
    pub target_renderer: i32,
    pub resource_reflection_data: RayTracingShaderReflectionData,
}

impl crate::Parse for RayTracingShaderVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = r.read_i32()?;
        let resource_reflection_data = RayTracingShaderReflectionData::parse(r, ver)?;
        r.align(4)?;

        Ok(RayTracingShaderVariant {
            target_renderer,
            resource_reflection_data,
        })
    }
}
