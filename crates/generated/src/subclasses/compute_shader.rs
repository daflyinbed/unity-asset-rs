//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;
use crate::subclasses::shader_error::ShaderError;

#[derive(Debug, Clone)]
pub struct ComputeShader {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub kernels: Vec<ComputeShaderKernel>,
    pub constant_buffers: Vec<ComputeShaderCb>,
    pub errors: Vec<ShaderError>,
}

impl crate::Parse for ComputeShader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let kernels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderKernel::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let constant_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderCb::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let errors = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ShaderError::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ComputeShader {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            kernels,
            constant_buffers,
            errors,
        })
    }
}
