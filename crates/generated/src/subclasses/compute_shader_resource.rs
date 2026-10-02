//! 自动生成。请勿手动修改。

use crate::subclasses::compute_buffer_counter::ComputeBufferCounter;
use crate::subclasses::fast_property_name::FastPropertyName;

/// 版本枚举：ComputeShaderResourceNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderResourceNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for ComputeShaderResourceNameVariant {
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

impl crate::ToValue for ComputeShaderResourceNameVariant {
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

/// 版本枚举：ComputeShaderResourceGeneratedNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderResourceGeneratedNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for ComputeShaderResourceGeneratedNameVariant {
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

impl crate::ToValue for ComputeShaderResourceGeneratedNameVariant {
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
#[derive(Debug, Clone)]
pub struct ComputeShaderResource {
    pub name: ComputeShaderResourceNameVariant,
    pub bind_point: i32,
    pub generated_name: Option<ComputeShaderResourceGeneratedNameVariant>,
    pub secondary_bind_point: Option<i32>,
    pub counter: Option<ComputeBufferCounter>,
    pub sampler_bind_point: Option<i32>,
    pub tex_dimension: Option<i32>,
    pub res_type: Option<i32>,
}

impl crate::Parse for ComputeShaderResource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut bind_point = None;
        let mut generated_name = None;
        let mut secondary_bind_point = None;
        let mut counter = None;
        let mut sampler_bind_point = None;
        let mut tex_dimension = None;
        let mut res_type = None;
        name = Some(ComputeShaderResourceNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            generated_name = Some(ComputeShaderResourceGeneratedNameVariant::parse(rdr, ver)?);
        } else {
        }
        bind_point = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            sampler_bind_point = Some(rdr.read_i32()?);
            tex_dimension = Some(rdr.read_i32()?);
            res_type = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            sampler_bind_point = Some(rdr.read_i32()?);
            tex_dimension = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 2, 3) {
            counter = Some(ComputeBufferCounter::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            secondary_bind_point = Some(rdr.read_i32()?);
        } else {
        }

        Ok(ComputeShaderResource {
            name: name.unwrap(),
            bind_point: bind_point.unwrap(),
            generated_name,
            secondary_bind_point,
            counter,
            sampler_bind_point,
            tex_dimension,
            res_type,
        })
    }
}

impl crate::ToValue for ComputeShaderResource {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("bindPoint".to_string(), self.bind_point.to_value())),
            crate::opt_field("generatedName", &self.generated_name),
            crate::opt_field("secondaryBindPoint", &self.secondary_bind_point),
            crate::opt_field("counter", &self.counter),
            crate::opt_field("samplerBindPoint", &self.sampler_bind_point),
            crate::opt_field("texDimension", &self.tex_dimension),
            crate::opt_field("resType", &self.res_type),
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
                "bindPoint" => (self.bind_point).inspect_value(path, offset, limit),
                "generatedName" => (self.generated_name).inspect_value(path, offset, limit),
                "secondaryBindPoint" => {
                    (self.secondary_bind_point).inspect_value(path, offset, limit)
                }
                "counter" => (self.counter).inspect_value(path, offset, limit),
                "samplerBindPoint" => (self.sampler_bind_point).inspect_value(path, offset, limit),
                "texDimension" => (self.tex_dimension).inspect_value(path, offset, limit),
                "resType" => (self.res_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.bind_point).inspect_value(path, offset, limit))
                .map(|v| ("bindPoint".to_string(), v)),
            ((self.generated_name).inspect_value(path, offset, limit))
                .map(|v| ("generatedName".to_string(), v)),
            ((self.secondary_bind_point).inspect_value(path, offset, limit))
                .map(|v| ("secondaryBindPoint".to_string(), v)),
            ((self.counter).inspect_value(path, offset, limit)).map(|v| ("counter".to_string(), v)),
            ((self.sampler_bind_point).inspect_value(path, offset, limit))
                .map(|v| ("samplerBindPoint".to_string(), v)),
            ((self.tex_dimension).inspect_value(path, offset, limit))
                .map(|v| ("texDimension".to_string(), v)),
            ((self.res_type).inspect_value(path, offset, limit))
                .map(|v| ("resType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
