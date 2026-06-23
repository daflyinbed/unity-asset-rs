//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_texture_stack_reference::BuildTextureStackReference;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::substance_input::SubstanceInput;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;
use super::material::Material;

#[derive(Debug, Clone)]
pub struct ProceduralMaterial {
    pub parent: Material,
    pub m_name: String,
    pub m_shader: PPtr,
    pub m_saved_properties: UnityPropertySheet,
    pub m_shader_keywords: Option<Vec<String>>,
    pub m_valid_keywords: Option<Vec<String>>,
    pub m_substance_package: Option<PPtr>,
    pub m_width: Option<i32>,
    pub m_custom_render_queue: Option<i32>,
    pub m_lightmap_flags: Option<u32>,
    pub m_invalid_keywords: Option<Vec<String>>,
    pub m_maximum_size: Option<i32>,
    pub m_height: Option<i32>,
    pub m_enable_instancing_variants: Option<bool>,
    pub m_textures: Option<Vec<PPtr>>,
    pub string_tag_map: Option<Vec<(String, String)>>,
    pub m_double_sided_gi: Option<bool>,
    pub m_inputs: Option<Vec<SubstanceInput>>,
    pub m_flags: Option<u32>,
    pub m_loading_behavior: Option<i32>,
    pub m_generate_mipmaps: Option<bool>,
    pub disabled_shader_passes: Option<Vec<String>>,
    pub m_cache_size: Option<i32>,
    pub m_animation_update_rate: Option<i32>,
    pub m_build_texture_stacks: Option<Vec<BuildTextureStackReference>>,
    pub m_prototype_name: Option<String>,
    pub m_hash: Option<Hash128>,
}

impl crate::Parse for ProceduralMaterial {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Material::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_shader = crate::PPtr::parse(r, ver)?;
        let m_saved_properties = UnityPropertySheet::parse(r, ver)?;
        r.align(4)?;
        let m_shader_keywords = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_valid_keywords = if *ver >= crate::UnityVersion::new(2021, 2, 18) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_substance_package = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_width = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_custom_render_queue = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmap_flags = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_invalid_keywords = if *ver >= crate::UnityVersion::new(2021, 2, 18) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_maximum_size = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_height = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_instancing_variants = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_textures = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let string_tag_map = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_double_sided_gi = if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_inputs = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubstanceInput::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_flags = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_loading_behavior = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_generate_mipmaps = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let disabled_shader_passes = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_size = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_animation_update_rate = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_build_texture_stacks = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTextureStackReference::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_prototype_name = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_hash = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(Hash128::parse(r, ver)?)
        } else {
            None
        };

        Ok(ProceduralMaterial {
            parent,
            m_name,
            m_shader,
            m_saved_properties,
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
