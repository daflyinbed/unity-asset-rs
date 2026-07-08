//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_id::RayTracingShaderId;

#[derive(Debug, Clone)]
pub struct RayTracingShaderFunctionDesc {
    pub identifier: RayTracingShaderId,
    pub payload_size_in_bytes: u32,
    pub attribute_size_in_bytes: u32,
}

impl crate::Parse for RayTracingShaderFunctionDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let identifier = RayTracingShaderId::parse(rdr, ver)?;
        let payload_size_in_bytes = rdr.read_u32()?;
        let attribute_size_in_bytes = rdr.read_u32()?;

        Ok(RayTracingShaderFunctionDesc {
            identifier,
            payload_size_in_bytes,
            attribute_size_in_bytes,
        })
    }
}
