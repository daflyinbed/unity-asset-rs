//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;
use crate::subclasses::compute_shader_kernel_parent::ComputeShaderKernelParent;

/// 版本枚举：ComputeShaderVariantKernelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderVariantKernelsVariant {
    Default(Vec<ComputeShaderKernel>),
    V2017_1(Vec<ComputeShaderKernel>),
    V2020_1(Vec<ComputeShaderKernelParent>),
}

impl crate::Parse for ComputeShaderVariantKernelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderKernelParent::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderKernel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderKernel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ComputeShaderVariantConstantBuffersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderVariantConstantBuffersVariant {
    Default(Vec<ComputeShaderCb>),
    V5_1(Vec<ComputeShaderCb>),
    V2017_1(Vec<ComputeShaderCb>),
}

impl crate::Parse for ComputeShaderVariantConstantBuffersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderCb::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderCb::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderCb::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderVariant {
    pub target_renderer: i32,
    pub target_level: i32,
    pub kernels: ComputeShaderVariantKernelsVariant,
    pub constant_buffers: ComputeShaderVariantConstantBuffersVariant,
    pub resources_resolved: Option<bool>,
}

impl crate::Parse for ComputeShaderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = rdr.read_i32()?;
        let target_level = rdr.read_i32()?;
        let kernels = ComputeShaderVariantKernelsVariant::parse(rdr, ver)?;
        let constant_buffers = ComputeShaderVariantConstantBuffersVariant::parse(rdr, ver)?;
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
