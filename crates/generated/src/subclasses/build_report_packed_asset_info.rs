//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct BuildReportPackedAssetInfo {
    pub file_id: i64,
    pub class_id: i32,
    pub packed_size: i32,
    pub source_asset_guid: Guid,
}

impl crate::Parse for BuildReportPackedAssetInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let file_id = r.read_i64()?;
        let class_id = r.read_i32()?;
        let packed_size = r.read_i32()?;
        let source_asset_guid = Guid::parse(r, ver)?;

        Ok(BuildReportPackedAssetInfo {
            file_id,
            class_id,
            packed_size,
            source_asset_guid,
        })
    }
}
