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

impl crate::ToValue for SerializedPassMHasInstancingVariantVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_1(v) => (v).inspect_value(path, offset, limit),
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
            m_editor_data_hash = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Hash128::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_platforms = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_editor_data_hash = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Hash128::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_platforms = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_local_keyword_mask = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_global_keyword_mask = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_name_indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_i32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_type = Some(rdr.read_i32()?);
        m_state = Some(SerializedShaderState::parse(rdr, ver)?);
        m_program_mask = Some(rdr.read_u32()?);
        prog_vertex = Some(SerializedProgram::parse(rdr, ver)?);
        prog_fragment = Some(SerializedProgram::parse(rdr, ver)?);
        prog_geometry = Some(SerializedProgram::parse(rdr, ver)?);
        prog_hull = Some(SerializedProgram::parse(rdr, ver)?);
        prog_domain = Some(SerializedProgram::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            prog_ray_tracing = Some(SerializedProgram::parse(rdr, ver)?);
        } else {
        }
        m_has_instancing_variant =
            Some(SerializedPassMHasInstancingVariantVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_has_procedural_instancing_variant = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_use_name = Some(rdr.read_aligned_string()?);
        m_name = Some(rdr.read_aligned_string()?);
        m_texture_name = Some(rdr.read_aligned_string()?);
        m_tags = Some(SerializedTagMap::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_serialized_keyword_state_mask = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for SerializedPass {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NameIndices".to_string(), self.m_name_indices.to_value())),
            crate::opt_field("m_EditorDataHash", &self.m_editor_data_hash),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            crate::opt_field("m_Platforms", &self.m_platforms),
            Some(("m_State".to_string(), self.m_state.to_value())),
            crate::opt_field("m_LocalKeywordMask", &self.m_local_keyword_mask),
            Some(("m_ProgramMask".to_string(), self.m_program_mask.to_value())),
            crate::opt_field("m_GlobalKeywordMask", &self.m_global_keyword_mask),
            Some(("progVertex".to_string(), self.prog_vertex.to_value())),
            Some(("progFragment".to_string(), self.prog_fragment.to_value())),
            Some(("progGeometry".to_string(), self.prog_geometry.to_value())),
            Some(("progHull".to_string(), self.prog_hull.to_value())),
            Some(("progDomain".to_string(), self.prog_domain.to_value())),
            Some((
                "m_HasInstancingVariant".to_string(),
                self.m_has_instancing_variant.to_value(),
            )),
            crate::opt_field("progRayTracing", &self.prog_ray_tracing),
            Some(("m_UseName".to_string(), self.m_use_name.to_value())),
            crate::opt_field(
                "m_HasProceduralInstancingVariant",
                &self.m_has_procedural_instancing_variant,
            ),
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_TextureName".to_string(), self.m_texture_name.to_value())),
            Some(("m_Tags".to_string(), self.m_tags.to_value())),
            crate::opt_field(
                "m_SerializedKeywordStateMask",
                &self.m_serialized_keyword_state_mask,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NameIndices" => (self.m_name_indices).inspect_value(path, offset, limit),
                "m_EditorDataHash" => (self.m_editor_data_hash).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Platforms" => (self.m_platforms).inspect_value(path, offset, limit),
                "m_State" => (self.m_state).inspect_value(path, offset, limit),
                "m_LocalKeywordMask" => {
                    (self.m_local_keyword_mask).inspect_value(path, offset, limit)
                }
                "m_ProgramMask" => (self.m_program_mask).inspect_value(path, offset, limit),
                "m_GlobalKeywordMask" => {
                    (self.m_global_keyword_mask).inspect_value(path, offset, limit)
                }
                "progVertex" => (self.prog_vertex).inspect_value(path, offset, limit),
                "progFragment" => (self.prog_fragment).inspect_value(path, offset, limit),
                "progGeometry" => (self.prog_geometry).inspect_value(path, offset, limit),
                "progHull" => (self.prog_hull).inspect_value(path, offset, limit),
                "progDomain" => (self.prog_domain).inspect_value(path, offset, limit),
                "m_HasInstancingVariant" => {
                    (self.m_has_instancing_variant).inspect_value(path, offset, limit)
                }
                "progRayTracing" => (self.prog_ray_tracing).inspect_value(path, offset, limit),
                "m_UseName" => (self.m_use_name).inspect_value(path, offset, limit),
                "m_HasProceduralInstancingVariant" => {
                    (self.m_has_procedural_instancing_variant).inspect_value(path, offset, limit)
                }
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_TextureName" => (self.m_texture_name).inspect_value(path, offset, limit),
                "m_Tags" => (self.m_tags).inspect_value(path, offset, limit),
                "m_SerializedKeywordStateMask" => {
                    (self.m_serialized_keyword_state_mask).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_NameIndices".to_string(), v)),
            ((self.m_editor_data_hash).inspect_value(path, offset, limit))
                .map(|v| ("m_EditorDataHash".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_platforms).inspect_value(path, offset, limit))
                .map(|v| ("m_Platforms".to_string(), v)),
            ((self.m_state).inspect_value(path, offset, limit)).map(|v| ("m_State".to_string(), v)),
            ((self.m_local_keyword_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalKeywordMask".to_string(), v)),
            ((self.m_program_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_ProgramMask".to_string(), v)),
            ((self.m_global_keyword_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_GlobalKeywordMask".to_string(), v)),
            ((self.prog_vertex).inspect_value(path, offset, limit))
                .map(|v| ("progVertex".to_string(), v)),
            ((self.prog_fragment).inspect_value(path, offset, limit))
                .map(|v| ("progFragment".to_string(), v)),
            ((self.prog_geometry).inspect_value(path, offset, limit))
                .map(|v| ("progGeometry".to_string(), v)),
            ((self.prog_hull).inspect_value(path, offset, limit))
                .map(|v| ("progHull".to_string(), v)),
            ((self.prog_domain).inspect_value(path, offset, limit))
                .map(|v| ("progDomain".to_string(), v)),
            ((self.m_has_instancing_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_HasInstancingVariant".to_string(), v)),
            ((self.prog_ray_tracing).inspect_value(path, offset, limit))
                .map(|v| ("progRayTracing".to_string(), v)),
            ((self.m_use_name).inspect_value(path, offset, limit))
                .map(|v| ("m_UseName".to_string(), v)),
            ((self.m_has_procedural_instancing_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_HasProceduralInstancingVariant".to_string(), v)),
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_texture_name).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureName".to_string(), v)),
            ((self.m_tags).inspect_value(path, offset, limit)).map(|v| ("m_Tags".to_string(), v)),
            ((self.m_serialized_keyword_state_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_SerializedKeywordStateMask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
