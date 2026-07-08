//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;
use crate::subclasses::compute_shader_platform_variant::ComputeShaderPlatformVariant;
use crate::subclasses::compute_shader_variant::ComputeShaderVariant;


/// 版本枚举：ComputeShaderVariantsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderVariantsVariant {
    Default(Vec<ComputeShaderVariant>),
    V2020_1(Vec<ComputeShaderPlatformVariant>),
}

impl crate::Parse for ComputeShaderVariantsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderPlatformVariant::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2020_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderVariant::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShader {
    pub m_name: String,
    pub kernels: Option<Vec<ComputeShaderKernel>>,
    pub variants: Option<ComputeShaderVariantsVariant>,
    pub constant_buffers: Option<Vec<ComputeShaderCb>>,
}

impl crate::Parse for ComputeShader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let kernels = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderKernel::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let variants = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(ComputeShaderVariantsVariant::parse(r, ver)?)
        } else {
            None
        };
        let constant_buffers = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderCb::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(ComputeShader {
            m_name,
            kernels,
            variants,
            constant_buffers,
        })
    }
}
