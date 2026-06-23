//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PropertyModification {
    pub target: PPtr,
    pub property_path: String,
    pub value: String,
    pub object_reference: PPtr,
}

impl crate::Parse for PropertyModification {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target = crate::PPtr::parse(r, ver)?;
        let property_path = r.read_aligned_string()?;
        r.align(4)?;
        let value = r.read_aligned_string()?;
        r.align(4)?;
        let object_reference = crate::PPtr::parse(r, ver)?;

        Ok(PropertyModification {
            target,
            property_path,
            value,
            object_reference,
        })
    }
}
