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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = r.read_bool()?;
            Ok(Self::V5_6_2(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
    pub disabled_shader_passes: Option<Vec<String>>,
    pub m_build_texture_stacks: Option<Vec<BuildTextureStackReference>>,
}

impl crate::Parse for Material {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
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
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_shader = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 18) {
            m_valid_keywords = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_invalid_keywords = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_lightmap_flags = Some(r.read_u32()?);
            m_enable_instancing_variants = Some(MaterialMEnableInstancingVariantsVariant::parse(r, ver)?);
            m_double_sided_gi = Some(r.read_bool()?);
            r.align(4)?;
            m_custom_render_queue = Some(r.read_i32()?);
            string_tag_map = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            disabled_shader_passes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(r, ver)?);
            m_lightmap_flags = Some(r.read_u32()?);
            m_enable_instancing_variants = Some(MaterialMEnableInstancingVariantsVariant::parse(r, ver)?);
            m_double_sided_gi = Some(r.read_bool()?);
            r.align(4)?;
            m_custom_render_queue = Some(r.read_i32()?);
            string_tag_map = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            disabled_shader_passes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(r, ver)?);
            m_lightmap_flags = Some(r.read_u32()?);
            m_enable_instancing_variants = Some(MaterialMEnableInstancingVariantsVariant::parse(r, ver)?);
            m_custom_render_queue = Some(r.read_i32()?);
            string_tag_map = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            disabled_shader_passes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(r, ver)?);
            m_lightmap_flags = Some(r.read_u32()?);
            m_custom_render_queue = Some(r.read_i32()?);
            string_tag_map = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(r, ver)?);
            m_lightmap_flags = Some(r.read_u32()?);
            m_custom_render_queue = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(r, ver)?);
            m_custom_render_queue = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_shader_keywords = Some(MaterialMShaderKeywordsVariant::parse(r, ver)?);
        } else {
        }
            m_saved_properties = Some(UnityPropertySheet::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_build_texture_stacks = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTextureStackReference::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
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
