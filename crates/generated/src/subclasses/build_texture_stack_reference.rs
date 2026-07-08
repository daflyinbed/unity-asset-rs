//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTextureStackReference {
    pub group_name: String,
    pub item_name: String,
}

impl crate::Parse for BuildTextureStackReference {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let group_name = rdr.read_aligned_string()?;
        let item_name = rdr.read_aligned_string()?;

        Ok(BuildTextureStackReference {
            group_name,
            item_name,
        })
    }
}
