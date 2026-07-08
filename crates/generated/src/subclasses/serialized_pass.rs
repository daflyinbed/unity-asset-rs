//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;
use crate::subclasses::serialized_program::SerializedProgram;
use crate::subclasses::serialized_shader_state::SerializedShaderState;
use crate::subclasses::serialized_tag_map::SerializedTagMap;


/// 版本枚举：SerializedPassMHasInstancingVariantVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedPassMHasInstancingVariantVariant {
    Default(bool),
    V2018_1(bool),
}

impl crate::Parse for SerializedPassMHasInstancingVariantVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedPass {
    pub m_name_indices: Vec<(String, i32)>,
    pub m_editor_data_hash: Option<Vec<Hash128>>,
    pub m_type: i32,
    pub m_platforms: Option<Vec<u8>>,
    pub m_state: SerializedShaderState,
    pub m_local_keyword_mask: Option<Vec<u16>>,
    pub m_program_mask: u32,
    pub m_global_keyword_mask: Option<Vec<u16>>,
    pub prog_vertex: SerializedProgram,
    pub prog_fragment: SerializedProgram,
    pub prog_geometry: SerializedProgram,
    pub prog_hull: SerializedProgram,
    pub prog_domain: SerializedProgram,
    pub m_has_instancing_variant: SerializedPassMHasInstancingVariantVariant,
    pub prog_ray_tracing: Option<SerializedProgram>,
    pub m_use_name: String,
    pub m_has_procedural_instancing_variant: Option<bool>,
    pub m_name: String,
    pub m_texture_name: String,
    pub m_tags: SerializedTagMap,
    pub m_serialized_keyword_state_mask: Option<Vec<u16>>,
}

impl crate::Parse for SerializedPass {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_indices = None;
        let mut m_editor_data_hash = None;
        let mut m_type = None;
        let mut m_platforms = None;
        let mut m_state = None;
        let mut m_local_keyword_mask = None;
        let mut m_program_mask = None;
        let mut m_global_keyword_mask = None;
        let mut prog_vertex = None;
        let mut prog_fragment = None;
        let mut prog_geometry = None;
        let mut prog_hull = None;
        let mut prog_domain = None;
        let mut m_has_instancing_variant = None;
        let mut prog_ray_tracing = None;
        let mut m_use_name = None;
        let mut m_has_procedural_instancing_variant = None;
        let mut m_name = None;
        let mut m_texture_name = None;
        let mut m_tags = None;
        let mut m_serialized_keyword_state_mask = None;
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_editor_data_hash = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Hash128::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_platforms = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_editor_data_hash = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Hash128::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_platforms = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_local_keyword_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_global_keyword_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_name_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_type = Some(rdr.read_i32()?);
            m_state = Some(SerializedShaderState::parse(rdr, ver)?);
            rdr.align(4)?;
            m_program_mask = Some(rdr.read_u32()?);
            prog_vertex = Some(SerializedProgram::parse(rdr, ver)?);
            rdr.align(4)?;
            prog_fragment = Some(SerializedProgram::parse(rdr, ver)?);
            rdr.align(4)?;
            prog_geometry = Some(SerializedProgram::parse(rdr, ver)?);
            rdr.align(4)?;
            prog_hull = Some(SerializedProgram::parse(rdr, ver)?);
            rdr.align(4)?;
            prog_domain = Some(SerializedProgram::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            prog_ray_tracing = Some(SerializedProgram::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
        }
            m_has_instancing_variant = Some(SerializedPassMHasInstancingVariantVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_has_procedural_instancing_variant = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_use_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_texture_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_tags = Some(SerializedTagMap::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_serialized_keyword_state_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }

        Ok(SerializedPass {
            m_name_indices: m_name_indices.unwrap(),
            m_editor_data_hash,
            m_type: m_type.unwrap(),
            m_platforms,
            m_state: m_state.unwrap(),
            m_local_keyword_mask,
            m_program_mask: m_program_mask.unwrap(),
            m_global_keyword_mask,
            prog_vertex: prog_vertex.unwrap(),
            prog_fragment: prog_fragment.unwrap(),
            prog_geometry: prog_geometry.unwrap(),
            prog_hull: prog_hull.unwrap(),
            prog_domain: prog_domain.unwrap(),
            m_has_instancing_variant: m_has_instancing_variant.unwrap(),
            prog_ray_tracing,
            m_use_name: m_use_name.unwrap(),
            m_has_procedural_instancing_variant,
            m_name: m_name.unwrap(),
            m_texture_name: m_texture_name.unwrap(),
            m_tags: m_tags.unwrap(),
            m_serialized_keyword_state_mask,
        })
    }
}
