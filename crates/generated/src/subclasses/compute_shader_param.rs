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
