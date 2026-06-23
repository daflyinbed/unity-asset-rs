//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_param::RayTracingShaderParam;

#[derive(Debug, Clone)]
pub struct RayTracingShaderConstantBuffer {
    pub name: String,
    pub byte_size: i32,
    pub params: Vec<RayTracingShaderParam>,
}

impl crate::Parse for RayTracingShaderConstantBuffer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let byte_size = r.read_i32()?;
        let params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderParam::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(RayTracingShaderConstantBuffer {
            name,
            byte_size,
            params,
        })
    }
}
