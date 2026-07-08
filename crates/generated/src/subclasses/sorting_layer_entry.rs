//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SortingLayerEntry {
    pub name: String,
    pub user_id: Option<u32>,
    pub unique_id: u32,
}

impl crate::Parse for SortingLayerEntry {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let user_id = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let unique_id = rdr.read_u32()?;
        rdr.align(4)?;

        Ok(SortingLayerEntry {
            name,
            user_id,
            unique_id,
        })
    }
}
