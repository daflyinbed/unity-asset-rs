//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SortingLayerEntry {
    pub name: String,
    pub user_id: u32,
    pub unique_id: u32,
    pub locked: bool,
}

impl crate::Parse for SortingLayerEntry {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let user_id = r.read_u32()?;
        let unique_id = r.read_u32()?;
        let locked = r.read_bool()?;
        r.align(4)?;

        Ok(SortingLayerEntry {
            name,
            user_id,
            unique_id,
            locked,
        })
    }
}
