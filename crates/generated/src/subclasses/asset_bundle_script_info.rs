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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let class_name = rdr.read_aligned_string()?;
        let name_space = rdr.read_aligned_string()?;
        let assembly_name = rdr.read_aligned_string()?;
        let hash = rdr.read_u32()?;

        Ok(AssetBundleScriptInfo {
            class_name,
            name_space,
            assembly_name,
            hash,
        })
    }
}
