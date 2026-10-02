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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralMaterialMShaderKeywordsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ProceduralMaterialMEnableInstancingVariantsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_2(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralMaterialMTexturesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubstanceInput::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubstanceInput::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralMaterialMInputsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralMaterialDisabledShaderPassesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            m_valid_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_invalid_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants = Some(
                ProceduralMaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?,
            );
            m_double_sided_gi = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            disabled_shader_passes = Some(ProceduralMaterialDisabledShaderPassesVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants = Some(
                ProceduralMaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?,
            );
            m_double_sided_gi = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            disabled_shader_passes = Some(ProceduralMaterialDisabledShaderPassesVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants = Some(
                ProceduralMaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?,
            );
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            disabled_shader_passes = Some(ProceduralMaterialDisabledShaderPassesVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_shader_keywords = Some(ProceduralMaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_custom_render_queue = Some(rdr.read_i32()?);
            string_tag_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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
            m_build_texture_stacks = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| BuildTextureStackReference::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for ProceduralMaterial {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Shader".to_string(), self.m_shader.to_value())),
            Some((
                "m_SavedProperties".to_string(),
                self.m_saved_properties.to_value(),
            )),
            crate::opt_field("m_ShaderKeywords", &self.m_shader_keywords),
            crate::opt_field("m_ValidKeywords", &self.m_valid_keywords),
            crate::opt_field("m_SubstancePackage", &self.m_substance_package),
            crate::opt_field("m_Width", &self.m_width),
            crate::opt_field("m_CustomRenderQueue", &self.m_custom_render_queue),
            crate::opt_field("m_LightmapFlags", &self.m_lightmap_flags),
            crate::opt_field("m_InvalidKeywords", &self.m_invalid_keywords),
            crate::opt_field("m_MaximumSize", &self.m_maximum_size),
            crate::opt_field("m_Height", &self.m_height),
            crate::opt_field(
                "m_EnableInstancingVariants",
                &self.m_enable_instancing_variants,
            ),
            crate::opt_field("m_Textures", &self.m_textures),
            crate::opt_field("stringTagMap", &self.string_tag_map),
            crate::opt_field("m_DoubleSidedGI", &self.m_double_sided_gi),
            crate::opt_field("m_Inputs", &self.m_inputs),
            crate::opt_field("m_Flags", &self.m_flags),
            crate::opt_field("m_LoadingBehavior", &self.m_loading_behavior),
            crate::opt_field("m_GenerateMipmaps", &self.m_generate_mipmaps),
            crate::opt_field("disabledShaderPasses", &self.disabled_shader_passes),
            crate::opt_field("m_CacheSize", &self.m_cache_size),
            crate::opt_field("m_AnimationUpdateRate", &self.m_animation_update_rate),
            crate::opt_field("m_BuildTextureStacks", &self.m_build_texture_stacks),
            crate::opt_field("m_PrototypeName", &self.m_prototype_name),
            crate::opt_field("m_Hash", &self.m_hash),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Shader" => (self.m_shader).inspect_value(path, offset, limit),
                "m_SavedProperties" => (self.m_saved_properties).inspect_value(path, offset, limit),
                "m_ShaderKeywords" => (self.m_shader_keywords).inspect_value(path, offset, limit),
                "m_ValidKeywords" => (self.m_valid_keywords).inspect_value(path, offset, limit),
                "m_SubstancePackage" => {
                    (self.m_substance_package).inspect_value(path, offset, limit)
                }
                "m_Width" => (self.m_width).inspect_value(path, offset, limit),
                "m_CustomRenderQueue" => {
                    (self.m_custom_render_queue).inspect_value(path, offset, limit)
                }
                "m_LightmapFlags" => (self.m_lightmap_flags).inspect_value(path, offset, limit),
                "m_InvalidKeywords" => (self.m_invalid_keywords).inspect_value(path, offset, limit),
                "m_MaximumSize" => (self.m_maximum_size).inspect_value(path, offset, limit),
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_EnableInstancingVariants" => {
                    (self.m_enable_instancing_variants).inspect_value(path, offset, limit)
                }
                "m_Textures" => (self.m_textures).inspect_value(path, offset, limit),
                "stringTagMap" => (self.string_tag_map).inspect_value(path, offset, limit),
                "m_DoubleSidedGI" => (self.m_double_sided_gi).inspect_value(path, offset, limit),
                "m_Inputs" => (self.m_inputs).inspect_value(path, offset, limit),
                "m_Flags" => (self.m_flags).inspect_value(path, offset, limit),
                "m_LoadingBehavior" => (self.m_loading_behavior).inspect_value(path, offset, limit),
                "m_GenerateMipmaps" => (self.m_generate_mipmaps).inspect_value(path, offset, limit),
                "disabledShaderPasses" => {
                    (self.disabled_shader_passes).inspect_value(path, offset, limit)
                }
                "m_CacheSize" => (self.m_cache_size).inspect_value(path, offset, limit),
                "m_AnimationUpdateRate" => {
                    (self.m_animation_update_rate).inspect_value(path, offset, limit)
                }
                "m_BuildTextureStacks" => {
                    (self.m_build_texture_stacks).inspect_value(path, offset, limit)
                }
                "m_PrototypeName" => (self.m_prototype_name).inspect_value(path, offset, limit),
                "m_Hash" => (self.m_hash).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_Shader".to_string(), v)),
            ((self.m_saved_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_SavedProperties".to_string(), v)),
            ((self.m_shader_keywords).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderKeywords".to_string(), v)),
            ((self.m_valid_keywords).inspect_value(path, offset, limit))
                .map(|v| ("m_ValidKeywords".to_string(), v)),
            ((self.m_substance_package).inspect_value(path, offset, limit))
                .map(|v| ("m_SubstancePackage".to_string(), v)),
            ((self.m_width).inspect_value(path, offset, limit)).map(|v| ("m_Width".to_string(), v)),
            ((self.m_custom_render_queue).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomRenderQueue".to_string(), v)),
            ((self.m_lightmap_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapFlags".to_string(), v)),
            ((self.m_invalid_keywords).inspect_value(path, offset, limit))
                .map(|v| ("m_InvalidKeywords".to_string(), v)),
            ((self.m_maximum_size).inspect_value(path, offset, limit))
                .map(|v| ("m_MaximumSize".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_enable_instancing_variants).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableInstancingVariants".to_string(), v)),
            ((self.m_textures).inspect_value(path, offset, limit))
                .map(|v| ("m_Textures".to_string(), v)),
            ((self.string_tag_map).inspect_value(path, offset, limit))
                .map(|v| ("stringTagMap".to_string(), v)),
            ((self.m_double_sided_gi).inspect_value(path, offset, limit))
                .map(|v| ("m_DoubleSidedGI".to_string(), v)),
            ((self.m_inputs).inspect_value(path, offset, limit))
                .map(|v| ("m_Inputs".to_string(), v)),
            ((self.m_flags).inspect_value(path, offset, limit)).map(|v| ("m_Flags".to_string(), v)),
            ((self.m_loading_behavior).inspect_value(path, offset, limit))
                .map(|v| ("m_LoadingBehavior".to_string(), v)),
            ((self.m_generate_mipmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateMipmaps".to_string(), v)),
            ((self.disabled_shader_passes).inspect_value(path, offset, limit))
                .map(|v| ("disabledShaderPasses".to_string(), v)),
            ((self.m_cache_size).inspect_value(path, offset, limit))
                .map(|v| ("m_CacheSize".to_string(), v)),
            ((self.m_animation_update_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationUpdateRate".to_string(), v)),
            ((self.m_build_texture_stacks).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTextureStacks".to_string(), v)),
            ((self.m_prototype_name).inspect_value(path, offset, limit))
                .map(|v| ("m_PrototypeName".to_string(), v)),
            ((self.m_hash).inspect_value(path, offset, limit)).map(|v| ("m_Hash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
