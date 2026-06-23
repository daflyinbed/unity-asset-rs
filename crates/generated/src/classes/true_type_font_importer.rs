//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::output::Output;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct TrueTypeFontImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_font_size: i32,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_force_texture_case: i32,
    pub m_font_color: Option<ColorRgba>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_character_spacing: Option<i32>,
    pub m_render_mode: Option<i32>,
    pub m_character_padding: Option<i32>,
    pub m_style: Option<i32>,
    pub m_include_font_data: bool,
    pub m_use2x_behaviour: Option<bool>,
    pub m_font_names: Vec<String>,
    pub m_font_name: Option<String>,
    pub m_fallback_font_references: Option<Vec<PPtr>>,
    pub m_custom_characters: Option<String>,
    pub m_font_rendering_mode: Option<i32>,
    pub m_output: Option<Output>,
    pub m_user_data: Option<String>,
    pub m_ascent_calculation_mode: Option<i32>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_use_legacy_bounds_calculation: Option<bool>,
    pub m_should_round_advance_value: Option<bool>,
}

impl crate::Parse for TrueTypeFontImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_font_size = r.read_i32()?;
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_old_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_force_texture_case = r.read_i32()?;
        let m_font_color = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_new_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_character_spacing = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_render_mode = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_character_padding = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_style = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_include_font_data = r.read_bool()?;
        let m_use2x_behaviour = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_font_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_font_name = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_fallback_font_references = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_custom_characters = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_font_rendering_mode = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_output = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = Output::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_user_data = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ascent_calculation_mode = if *ver >= crate::UnityVersion::new(5, 3, 4) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_legacy_bounds_calculation = if *ver >= crate::UnityVersion::new(5, 6, 5) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_should_round_advance_value = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TrueTypeFontImporter {
            parent,
            m_name,
            m_file_id_to_recycle_name,
            m_font_size,
            m_external_objects,
            m_old_hash_identity,
            m_force_texture_case,
            m_font_color,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_character_spacing,
            m_render_mode,
            m_character_padding,
            m_style,
            m_include_font_data,
            m_use2x_behaviour,
            m_font_names,
            m_font_name,
            m_fallback_font_references,
            m_custom_characters,
            m_font_rendering_mode,
            m_output,
            m_user_data,
            m_ascent_calculation_mode,
            m_asset_bundle_name,
            m_asset_bundle_variant,
            m_use_legacy_bounds_calculation,
            m_should_round_advance_value,
        })
    }
}
