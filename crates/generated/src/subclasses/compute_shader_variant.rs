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

impl crate::ToValue for ComputeShaderVariantKernelsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ComputeShaderVariantConstantBuffersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_1(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ComputeShaderVariant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "targetRenderer".to_string(),
                self.target_renderer.to_value(),
            )),
            Some(("targetLevel".to_string(), self.target_level.to_value())),
            Some(("kernels".to_string(), self.kernels.to_value())),
            Some((
                "constantBuffers".to_string(),
                self.constant_buffers.to_value(),
            )),
            crate::opt_field("resourcesResolved", &self.resources_resolved),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "targetRenderer" => (self.target_renderer).inspect_value(path, offset, limit),
                "targetLevel" => (self.target_level).inspect_value(path, offset, limit),
                "kernels" => (self.kernels).inspect_value(path, offset, limit),
                "constantBuffers" => (self.constant_buffers).inspect_value(path, offset, limit),
                "resourcesResolved" => (self.resources_resolved).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.target_renderer).inspect_value(path, offset, limit))
                .map(|v| ("targetRenderer".to_string(), v)),
            ((self.target_level).inspect_value(path, offset, limit))
                .map(|v| ("targetLevel".to_string(), v)),
            ((self.kernels).inspect_value(path, offset, limit)).map(|v| ("kernels".to_string(), v)),
            ((self.constant_buffers).inspect_value(path, offset, limit))
                .map(|v| ("constantBuffers".to_string(), v)),
            ((self.resources_resolved).inspect_value(path, offset, limit))
                .map(|v| ("resourcesResolved".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
