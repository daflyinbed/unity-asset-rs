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
    pub precompiled: Option<Vec<u8>>,
    pub requirements: Option<i64>,
    pub code_hash: Option<u32>,
}

impl crate::Parse for RayTracingShaderReflectionData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let functions = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderFunctionDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let local_resources = RayTracingShaderResources::parse(rdr, ver)?;
        let global_resources = RayTracingShaderResources::parse(rdr, ver)?;
        let has_errors = rdr.read_bool()?;
        rdr.align(4)?;
        let code = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let precompiled = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let requirements = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(rdr.read_i64()?)
        } else {
            None
        };
        let code_hash = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };

        Ok(RayTracingShaderReflectionData {
            functions,
            local_resources,
            global_resources,
            has_errors,
            code,
            precompiled,
            requirements,
            code_hash,
        })
    }
}
