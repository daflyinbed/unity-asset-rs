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

impl crate::ToValue for BuildReportPackedAssetInfoPackedSizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
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
        packed_size = Some(BuildReportPackedAssetInfoPackedSizeVariant::parse(
            rdr, ver,
        )?);
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

impl crate::ToValue for BuildReportPackedAssetInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("fileID".to_string(), self.file_id.to_value())),
            Some(("classID".to_string(), self.class_id.to_value())),
            Some(("packedSize".to_string(), self.packed_size.to_value())),
            Some((
                "sourceAssetGUID".to_string(),
                self.source_asset_guid.to_value(),
            )),
            crate::opt_field("offset", &self.offset),
            crate::opt_field("buildTimeAssetPath", &self.build_time_asset_path),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "fileID" => (self.file_id).inspect_value(path, offset, limit),
                "classID" => (self.class_id).inspect_value(path, offset, limit),
                "packedSize" => (self.packed_size).inspect_value(path, offset, limit),
                "sourceAssetGUID" => (self.source_asset_guid).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                "buildTimeAssetPath" => {
                    (self.build_time_asset_path).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.file_id).inspect_value(path, offset, limit)).map(|v| ("fileID".to_string(), v)),
            ((self.class_id).inspect_value(path, offset, limit))
                .map(|v| ("classID".to_string(), v)),
            ((self.packed_size).inspect_value(path, offset, limit))
                .map(|v| ("packedSize".to_string(), v)),
            ((self.source_asset_guid).inspect_value(path, offset, limit))
                .map(|v| ("sourceAssetGUID".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
            ((self.build_time_asset_path).inspect_value(path, offset, limit))
                .map(|v| ("buildTimeAssetPath".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
