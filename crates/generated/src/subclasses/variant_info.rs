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
            graphics_state_info_set = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| GraphicsStateInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_name = Some(rdr.read_aligned_string()?);
            shader_asset_guid = Some(rdr.read_aligned_string()?);
            keyword_names = Some(rdr.read_aligned_string()?);
            sub_shader_index = Some(rdr.read_i32()?);
            pass_index = Some(rdr.read_i32()?);
            graphics_state_info_set = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| GraphicsStateInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for VariantInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("keywords", &self.keywords),
            crate::opt_field("shader", &self.shader),
            crate::opt_field("passType", &self.pass_type),
            crate::opt_field("shaderName", &self.shader_name),
            crate::opt_field("shaderAssetGUID", &self.shader_asset_guid),
            crate::opt_field("keywordNames", &self.keyword_names),
            crate::opt_field(
                "shaderAssetLocalIdentifierInFile",
                &self.shader_asset_local_identifier_in_file,
            ),
            crate::opt_field("subShaderIndex", &self.sub_shader_index),
            crate::opt_field("passIndex", &self.pass_index),
            crate::opt_field("graphicsStateInfoSet", &self.graphics_state_info_set),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "keywords" => (self.keywords).inspect_value(path, offset, limit),
                "shader" => (self.shader).inspect_value(path, offset, limit),
                "passType" => (self.pass_type).inspect_value(path, offset, limit),
                "shaderName" => (self.shader_name).inspect_value(path, offset, limit),
                "shaderAssetGUID" => (self.shader_asset_guid).inspect_value(path, offset, limit),
                "keywordNames" => (self.keyword_names).inspect_value(path, offset, limit),
                "shaderAssetLocalIdentifierInFile" => {
                    (self.shader_asset_local_identifier_in_file).inspect_value(path, offset, limit)
                }
                "subShaderIndex" => (self.sub_shader_index).inspect_value(path, offset, limit),
                "passIndex" => (self.pass_index).inspect_value(path, offset, limit),
                "graphicsStateInfoSet" => {
                    (self.graphics_state_info_set).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.keywords).inspect_value(path, offset, limit))
                .map(|v| ("keywords".to_string(), v)),
            ((self.shader).inspect_value(path, offset, limit)).map(|v| ("shader".to_string(), v)),
            ((self.pass_type).inspect_value(path, offset, limit))
                .map(|v| ("passType".to_string(), v)),
            ((self.shader_name).inspect_value(path, offset, limit))
                .map(|v| ("shaderName".to_string(), v)),
            ((self.shader_asset_guid).inspect_value(path, offset, limit))
                .map(|v| ("shaderAssetGUID".to_string(), v)),
            ((self.keyword_names).inspect_value(path, offset, limit))
                .map(|v| ("keywordNames".to_string(), v)),
            ((self.shader_asset_local_identifier_in_file).inspect_value(path, offset, limit))
                .map(|v| ("shaderAssetLocalIdentifierInFile".to_string(), v)),
            ((self.sub_shader_index).inspect_value(path, offset, limit))
                .map(|v| ("subShaderIndex".to_string(), v)),
            ((self.pass_index).inspect_value(path, offset, limit))
                .map(|v| ("passIndex".to_string(), v)),
            ((self.graphics_state_info_set).inspect_value(path, offset, limit))
                .map(|v| ("graphicsStateInfoSet".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
