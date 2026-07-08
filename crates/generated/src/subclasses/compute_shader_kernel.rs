//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_builtin_sampler::ComputeShaderBuiltinSampler;
use crate::subclasses::compute_shader_resource::ComputeShaderResource;
use crate::subclasses::fast_property_name::FastPropertyName;


/// 版本枚举：ComputeShaderKernelNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderKernelNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for ComputeShaderKernelNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::V5_6(v))
        } else {
            let v = ( { Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ComputeShaderKernelBuiltinSamplersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderKernelBuiltinSamplersVariant {
    Default(Vec<ComputeShaderBuiltinSampler>),
    V2017_1(Vec<ComputeShaderBuiltinSampler>),
}

impl crate::Parse for ComputeShaderKernelBuiltinSamplersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderBuiltinSampler::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderBuiltinSampler::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderKernel {
    pub name: Option<ComputeShaderKernelNameVariant>,
    pub cb_variant_indices: Option<Vec<u32>>,
    pub cbs: Vec<ComputeShaderResource>,
    pub textures: Vec<ComputeShaderResource>,
    pub builtin_samplers: ComputeShaderKernelBuiltinSamplersVariant,
    pub in_buffers: Vec<ComputeShaderResource>,
    pub out_buffers: Vec<ComputeShaderResource>,
    pub code: Vec<u8>,
    pub thread_group_size: Option<Vec<u32>>,
    pub requirements: Option<i64>,
}

impl crate::Parse for ComputeShaderKernel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(ComputeShaderKernelNameVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let cb_variant_indices = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let cbs = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let textures = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let builtin_samplers = ComputeShaderKernelBuiltinSamplersVariant::parse(rdr, ver)?;
        let in_buffers = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let out_buffers = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderResource::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let code = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let thread_group_size = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let requirements = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(rdr.read_i64()?)
        } else {
            None
        };

        Ok(ComputeShaderKernel {
            name,
            cb_variant_indices,
            cbs,
            textures,
            builtin_samplers,
            in_buffers,
            out_buffers,
            code,
            thread_group_size,
            requirements,
        })
    }
}
