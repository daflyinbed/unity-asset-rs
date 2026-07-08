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
        let mut name = None;
        let mut user_id = None;
        let mut unique_id = None;
            name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            user_id = Some(rdr.read_u32()?);
        }
            unique_id = Some(rdr.read_u32()?);
            rdr.align(4)?;

        Ok(SortingLayerEntry {
            name: name.unwrap(),
            user_id,
            unique_id: unique_id.unwrap(),
        })
    }
}
