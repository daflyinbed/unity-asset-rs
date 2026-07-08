//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StaticBatchInfo {
    pub first_sub_mesh: u16,
    pub sub_mesh_count: u16,
}

impl crate::Parse for StaticBatchInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first_sub_mesh = rdr.read_u16()?;
        let sub_mesh_count = rdr.read_u16()?;

        Ok(StaticBatchInfo {
            first_sub_mesh,
            sub_mesh_count,
        })
    }
}
