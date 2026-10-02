//! 自动生成。请勿手动修改。

use crate::subclasses::asset_stats::AssetStats;
use crate::subclasses::type_stats::TypeStats;

#[derive(Debug, Clone)]
pub struct ContentSummary {
    pub m_serialized_file_size: u64,
    pub m_generated_file_size: Option<u64>,
    pub m_reused_serialized_file_size: Option<u64>,
    pub m_size_reused_content_in_output_directory: u64,
    pub m_resource_data_size: u64,
    pub m_header_size: u64,
    pub m_serialized_file_count: i32,
    pub m_generated_file_count: Option<i32>,
    pub m_reused_serialized_file_count: Option<i32>,
    pub m_resource_file_count: i32,
    pub m_object_count: i32,
    pub m_type_stats_list: Vec<TypeStats>,
    pub m_asset_stats_list: Vec<AssetStats>,
}

impl crate::Parse for ContentSummary {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_serialized_file_size = None;
        let mut m_generated_file_size = None;
        let mut m_reused_serialized_file_size = None;
        let mut m_size_reused_content_in_output_directory = None;
        let mut m_resource_data_size = None;
        let mut m_header_size = None;
        let mut m_serialized_file_count = None;
        let mut m_generated_file_count = None;
        let mut m_reused_serialized_file_count = None;
        let mut m_resource_file_count = None;
        let mut m_object_count = None;
        let mut m_type_stats_list = None;
        let mut m_asset_stats_list = None;
        m_serialized_file_size = Some(rdr.read_u64()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_reused_serialized_file_size = Some(rdr.read_u64()?);
        } else {
            m_generated_file_size = Some(rdr.read_u64()?);
        }
        m_size_reused_content_in_output_directory = Some(rdr.read_u64()?);
        m_resource_data_size = Some(rdr.read_u64()?);
        m_header_size = Some(rdr.read_u64()?);
        m_serialized_file_count = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_reused_serialized_file_count = Some(rdr.read_i32()?);
        } else {
            m_generated_file_count = Some(rdr.read_i32()?);
        }
        m_resource_file_count = Some(rdr.read_i32()?);
        m_object_count = Some(rdr.read_i32()?);
        m_type_stats_list = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TypeStats::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_asset_stats_list = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AssetStats::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;

        Ok(ContentSummary {
            m_serialized_file_size: m_serialized_file_size.unwrap(),
            m_generated_file_size,
            m_reused_serialized_file_size,
            m_size_reused_content_in_output_directory: m_size_reused_content_in_output_directory
                .unwrap(),
            m_resource_data_size: m_resource_data_size.unwrap(),
            m_header_size: m_header_size.unwrap(),
            m_serialized_file_count: m_serialized_file_count.unwrap(),
            m_generated_file_count,
            m_reused_serialized_file_count,
            m_resource_file_count: m_resource_file_count.unwrap(),
            m_object_count: m_object_count.unwrap(),
            m_type_stats_list: m_type_stats_list.unwrap(),
            m_asset_stats_list: m_asset_stats_list.unwrap(),
        })
    }
}

impl crate::ToValue for ContentSummary {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_serializedFileSize".to_string(),
                self.m_serialized_file_size.to_value(),
            )),
            crate::opt_field("m_generatedFileSize", &self.m_generated_file_size),
            crate::opt_field(
                "m_reusedSerializedFileSize",
                &self.m_reused_serialized_file_size,
            ),
            Some((
                "m_sizeReusedContentInOutputDirectory".to_string(),
                self.m_size_reused_content_in_output_directory.to_value(),
            )),
            Some((
                "m_resourceDataSize".to_string(),
                self.m_resource_data_size.to_value(),
            )),
            Some(("m_headerSize".to_string(), self.m_header_size.to_value())),
            Some((
                "m_serializedFileCount".to_string(),
                self.m_serialized_file_count.to_value(),
            )),
            crate::opt_field("m_generatedFileCount", &self.m_generated_file_count),
            crate::opt_field(
                "m_reusedSerializedFileCount",
                &self.m_reused_serialized_file_count,
            ),
            Some((
                "m_resourceFileCount".to_string(),
                self.m_resource_file_count.to_value(),
            )),
            Some(("m_objectCount".to_string(), self.m_object_count.to_value())),
            Some((
                "m_typeStatsList".to_string(),
                self.m_type_stats_list.to_value(),
            )),
            Some((
                "m_assetStatsList".to_string(),
                self.m_asset_stats_list.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_serializedFileSize" => {
                    (self.m_serialized_file_size).inspect_value(path, offset, limit)
                }
                "m_generatedFileSize" => {
                    (self.m_generated_file_size).inspect_value(path, offset, limit)
                }
                "m_reusedSerializedFileSize" => {
                    (self.m_reused_serialized_file_size).inspect_value(path, offset, limit)
                }
                "m_sizeReusedContentInOutputDirectory" => (self
                    .m_size_reused_content_in_output_directory)
                    .inspect_value(path, offset, limit),
                "m_resourceDataSize" => {
                    (self.m_resource_data_size).inspect_value(path, offset, limit)
                }
                "m_headerSize" => (self.m_header_size).inspect_value(path, offset, limit),
                "m_serializedFileCount" => {
                    (self.m_serialized_file_count).inspect_value(path, offset, limit)
                }
                "m_generatedFileCount" => {
                    (self.m_generated_file_count).inspect_value(path, offset, limit)
                }
                "m_reusedSerializedFileCount" => {
                    (self.m_reused_serialized_file_count).inspect_value(path, offset, limit)
                }
                "m_resourceFileCount" => {
                    (self.m_resource_file_count).inspect_value(path, offset, limit)
                }
                "m_objectCount" => (self.m_object_count).inspect_value(path, offset, limit),
                "m_typeStatsList" => (self.m_type_stats_list).inspect_value(path, offset, limit),
                "m_assetStatsList" => (self.m_asset_stats_list).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_serialized_file_size).inspect_value(path, offset, limit))
                .map(|v| ("m_serializedFileSize".to_string(), v)),
            ((self.m_generated_file_size).inspect_value(path, offset, limit))
                .map(|v| ("m_generatedFileSize".to_string(), v)),
            ((self.m_reused_serialized_file_size).inspect_value(path, offset, limit))
                .map(|v| ("m_reusedSerializedFileSize".to_string(), v)),
            ((self.m_size_reused_content_in_output_directory).inspect_value(path, offset, limit))
                .map(|v| ("m_sizeReusedContentInOutputDirectory".to_string(), v)),
            ((self.m_resource_data_size).inspect_value(path, offset, limit))
                .map(|v| ("m_resourceDataSize".to_string(), v)),
            ((self.m_header_size).inspect_value(path, offset, limit))
                .map(|v| ("m_headerSize".to_string(), v)),
            ((self.m_serialized_file_count).inspect_value(path, offset, limit))
                .map(|v| ("m_serializedFileCount".to_string(), v)),
            ((self.m_generated_file_count).inspect_value(path, offset, limit))
                .map(|v| ("m_generatedFileCount".to_string(), v)),
            ((self.m_reused_serialized_file_count).inspect_value(path, offset, limit))
                .map(|v| ("m_reusedSerializedFileCount".to_string(), v)),
            ((self.m_resource_file_count).inspect_value(path, offset, limit))
                .map(|v| ("m_resourceFileCount".to_string(), v)),
            ((self.m_object_count).inspect_value(path, offset, limit))
                .map(|v| ("m_objectCount".to_string(), v)),
            ((self.m_type_stats_list).inspect_value(path, offset, limit))
                .map(|v| ("m_typeStatsList".to_string(), v)),
            ((self.m_asset_stats_list).inspect_value(path, offset, limit))
                .map(|v| ("m_assetStatsList".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
