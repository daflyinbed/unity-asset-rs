//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_function_desc::RayTracingShaderFunctionDesc;
use crate::subclasses::ray_tracing_shader_resources::RayTracingShaderResources;

#[derive(Debug, Clone)]
pub struct RayTracingShaderReflectionData {
    pub functions: Vec<RayTracingShaderFunctionDesc>,
    pub local_resources: RayTracingShaderResources,
    pub global_resources: RayTracingShaderResources,
    pub has_errors: bool,
    pub code: Vec<u8>,
}

impl crate::Parse for RayTracingShaderReflectionData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let functions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderFunctionDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let local_resources = RayTracingShaderResources::parse(r, ver)?;
        r.align(4)?;
        let global_resources = RayTracingShaderResources::parse(r, ver)?;
        r.align(4)?;
        let has_errors = r.read_bool()?;
        r.align(4)?;
        let code = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(RayTracingShaderReflectionData {
            functions,
            local_resources,
            global_resources,
            has_errors,
            code,
        })
    }
}
