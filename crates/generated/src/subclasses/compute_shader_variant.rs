//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;

#[derive(Debug, Clone)]
pub struct ComputeShaderVariant {
    pub target_renderer: i32,
    pub target_level: i32,
    pub kernels: Vec<ComputeShaderKernel>,
    pub constant_buffers: Vec<ComputeShaderCb>,
}

impl crate::Parse for ComputeShaderVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = r.read_i32()?;
        let target_level = r.read_i32()?;
        let kernels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderKernel::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let constant_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderCb::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ComputeShaderVariant {
            target_renderer,
            target_level,
            kernels,
            constant_buffers,
        })
    }
}
