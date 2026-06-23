//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_param::ComputeShaderParam;
use crate::subclasses::fast_property_name::FastPropertyName;

#[derive(Debug, Clone)]
pub struct ComputeShaderCb {
    pub name: FastPropertyName,
    pub byte_size: i32,
    pub params: Vec<ComputeShaderParam>,
}

impl crate::Parse for ComputeShaderCb {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = FastPropertyName::parse(r, ver)?;
        r.align(4)?;
        let byte_size = r.read_i32()?;
        let params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComputeShaderParam::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ComputeShaderCb {
            name,
            byte_size,
            params,
        })
    }
}
