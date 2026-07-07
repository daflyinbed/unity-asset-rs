//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_texture_stack_reference::BuildTextureStackReference;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;

#[derive(Debug, Clone)]
pub struct Material {
    pub m_name: String,
    pub m_shader: PPtr,
    pub m_saved_properties: UnityPropertySheet,
    pub m_shader_keywords: Option<String>,
    pub m_valid_keywords: Option<Vec<String>>,
    pub m_custom_render_queue: Option<i32>,
    pub m_lightmap_flags: Option<u32>,
    pub m_invalid_keywords: Option<Vec<String>>,
    pub m_enable_instancing_variants: Option<bool>,
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
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_shader = crate::PPtr::parse(r, ver)?;
        let m_saved_properties = UnityPropertySheet::parse(r, ver)?;
        r.align(4)?;
        let m_shader_keywords = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = r.read_aligned_string()?;
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
        let m_enable_instancing_variants = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
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
        let disabled_shader_passes = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
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

        Ok(Material {
            m_name,
            m_shader,
            m_saved_properties,
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
