//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_builtin_sampler::ComputeShaderBuiltinSampler;
use crate::subclasses::compute_shader_resource::ComputeShaderResource;
use crate::subclasses::fast_property_name::FastPropertyName;

#[derive(Debug, Clone)]
pub struct ComputeShaderKernel {
    pub name: FastPropertyName,
    pub cbs: Vec<ComputeShaderResource>,
    pub textures: Vec<ComputeShaderResource>,
    pub builtin_samplers: Vec<ComputeShaderBuiltinSampler>,
    pub in_buffers: Vec<ComputeShaderResource>,
    pub out_buffers: Vec<ComputeShaderResource>,
    pub code: Vec<u8>,
}

impl crate::Parse for ComputeShaderKernel {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = FastPropertyName::parse(r, ver)?;
        r.align(4)?;
        let cbs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let builtin_samplers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderBuiltinSampler::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let in_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let out_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let code = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ComputeShaderKernel {
            name,
            cbs,
            textures,
            builtin_samplers,
            in_buffers,
            out_buffers,
            code,
        })
    }
}
