//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SourceAssetIdentifier {
    pub r#type: String,
    pub assembly: String,
    pub name: String,
}

impl crate::Parse for SourceAssetIdentifier {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let assembly = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(SourceAssetIdentifier {
            r#type,
            assembly,
            name,
        })
    }
}
