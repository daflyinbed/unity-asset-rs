//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel_parent::ComputeShaderKernelParent;

#[derive(Debug, Clone)]
pub struct ComputeShaderPlatformVariant {
    pub target_renderer: i32,
    pub target_level: i32,
    pub kernels: Vec<ComputeShaderKernelParent>,
    pub constant_buffers: Vec<ComputeShaderCb>,
    pub resources_resolved: bool,
}

impl crate::Parse for ComputeShaderPlatformVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = rdr.read_i32()?;
        let target_level = rdr.read_i32()?;
        let kernels = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderKernelParent::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let constant_buffers = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderCb::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let resources_resolved = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ComputeShaderPlatformVariant {
            target_renderer,
            target_level,
            kernels,
            constant_buffers,
            resources_resolved,
        })
    }
}
