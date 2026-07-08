//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_texture_stack_reference::BuildTextureStackReference;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::substance_input::SubstanceInput;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;


/// 版本枚举：ProceduralMaterialMShaderKeywordsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralMaterialMShaderKeywordsVariant {
    Default(Vec<String>),
    V5_0(String),
}

impl crate::Parse for ProceduralMaterialMShaderKeywordsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ProceduralMaterialMEnableInstancingVariantsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralMaterialMEnableInstancingVariantsVariant {
    Default(bool),
    V5_6_2(bool),
}

impl crate::Parse for ProceduralMaterialMEnableInstancingVariantsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ProceduralMaterialMTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralMaterialMTexturesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for ProceduralMaterialMTexturesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ProceduralMaterialMInputsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralMaterialMInputsVariant {
    Default(Vec<SubstanceInput>),
    V2017_1(Vec<SubstanceInput>),
}

impl crate::Parse for ProceduralMaterialMInputsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubstanceInput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubstanceInput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ProceduralMaterialDisabledShaderPassesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralMaterialDisabledShaderPassesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for ProceduralMaterialDisabledShaderPassesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProceduralMaterial {
    pub m_name: String,
    pub m_shader: PPtr,
    pub m_saved_properties: UnityPropertySheet,
    pub m_shader_keywords: Option<ProceduralMaterialMShaderKeywordsVariant>,
    pub m_valid_keywords: Option<Vec<String>>,
    pub m_substance_package: Option<PPtr>,
    pub m_width: Option<i32>,
    pub m_custom_render_queue: Option<i32>,
    pub m_lightmap_flags: Option<u32>,
    pub m_invalid_keywords: Option<Vec<String>>,
    pub m_maximum_size: Option<i32>,
    pub m_height: Option<i32>,
    pub m_enable_instancing_variants: Option<ProceduralMaterialMEnableInstancingVariantsVariant>,
    pub m_textures: Option<ProceduralMaterialMTexturesVariant>,
    pub string_tag_map: Option<Vec<(String, String)>>,
    pub m_double_sided_gi: Option<bool>,
    pub m_inputs: Option<ProceduralMaterialMInputsVariant>,
    pub m_flags: Option<u32>,
    pub m_loading_behavior: Option<i32>,
    pub m_generate_mipmaps: Option<bool>,
    pub disabled_shader_passes: Option<ProceduralMaterialDisabledShaderPassesVariant>,
    pub m_cache_size: Option<i32>,
    pub m_animation_update_rate: Option<i32>,
    pub m_build_texture_stacks: Option<Vec<BuildTextureStackReference>>,
    pub m_prototype_name: Option<String>,
    pub m_hash: Option<Hash128>,
}

impl crate::Parse for ProceduralMaterial {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_shader = None;
        let mut m_saved_properties = None;
        let mut m_shader_keywords = None;
        let mut m_valid_keywords = None;
        let mut m_substance_package = None;
        let mut m_width = None;
        let mut m_custom_render_queue = None;
        let mut m_lightmap_flags = None;
        let mut m_invalid_keywords = None;
        let mut m_maximum_size = None;
        let mut m_height = None;
        let mut m_enable_instancing_variants = None;
        let mut m_textures = None;
        let mut string_tag_map = None;
        let mut m_double_sided_gi = None;
        let mut m_inputs = None;
        let mut m_flags = None;
        let mut m_loading_behavior = None;
        let mut m_generate_mipmaps = None;
        let mut disabled_shader_passes = None;
        let mut m_cache_size = None;
        let mut m_animation_update_rate = None;
        let mut m_build_texture_stacks = None;
        let mut m_prototype_name = None;
        let mut m_hash = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_shader = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 18) {
            m_valid_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_invalid_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants = Some(ProceduralMaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?);
            m_double_sided_gi = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            disabled_shader_passes = Some(ProceduralMaterialDisabledShaderPassesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants = Some(ProceduralMaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?);
            m_double_sided_gi = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            disabled_shader_passes = Some(ProceduralMaterialDisabledShaderPassesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants = Some(ProceduralMaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?);
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            disabled_shader_passes = Some(ProceduralMaterialDisabledShaderPassesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_custom_render_queue = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_custom_render_queue = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
        } else {
        }
            m_saved_properties = Some(UnityPropertySheet::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_build_texture_stacks = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildTextureStackReference::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_generate_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_textures = Some(ProceduralMaterialMTexturesVariant::parse(rdr, ver)?);
            m_flags = Some(rdr.read_u32()?);
            m_loading_behavior = Some(rdr.read_i32()?);
            m_substance_package = Some(crate::PPtr::parse(rdr, ver)?);
            m_inputs = Some(ProceduralMaterialMInputsVariant::parse(rdr, ver)?);
            m_prototype_name = Some(rdr.read_aligned_string()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
            m_hash = Some(Hash128::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_textures = Some(ProceduralMaterialMTexturesVariant::parse(rdr, ver)?);
            m_flags = Some(rdr.read_u32()?);
            m_loading_behavior = Some(rdr.read_i32()?);
            m_substance_package = Some(crate::PPtr::parse(rdr, ver)?);
            m_inputs = Some(ProceduralMaterialMInputsVariant::parse(rdr, ver)?);
            m_prototype_name = Some(rdr.read_aligned_string()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
            m_hash = Some(Hash128::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_textures = Some(ProceduralMaterialMTexturesVariant::parse(rdr, ver)?);
            m_flags = Some(rdr.read_u32()?);
            m_loading_behavior = Some(rdr.read_i32()?);
            m_substance_package = Some(crate::PPtr::parse(rdr, ver)?);
            m_inputs = Some(ProceduralMaterialMInputsVariant::parse(rdr, ver)?);
            m_animation_update_rate = Some(rdr.read_i32()?);
            m_hash = Some(Hash128::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_textures = Some(ProceduralMaterialMTexturesVariant::parse(rdr, ver)?);
            m_flags = Some(rdr.read_u32()?);
            m_loading_behavior = Some(rdr.read_i32()?);
            m_substance_package = Some(crate::PPtr::parse(rdr, ver)?);
            m_inputs = Some(ProceduralMaterialMInputsVariant::parse(rdr, ver)?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_substance_package = Some(crate::PPtr::parse(rdr, ver)?);
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_textures = Some(ProceduralMaterialMTexturesVariant::parse(rdr, ver)?);
            m_inputs = Some(ProceduralMaterialMInputsVariant::parse(rdr, ver)?);
            m_flags = Some(rdr.read_u32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else {
            m_substance_package = Some(crate::PPtr::parse(rdr, ver)?);
            m_maximum_size = Some(rdr.read_i32()?);
            m_textures = Some(ProceduralMaterialMTexturesVariant::parse(rdr, ver)?);
            m_inputs = Some(ProceduralMaterialMInputsVariant::parse(rdr, ver)?);
            m_flags = Some(rdr.read_u32()?);
            m_cache_size = Some(rdr.read_i32()?);
        }

        Ok(ProceduralMaterial {
            m_name: m_name.unwrap(),
            m_shader: m_shader.unwrap(),
            m_saved_properties: m_saved_properties.unwrap(),
            m_shader_keywords,
            m_valid_keywords,
            m_substance_package,
            m_width,
            m_custom_render_queue,
            m_lightmap_flags,
            m_invalid_keywords,
            m_maximum_size,
            m_height,
            m_enable_instancing_variants,
            m_textures,
            string_tag_map,
            m_double_sided_gi,
            m_inputs,
            m_flags,
            m_loading_behavior,
            m_generate_mipmaps,
            disabled_shader_passes,
            m_cache_size,
            m_animation_update_rate,
            m_build_texture_stacks,
            m_prototype_name,
            m_hash,
        })
    }
}
