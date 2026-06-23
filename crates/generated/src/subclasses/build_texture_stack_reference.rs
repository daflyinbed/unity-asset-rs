//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTextureStackReference {
    pub group_name: String,
    pub item_name: String,
}

impl crate::Parse for BuildTextureStackReference {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let group_name = r.read_aligned_string()?;
        r.align(4)?;
        let item_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(BuildTextureStackReference {
            group_name,
            item_name,
        })
    }
}
