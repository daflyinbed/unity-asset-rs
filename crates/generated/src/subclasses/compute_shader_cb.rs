//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_param::ComputeShaderParam;
use crate::subclasses::fast_property_name::FastPropertyName;

/// 版本枚举：ComputeShaderCbNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderCbNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for ComputeShaderCbNameVariant {
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

impl crate::ToValue for ComputeShaderCbNameVariant {
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

/// 版本枚举：ComputeShaderCbParamsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderCbParamsVariant {
    Default(Vec<ComputeShaderParam>),
    V2017_1(Vec<ComputeShaderParam>),
}

impl crate::Parse for ComputeShaderCbParamsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderParam::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ComputeShaderParam::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderCbParamsVariant {
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
pub struct ComputeShaderCb {
    pub name: ComputeShaderCbNameVariant,
    pub byte_size: i32,
    pub params: ComputeShaderCbParamsVariant,
}

impl crate::Parse for ComputeShaderCb {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = ComputeShaderCbNameVariant::parse(rdr, ver)?;
        let byte_size = rdr.read_i32()?;
        let params = ComputeShaderCbParamsVariant::parse(rdr, ver)?;

        Ok(ComputeShaderCb {
            name,
            byte_size,
            params,
        })
    }
}

impl crate::ToValue for ComputeShaderCb {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("byteSize".to_string(), self.byte_size.to_value())),
            Some(("params".to_string(), self.params.to_value())),
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
                "byteSize" => (self.byte_size).inspect_value(path, offset, limit),
                "params" => (self.params).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.byte_size).inspect_value(path, offset, limit))
                .map(|v| ("byteSize".to_string(), v)),
            ((self.params).inspect_value(path, offset, limit)).map(|v| ("params".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
