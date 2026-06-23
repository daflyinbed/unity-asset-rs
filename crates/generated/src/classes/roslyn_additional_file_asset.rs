//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct RoslynAdditionalFileAsset {
    pub parent: NamedObject,
    pub m_name: String,
}

impl crate::Parse for RoslynAdditionalFileAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(RoslynAdditionalFileAsset {
            parent,
            m_name,
        })
    }
}
