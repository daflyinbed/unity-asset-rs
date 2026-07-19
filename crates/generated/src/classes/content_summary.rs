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
