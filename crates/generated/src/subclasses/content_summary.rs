//! 自动生成。请勿手动修改。

use crate::subclasses::asset_stats::AssetStats;
use crate::subclasses::type_stats::TypeStats;

#[derive(Debug, Clone)]
pub struct ContentSummary {
    pub m_serialized_file_size: u64,
    pub m_generated_file_size: u64,
    pub m_size_reused_content_in_output_directory: u64,
    pub m_resource_data_size: u64,
    pub m_header_size: u64,
    pub m_serialized_file_count: i32,
    pub m_generated_file_count: i32,
    pub m_resource_file_count: i32,
    pub m_object_count: i32,
    pub m_type_stats_list: Vec<TypeStats>,
    pub m_asset_stats_list: Vec<AssetStats>,
}

impl crate::Parse for ContentSummary {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_serialized_file_size = r.read_u64()?;
        let m_generated_file_size = r.read_u64()?;
        let m_size_reused_content_in_output_directory = r.read_u64()?;
        let m_resource_data_size = r.read_u64()?;
        let m_header_size = r.read_u64()?;
        let m_serialized_file_count = r.read_i32()?;
        let m_generated_file_count = r.read_i32()?;
        let m_resource_file_count = r.read_i32()?;
        let m_object_count = r.read_i32()?;
        let m_type_stats_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TypeStats::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_asset_stats_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AssetStats::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ContentSummary {
            m_serialized_file_size,
            m_generated_file_size,
            m_size_reused_content_in_output_directory,
            m_resource_data_size,
            m_header_size,
            m_serialized_file_count,
            m_generated_file_count,
            m_resource_file_count,
            m_object_count,
            m_type_stats_list,
            m_asset_stats_list,
        })
    }
}
