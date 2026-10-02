//! 自动生成。请勿手动修改。

use crate::subclasses::fast_property_name::FastPropertyName;

/// 版本枚举：ComputeShaderParamNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderParamNameVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for ComputeShaderParamNameVariant {
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

impl crate::ToValue for ComputeShaderParamNameVariant {
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

/// 版本枚举：ComputeShaderParamOffsetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderParamOffsetVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for ComputeShaderParamOffsetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderParamOffsetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderParamArraySizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderParamArraySizeVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for ComputeShaderParamArraySizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderParamArraySizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderParamRowCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderParamRowCountVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for ComputeShaderParamRowCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderParamRowCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderParamColCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderParamColCountVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for ComputeShaderParamColCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderParamColCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderParam {
    pub name: ComputeShaderParamNameVariant,
    pub r#type: i32,
    pub offset: ComputeShaderParamOffsetVariant,
    pub array_size: ComputeShaderParamArraySizeVariant,
    pub row_count: ComputeShaderParamRowCountVariant,
    pub col_count: ComputeShaderParamColCountVariant,
}

impl crate::Parse for ComputeShaderParam {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = ComputeShaderParamNameVariant::parse(rdr, ver)?;
        let r#type = rdr.read_i32()?;
        let offset = ComputeShaderParamOffsetVariant::parse(rdr, ver)?;
        let array_size = ComputeShaderParamArraySizeVariant::parse(rdr, ver)?;
        let row_count = ComputeShaderParamRowCountVariant::parse(rdr, ver)?;
        let col_count = ComputeShaderParamColCountVariant::parse(rdr, ver)?;

        Ok(ComputeShaderParam {
            name,
            r#type,
            offset,
            array_size,
            row_count,
            col_count,
        })
    }
}

impl crate::ToValue for ComputeShaderParam {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("offset".to_string(), self.offset.to_value())),
            Some(("arraySize".to_string(), self.array_size.to_value())),
            Some(("rowCount".to_string(), self.row_count.to_value())),
            Some(("colCount".to_string(), self.col_count.to_value())),
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
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                "arraySize" => (self.array_size).inspect_value(path, offset, limit),
                "rowCount" => (self.row_count).inspect_value(path, offset, limit),
                "colCount" => (self.col_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
            ((self.array_size).inspect_value(path, offset, limit))
                .map(|v| ("arraySize".to_string(), v)),
            ((self.row_count).inspect_value(path, offset, limit))
                .map(|v| ("rowCount".to_string(), v)),
            ((self.col_count).inspect_value(path, offset, limit))
                .map(|v| ("colCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
