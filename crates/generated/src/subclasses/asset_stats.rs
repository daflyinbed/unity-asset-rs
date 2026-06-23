//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AssetStats {
    pub source_asset_guid: Guid,
    pub source_asset_path: String,
    pub size: u64,
    pub object_count: i32,
    pub resource_count: i32,
}

impl crate::Parse for AssetStats {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source_asset_guid = Guid::parse(r, ver)?;
        let source_asset_path = r.read_aligned_string()?;
        r.align(4)?;
        let size = r.read_u64()?;
        let object_count = r.read_i32()?;
        let resource_count = r.read_i32()?;

        Ok(AssetStats {
            source_asset_guid,
            source_asset_path,
            size,
            object_count,
            resource_count,
        })
    }
}
