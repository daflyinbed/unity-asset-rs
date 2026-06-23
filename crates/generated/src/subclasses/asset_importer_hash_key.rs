//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetImporterHashKey {
    pub r#type: i32,
    pub script_class: String,
}

impl crate::Parse for AssetImporterHashKey {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = r.read_i32()?;
        let script_class = r.read_aligned_string()?;
        r.align(4)?;

        Ok(AssetImporterHashKey {
            r#type,
            script_class,
        })
    }
}
