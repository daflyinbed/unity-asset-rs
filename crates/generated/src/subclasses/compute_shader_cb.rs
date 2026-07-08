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
            rdr.align(4)?;
            Ok(Self::V5_6(v))
        } else {
            let v = ( { Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderCb {
    pub name: ComputeShaderCbNameVariant,
    pub byte_size: i32,
    pub params: Vec<ComputeShaderParam>,
}

impl crate::Parse for ComputeShaderCb {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = ComputeShaderCbNameVariant::parse(rdr, ver)?;
        let byte_size = rdr.read_i32()?;
        let params = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderParam::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(ComputeShaderCb {
            name,
            byte_size,
            params,
        })
    }
}
