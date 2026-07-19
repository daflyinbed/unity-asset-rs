//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_builtin_sampler::RayTracingShaderBuiltinSampler;
use crate::subclasses::ray_tracing_shader_constant_buffer::RayTracingShaderConstantBuffer;
use crate::subclasses::ray_tracing_shader_resource::RayTracingShaderResource;

#[derive(Debug, Clone)]
pub struct RayTracingShaderResources {
    pub textures: Vec<RayTracingShaderResource>,
    pub builtin_samplers: Vec<RayTracingShaderBuiltinSampler>,
    pub input_buffers: Vec<RayTracingShaderResource>,
    pub output_buffers: Vec<RayTracingShaderResource>,
    pub constant_buffers_desc: Vec<RayTracingShaderConstantBuffer>,
    pub constant_buffers: Vec<RayTracingShaderResource>,
}

impl crate::Parse for RayTracingShaderResources {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let textures = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderResource::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let builtin_samplers = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderBuiltinSampler::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let input_buffers = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderResource::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let output_buffers = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderResource::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let constant_buffers_desc = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderConstantBuffer::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let constant_buffers = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderResource::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(RayTracingShaderResources {
            textures,
            builtin_samplers,
            input_buffers,
            output_buffers,
            constant_buffers_desc,
            constant_buffers,
        })
    }
}
