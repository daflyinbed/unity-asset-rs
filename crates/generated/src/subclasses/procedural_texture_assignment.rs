//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::fast_property_name::FastPropertyName;

#[derive(Debug, Clone)]
pub struct ProceduralTextureAssignment {
    pub shader_prop: FastPropertyName,
    pub material: PPtr,
    pub base_uid: u32,
}

impl crate::Parse for ProceduralTextureAssignment {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_prop = FastPropertyName::parse(r, ver)?;
        r.align(4)?;
        let material = crate::PPtr::parse(r, ver)?;
        let base_uid = r.read_u32()?;

        Ok(ProceduralTextureAssignment {
            shader_prop,
            material,
            base_uid,
        })
    }
}
