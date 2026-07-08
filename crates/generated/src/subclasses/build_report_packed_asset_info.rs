//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;


/// 版本枚举：BuildReportPackedAssetInfoPackedSizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildReportPackedAssetInfoPackedSizeVariant {
    Default(i32),
    V2019_3(u64),
}

impl crate::Parse for BuildReportPackedAssetInfoPackedSizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u64()?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuildReportPackedAssetInfo {
    pub file_id: i64,
    pub class_id: i32,
    pub packed_size: BuildReportPackedAssetInfoPackedSizeVariant,
    pub source_asset_guid: Guid,
    pub offset: Option<u64>,
    pub build_time_asset_path: Option<String>,
}

impl crate::Parse for BuildReportPackedAssetInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut file_id = None;
        let mut class_id = None;
        let mut packed_size = None;
        let mut source_asset_guid = None;
        let mut offset = None;
        let mut build_time_asset_path = None;
            file_id = Some(rdr.read_i64()?);
            class_id = Some(rdr.read_i32()?);
            packed_size = Some(BuildReportPackedAssetInfoPackedSizeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            offset = Some(rdr.read_u64()?);
        } else {
        }
            source_asset_guid = Some(Guid::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 2) {
            build_time_asset_path = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(BuildReportPackedAssetInfo {
            file_id: file_id.unwrap(),
            class_id: class_id.unwrap(),
            packed_size: packed_size.unwrap(),
            source_asset_guid: source_asset_guid.unwrap(),
            offset,
            build_time_asset_path,
        })
    }
}
