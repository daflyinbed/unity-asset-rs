//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetBundleScriptInfo {
    pub class_name: String,
    pub name_space: String,
    pub assembly_name: String,
    pub hash: u32,
}

impl crate::Parse for AssetBundleScriptInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let class_name = r.read_aligned_string()?;
        r.align(4)?;
        let name_space = r.read_aligned_string()?;
        r.align(4)?;
        let assembly_name = r.read_aligned_string()?;
        r.align(4)?;
        let hash = r.read_u32()?;

        Ok(AssetBundleScriptInfo {
            class_name,
            name_space,
            assembly_name,
            hash,
        })
    }
}
