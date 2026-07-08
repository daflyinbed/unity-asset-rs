//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_reflection_data::RayTracingShaderReflectionData;

#[derive(Debug, Clone)]
pub struct RayTracingShaderVariant {
    pub target_renderer: i32,
    pub resource_reflection_data: RayTracingShaderReflectionData,
    pub editor_only_variant: Option<bool>,
}

impl crate::Parse for RayTracingShaderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = rdr.read_i32()?;
        let resource_reflection_data = RayTracingShaderReflectionData::parse(rdr, ver)?;
        rdr.align(4)?;
        let editor_only_variant = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(RayTracingShaderVariant {
            target_renderer,
            resource_reflection_data,
            editor_only_variant,
        })
    }
}
