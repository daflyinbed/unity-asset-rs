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

impl crate::ToValue for RayTracingShaderResources {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("textures".to_string(), self.textures.to_value())),
            Some((
                "builtinSamplers".to_string(),
                self.builtin_samplers.to_value(),
            )),
            Some(("inputBuffers".to_string(), self.input_buffers.to_value())),
            Some(("outputBuffers".to_string(), self.output_buffers.to_value())),
            Some((
                "constantBuffersDesc".to_string(),
                self.constant_buffers_desc.to_value(),
            )),
            Some((
                "constantBuffers".to_string(),
                self.constant_buffers.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "textures" => (self.textures).inspect_value(path, offset, limit),
                "builtinSamplers" => (self.builtin_samplers).inspect_value(path, offset, limit),
                "inputBuffers" => (self.input_buffers).inspect_value(path, offset, limit),
                "outputBuffers" => (self.output_buffers).inspect_value(path, offset, limit),
                "constantBuffersDesc" => {
                    (self.constant_buffers_desc).inspect_value(path, offset, limit)
                }
                "constantBuffers" => (self.constant_buffers).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.textures).inspect_value(path, offset, limit))
                .map(|v| ("textures".to_string(), v)),
            ((self.builtin_samplers).inspect_value(path, offset, limit))
                .map(|v| ("builtinSamplers".to_string(), v)),
            ((self.input_buffers).inspect_value(path, offset, limit))
                .map(|v| ("inputBuffers".to_string(), v)),
            ((self.output_buffers).inspect_value(path, offset, limit))
                .map(|v| ("outputBuffers".to_string(), v)),
            ((self.constant_buffers_desc).inspect_value(path, offset, limit))
                .map(|v| ("constantBuffersDesc".to_string(), v)),
            ((self.constant_buffers).inspect_value(path, offset, limit))
                .map(|v| ("constantBuffers".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
