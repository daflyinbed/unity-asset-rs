//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::output::Output;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：TrueTypeFontImporterMIncludeFontDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrueTypeFontImporterMIncludeFontDataVariant {
    Default(bool),
    V5_3_4(bool),
}

impl crate::Parse for TrueTypeFontImporterMIncludeFontDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 4) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_3_4(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TrueTypeFontImporterMFallbackFontReferencesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrueTypeFontImporterMFallbackFontReferencesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for TrueTypeFontImporterMFallbackFontReferencesVariant {
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

/// 版本枚举：TrueTypeFontImporterMAscentCalculationModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrueTypeFontImporterMAscentCalculationModeVariant {
    Default(i32),
    V5_6_5(i32),
    V2017_1(i32),
    V2017_1_2(i32),
}

impl crate::Parse for TrueTypeFontImporterMAscentCalculationModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            let v = rdr.read_i32()?;
            Ok(Self::V2017_1_2(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6_5(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TrueTypeFontImporterMUseLegacyBoundsCalculationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrueTypeFontImporterMUseLegacyBoundsCalculationVariant {
    Default(bool),
    V2018_1(bool),
}

impl crate::Parse for TrueTypeFontImporterMUseLegacyBoundsCalculationVariant {
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
pub struct TrueTypeFontImporter {
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
    pub m_include_font_data: TrueTypeFontImporterMIncludeFontDataVariant,
    pub m_use2x_behaviour: Option<bool>,
    pub m_font_names: Vec<String>,
    pub m_font_name: Option<String>,
    pub m_fallback_font_references: Option<TrueTypeFontImporterMFallbackFontReferencesVariant>,
    pub m_custom_characters: Option<String>,
    pub m_font_rendering_mode: Option<i32>,
    pub m_output: Option<Output>,
    pub m_user_data: Option<String>,
    pub m_ascent_calculation_mode: Option<TrueTypeFontImporterMAscentCalculationModeVariant>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_use_legacy_bounds_calculation: Option<TrueTypeFontImporterMUseLegacyBoundsCalculationVariant>,
    pub m_should_round_advance_value: Option<bool>,
}

impl crate::Parse for TrueTypeFontImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_font_size = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut m_force_texture_case = None;
        let mut m_font_color = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut m_character_spacing = None;
        let mut m_render_mode = None;
        let mut m_character_padding = None;
        let mut m_style = None;
        let mut m_include_font_data = None;
        let mut m_use2x_behaviour = None;
        let mut m_font_names = None;
        let mut m_font_name = None;
        let mut m_fallback_font_references = None;
        let mut m_custom_characters = None;
        let mut m_font_rendering_mode = None;
        let mut m_output = None;
        let mut m_user_data = None;
        let mut m_ascent_calculation_mode = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
        let mut m_use_legacy_bounds_calculation = None;
        let mut m_should_round_advance_value = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            rdr.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            rdr.align(4)?;
        }
            m_font_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_font_color = Some(ColorRgba::parse(rdr, ver)?);
        } else {
        }
            m_force_texture_case = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_character_spacing = Some(rdr.read_i32()?);
            m_character_padding = Some(rdr.read_i32()?);
        } else {
            m_render_mode = Some(rdr.read_i32()?);
            m_style = Some(rdr.read_i32()?);
        }
            m_include_font_data = Some(TrueTypeFontImporterMIncludeFontDataVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_font_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
        } else {
            m_use2x_behaviour = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
            m_font_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_fallback_font_references = Some(TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?);
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?);
            m_use_legacy_bounds_calculation = Some(TrueTypeFontImporterMUseLegacyBoundsCalculationVariant::parse(rdr, ver)?);
            m_should_round_advance_value = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            m_fallback_font_references = Some(TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?);
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?);
            m_use_legacy_bounds_calculation = Some(TrueTypeFontImporterMUseLegacyBoundsCalculationVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_fallback_font_references = Some(TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?);
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            m_fallback_font_references = Some(TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?);
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?);
            m_use_legacy_bounds_calculation = Some(TrueTypeFontImporterMUseLegacyBoundsCalculationVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
            m_fallback_font_references = Some(TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?);
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_fallback_font_references = Some(TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?);
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_custom_characters = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }

        Ok(TrueTypeFontImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_font_size: m_font_size.unwrap(),
            m_external_objects,
            m_old_hash_identity,
            m_force_texture_case: m_force_texture_case.unwrap(),
            m_font_color,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_character_spacing,
            m_render_mode,
            m_character_padding,
            m_style,
            m_include_font_data: m_include_font_data.unwrap(),
            m_use2x_behaviour,
            m_font_names: m_font_names.unwrap(),
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
