//! 自动生成。请勿手动修改。

use crate::subclasses::fast_property_name::FastPropertyName;

#[derive(Debug, Clone)]
pub struct ComputeShaderResource {
    pub name: FastPropertyName,
    pub bind_point: i32,
}

impl crate::Parse for ComputeShaderResource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = FastPropertyName::parse(r, ver)?;
        r.align(4)?;
        let bind_point = r.read_i32()?;

        Ok(ComputeShaderResource {
            name,
            bind_point,
        })
    }
}
