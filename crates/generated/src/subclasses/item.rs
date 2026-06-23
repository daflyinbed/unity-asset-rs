//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Item {
    pub marked_for_removal: bool,
    pub int_download_resolution: i32,
    pub int_name_conflict_resolution: i32,
}

impl crate::Parse for Item {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let marked_for_removal = r.read_bool()?;
        r.align(4)?;
        let int_download_resolution = r.read_i32()?;
        let int_name_conflict_resolution = r.read_i32()?;

        Ok(Item {
            marked_for_removal,
            int_download_resolution,
            int_name_conflict_resolution,
        })
    }
}
