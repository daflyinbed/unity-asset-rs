//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_cb::ComputeShaderCb;
use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;
use crate::subclasses::compute_shader_variant::ComputeShaderVariant;

/// 版本枚举：ComputeShaderVariantsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderVariantsVariant {
    Default(Vec<ComputeShaderVariant>),
    V2017_1(Vec<ComputeShaderVariant>),
}

impl crate::Parse for ComputeShaderVariantsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderVariant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderVariant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut kernels = None;
        let mut variants = None;
        let mut constant_buffers = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            variants = Some(ComputeShaderVariantsVariant::parse(rdr, ver)?);
        } else {
            kernels = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| ComputeShaderKernel::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            constant_buffers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| ComputeShaderCb::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }

        Ok(ComputeShader {
            m_name: m_name.unwrap(),
            kernels,
            variants,
            constant_buffers,
        })
    }
}
