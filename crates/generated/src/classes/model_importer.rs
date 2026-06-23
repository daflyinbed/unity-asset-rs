//! 自动生成。请勿手动修改。

use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct ModelImporter {
    pub parent: AssetImporter,
}

impl crate::Parse for ModelImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;

        Ok(ModelImporter {
            parent,
        })
    }
}
