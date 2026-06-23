//! 自动生成。请勿手动修改。

use crate::subclasses::fast_property_name::FastPropertyName;

#[derive(Debug, Clone)]
pub struct SerializedShaderFloatValue {
    pub val: f32,
    pub name: FastPropertyName,
}

impl crate::Parse for SerializedShaderFloatValue {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let val = r.read_f32()?;
        let name = FastPropertyName::parse(r, ver)?;
        r.align(4)?;

        Ok(SerializedShaderFloatValue {
            val,
            name,
        })
    }
}
