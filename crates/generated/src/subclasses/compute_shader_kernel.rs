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
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderKernelNameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderKernelCbsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderKernelCbsVariant {
    Default(Vec<ComputeShaderResource>),
    V2017_1(Vec<ComputeShaderResource>),
}

impl crate::Parse for ComputeShaderKernelCbsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderKernelCbsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderKernelTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderKernelTexturesVariant {
    Default(Vec<ComputeShaderResource>),
    V2017_1(Vec<ComputeShaderResource>),
}

impl crate::Parse for ComputeShaderKernelTexturesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderKernelTexturesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderBuiltinSampler::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderBuiltinSampler::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderKernelBuiltinSamplersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderKernelInBuffersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderKernelInBuffersVariant {
    Default(Vec<ComputeShaderResource>),
    V2017_1(Vec<ComputeShaderResource>),
}

impl crate::Parse for ComputeShaderKernelInBuffersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderKernelInBuffersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderKernelOutBuffersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderKernelOutBuffersVariant {
    Default(Vec<ComputeShaderResource>),
    V2017_1(Vec<ComputeShaderResource>),
}

impl crate::Parse for ComputeShaderKernelOutBuffersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderResource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderKernelOutBuffersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderKernel {
    pub name: Option<ComputeShaderKernelNameVariant>,
    pub cb_variant_indices: Option<Vec<u32>>,
    pub cbs: ComputeShaderKernelCbsVariant,
    pub textures: ComputeShaderKernelTexturesVariant,
    pub builtin_samplers: ComputeShaderKernelBuiltinSamplersVariant,
    pub in_buffers: ComputeShaderKernelInBuffersVariant,
    pub out_buffers: ComputeShaderKernelOutBuffersVariant,
    pub code: Vec<u8>,
    pub thread_group_size: Option<Vec<u32>>,
    pub requirements: Option<i64>,
}

impl crate::Parse for ComputeShaderKernel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut cb_variant_indices = None;
        let mut cbs = None;
        let mut textures = None;
        let mut builtin_samplers = None;
        let mut in_buffers = None;
        let mut out_buffers = None;
        let mut code = None;
        let mut thread_group_size = None;
        let mut requirements = None;
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            cb_variant_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            name = Some(ComputeShaderKernelNameVariant::parse(rdr, ver)?);
        }
        cbs = Some(ComputeShaderKernelCbsVariant::parse(rdr, ver)?);
        textures = Some(ComputeShaderKernelTexturesVariant::parse(rdr, ver)?);
        builtin_samplers = Some(ComputeShaderKernelBuiltinSamplersVariant::parse(rdr, ver)?);
        in_buffers = Some(ComputeShaderKernelInBuffersVariant::parse(rdr, ver)?);
        out_buffers = Some(ComputeShaderKernelOutBuffersVariant::parse(rdr, ver)?);
        code = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            thread_group_size = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            requirements = Some(rdr.read_i64()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            thread_group_size = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }

        Ok(ComputeShaderKernel {
            name,
            cb_variant_indices,
            cbs: cbs.unwrap(),
            textures: textures.unwrap(),
            builtin_samplers: builtin_samplers.unwrap(),
            in_buffers: in_buffers.unwrap(),
            out_buffers: out_buffers.unwrap(),
            code: code.unwrap(),
            thread_group_size,
            requirements,
        })
    }
}

impl crate::ToValue for ComputeShaderKernel {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("name", &self.name),
            crate::opt_field("cbVariantIndices", &self.cb_variant_indices),
            Some(("cbs".to_string(), self.cbs.to_value())),
            Some(("textures".to_string(), self.textures.to_value())),
            Some((
                "builtinSamplers".to_string(),
                self.builtin_samplers.to_value(),
            )),
            Some(("inBuffers".to_string(), self.in_buffers.to_value())),
            Some(("outBuffers".to_string(), self.out_buffers.to_value())),
            Some(("code".to_string(), self.code.to_value())),
            crate::opt_field("threadGroupSize", &self.thread_group_size),
            crate::opt_field("requirements", &self.requirements),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "cbVariantIndices" => (self.cb_variant_indices).inspect_value(path, offset, limit),
                "cbs" => (self.cbs).inspect_value(path, offset, limit),
                "textures" => (self.textures).inspect_value(path, offset, limit),
                "builtinSamplers" => (self.builtin_samplers).inspect_value(path, offset, limit),
                "inBuffers" => (self.in_buffers).inspect_value(path, offset, limit),
                "outBuffers" => (self.out_buffers).inspect_value(path, offset, limit),
                "code" => (self.code).inspect_value(path, offset, limit),
                "threadGroupSize" => (self.thread_group_size).inspect_value(path, offset, limit),
                "requirements" => (self.requirements).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.cb_variant_indices).inspect_value(path, offset, limit))
                .map(|v| ("cbVariantIndices".to_string(), v)),
            ((self.cbs).inspect_value(path, offset, limit)).map(|v| ("cbs".to_string(), v)),
            ((self.textures).inspect_value(path, offset, limit))
                .map(|v| ("textures".to_string(), v)),
            ((self.builtin_samplers).inspect_value(path, offset, limit))
                .map(|v| ("builtinSamplers".to_string(), v)),
            ((self.in_buffers).inspect_value(path, offset, limit))
                .map(|v| ("inBuffers".to_string(), v)),
            ((self.out_buffers).inspect_value(path, offset, limit))
                .map(|v| ("outBuffers".to_string(), v)),
            ((self.code).inspect_value(path, offset, limit)).map(|v| ("code".to_string(), v)),
            ((self.thread_group_size).inspect_value(path, offset, limit))
                .map(|v| ("threadGroupSize".to_string(), v)),
            ((self.requirements).inspect_value(path, offset, limit))
                .map(|v| ("requirements".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
