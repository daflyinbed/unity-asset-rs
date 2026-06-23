//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AssetImporter {
    pub parent: NamedObject,
}

impl crate::Parse for AssetImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;

        Ok(AssetImporter {
            parent,
        })
    }
}
