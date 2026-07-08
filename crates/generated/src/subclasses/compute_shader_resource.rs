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
            let v = ( { Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
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
            let v = ( { Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
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
