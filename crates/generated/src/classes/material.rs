//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_texture_stack_reference::BuildTextureStackReference;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;

/// 版本枚举：MaterialMShaderKeywordsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialMShaderKeywordsVariant {
    Default(Vec<String>),
    V5_0(String),
}

impl crate::Parse for MaterialMShaderKeywordsVariant {
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

impl crate::ToValue for MaterialMShaderKeywordsVariant {
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

/// 版本枚举：MaterialMEnableInstancingVariantsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialMEnableInstancingVariantsVariant {
    Default(bool),
    V5_6_2(bool),
}

impl crate::Parse for MaterialMEnableInstancingVariantsVariant {
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

impl crate::ToValue for MaterialMEnableInstancingVariantsVariant {
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

/// 版本枚举：MaterialDisabledShaderPassesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialDisabledShaderPassesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for MaterialDisabledShaderPassesVariant {
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

impl crate::ToValue for MaterialDisabledShaderPassesVariant {
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
pub struct Material {
    pub m_name: String,
    pub m_shader: PPtr,
    pub m_saved_properties: UnityPropertySheet,
    pub m_shader_keywords: Option<MaterialMShaderKeywordsVariant>,
    pub m_valid_keywords: Option<Vec<String>>,
    pub m_custom_render_queue: Option<i32>,
    pub m_lightmap_flags: Option<u32>,
    pub m_invalid_keywords: Option<Vec<String>>,
    pub m_enable_instancing_variants: Option<MaterialMEnableInstancingVariantsVariant>,
    pub string_tag_map: Option<Vec<(String, String)>>,
    pub m_double_sided_gi: Option<bool>,
    pub disabled_shader_passes: Option<MaterialDisabledShaderPassesVariant>,
    pub m_build_texture_stacks: Option<Vec<BuildTextureStackReference>>,
}

impl crate::Parse for Material {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_shader = None;
        let mut m_saved_properties = None;
        let mut m_shader_keywords = None;
        let mut m_valid_keywords = None;
        let mut m_custom_render_queue = None;
        let mut m_lightmap_flags = None;
        let mut m_invalid_keywords = None;
        let mut m_enable_instancing_variants = None;
        let mut string_tag_map = None;
        let mut m_double_sided_gi = None;
        let mut disabled_shader_passes = None;
        let mut m_build_texture_stacks = None;
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
            m_enable_instancing_variants =
                Some(MaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?);
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
            disabled_shader_passes = Some(MaterialDisabledShaderPassesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants =
                Some(MaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?);
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
            disabled_shader_passes = Some(MaterialDisabledShaderPassesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_enable_instancing_variants =
                Some(MaterialMEnableInstancingVariantsVariant::parse(rdr, ver)?);
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
            disabled_shader_passes = Some(MaterialDisabledShaderPassesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(rdr, ver)?);
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
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_lightmap_flags = Some(rdr.read_u32()?);
            m_custom_render_queue = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(rdr, ver)?);
            m_custom_render_queue = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(rdr, ver)?);
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
        } else {
        }

        Ok(Material {
            m_name: m_name.unwrap(),
            m_shader: m_shader.unwrap(),
            m_saved_properties: m_saved_properties.unwrap(),
            m_shader_keywords,
            m_valid_keywords,
            m_custom_render_queue,
            m_lightmap_flags,
            m_invalid_keywords,
            m_enable_instancing_variants,
            string_tag_map,
            m_double_sided_gi,
            disabled_shader_passes,
            m_build_texture_stacks,
        })
    }
}

impl crate::ToValue for Material {
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
            crate::opt_field("m_CustomRenderQueue", &self.m_custom_render_queue),
            crate::opt_field("m_LightmapFlags", &self.m_lightmap_flags),
            crate::opt_field("m_InvalidKeywords", &self.m_invalid_keywords),
            crate::opt_field(
                "m_EnableInstancingVariants",
                &self.m_enable_instancing_variants,
            ),
            crate::opt_field("stringTagMap", &self.string_tag_map),
            crate::opt_field("m_DoubleSidedGI", &self.m_double_sided_gi),
            crate::opt_field("disabledShaderPasses", &self.disabled_shader_passes),
            crate::opt_field("m_BuildTextureStacks", &self.m_build_texture_stacks),
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
                "m_CustomRenderQueue" => {
                    (self.m_custom_render_queue).inspect_value(path, offset, limit)
                }
                "m_LightmapFlags" => (self.m_lightmap_flags).inspect_value(path, offset, limit),
                "m_InvalidKeywords" => (self.m_invalid_keywords).inspect_value(path, offset, limit),
                "m_EnableInstancingVariants" => {
                    (self.m_enable_instancing_variants).inspect_value(path, offset, limit)
                }
                "stringTagMap" => (self.string_tag_map).inspect_value(path, offset, limit),
                "m_DoubleSidedGI" => (self.m_double_sided_gi).inspect_value(path, offset, limit),
                "disabledShaderPasses" => {
                    (self.disabled_shader_passes).inspect_value(path, offset, limit)
                }
                "m_BuildTextureStacks" => {
                    (self.m_build_texture_stacks).inspect_value(path, offset, limit)
                }
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
            ((self.m_custom_render_queue).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomRenderQueue".to_string(), v)),
            ((self.m_lightmap_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapFlags".to_string(), v)),
            ((self.m_invalid_keywords).inspect_value(path, offset, limit))
                .map(|v| ("m_InvalidKeywords".to_string(), v)),
            ((self.m_enable_instancing_variants).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableInstancingVariants".to_string(), v)),
            ((self.string_tag_map).inspect_value(path, offset, limit))
                .map(|v| ("stringTagMap".to_string(), v)),
            ((self.m_double_sided_gi).inspect_value(path, offset, limit))
                .map(|v| ("m_DoubleSidedGI".to_string(), v)),
            ((self.disabled_shader_passes).inspect_value(path, offset, limit))
                .map(|v| ("disabledShaderPasses".to_string(), v)),
            ((self.m_build_texture_stacks).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTextureStacks".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
