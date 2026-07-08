//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;

#[derive(Debug, Clone)]
pub struct ComputeShaderVariant {
    pub target_renderer: i32,
    pub target_level: i32,
    pub kernels: Vec<ComputeShaderKernel>,
    pub constant_buffers: Vec<ComputeShaderCb>,
    pub resources_resolved: Option<bool>,
}

impl crate::Parse for ComputeShaderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = rdr.read_i32()?;
        let target_level = rdr.read_i32()?;
        let kernels = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderKernel::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let constant_buffers = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderCb::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let resources_resolved = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(ComputeShaderVariant {
            target_renderer,
            target_level,
            kernels,
            constant_buffers,
            resources_resolved,
        })
    }
}
