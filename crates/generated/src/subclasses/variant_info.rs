//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::graphics_state_info::GraphicsStateInfo;

#[derive(Debug, Clone)]
pub struct VariantInfo {
    pub keywords: Option<String>,
    pub shader: Option<PPtr>,
    pub pass_type: Option<i32>,
    pub shader_name: Option<String>,
    pub shader_asset_guid: Option<String>,
    pub keyword_names: Option<String>,
    pub shader_asset_local_identifier_in_file: Option<i64>,
    pub sub_shader_index: Option<i32>,
    pub pass_index: Option<i32>,
    pub graphics_state_info_set: Option<Vec<GraphicsStateInfo>>,
}

impl crate::Parse for VariantInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut keywords = None;
        let mut shader = None;
        let mut pass_type = None;
        let mut shader_name = None;
        let mut shader_asset_guid = None;
        let mut keyword_names = None;
        let mut shader_asset_local_identifier_in_file = None;
        let mut sub_shader_index = None;
        let mut pass_index = None;
        let mut graphics_state_info_set = None;
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_name = Some(rdr.read_aligned_string()?);
            shader_asset_guid = Some(rdr.read_aligned_string()?);
            shader_asset_local_identifier_in_file = Some(rdr.read_i64()?);
            keyword_names = Some(rdr.read_aligned_string()?);
            sub_shader_index = Some(rdr.read_i32()?);
            pass_index = Some(rdr.read_i32()?);
            graphics_state_info_set = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GraphicsStateInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_name = Some(rdr.read_aligned_string()?);
            shader_asset_guid = Some(rdr.read_aligned_string()?);
            keyword_names = Some(rdr.read_aligned_string()?);
            sub_shader_index = Some(rdr.read_i32()?);
            pass_index = Some(rdr.read_i32()?);
            graphics_state_info_set = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GraphicsStateInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
            keywords = Some(rdr.read_aligned_string()?);
            pass_type = Some(rdr.read_i32()?);
        }

        Ok(VariantInfo {
            keywords,
            shader,
            pass_type,
            shader_name,
            shader_asset_guid,
            keyword_names,
            shader_asset_local_identifier_in_file,
            sub_shader_index,
            pass_index,
            graphics_state_info_set,
        })
    }
}
