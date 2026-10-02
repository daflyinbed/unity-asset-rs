//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::output::Output;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：TrueTypeFontImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrueTypeFontImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for TrueTypeFontImporterMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TrueTypeFontImporterMExternalObjectsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

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

impl crate::ToValue for TrueTypeFontImporterMIncludeFontDataVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：TrueTypeFontImporterMFontNamesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrueTypeFontImporterMFontNamesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for TrueTypeFontImporterMFontNamesVariant {
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

impl crate::ToValue for TrueTypeFontImporterMFontNamesVariant {
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

impl crate::ToValue for TrueTypeFontImporterMFallbackFontReferencesVariant {
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

impl crate::ToValue for TrueTypeFontImporterMAscentCalculationModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_5(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2017_1_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_5(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for TrueTypeFontImporterMUseLegacyBoundsCalculationVariant {
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
pub struct TrueTypeFontImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_font_size: i32,
    pub m_external_objects: Option<TrueTypeFontImporterMExternalObjectsVariant>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_force_texture_case: i32,
    pub m_font_color: Option<ColorRgbAf>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_character_spacing: Option<i32>,
    pub m_render_mode: Option<i32>,
    pub m_character_padding: Option<i32>,
    pub m_style: Option<i32>,
    pub m_include_font_data: TrueTypeFontImporterMIncludeFontDataVariant,
    pub m_use2x_behaviour: Option<bool>,
    pub m_font_names: TrueTypeFontImporterMFontNamesVariant,
    pub m_font_name: Option<String>,
    pub m_fallback_font_references: Option<TrueTypeFontImporterMFallbackFontReferencesVariant>,
    pub m_custom_characters: Option<String>,
    pub m_font_rendering_mode: Option<i32>,
    pub m_output: Option<Output>,
    pub m_user_data: Option<String>,
    pub m_ascent_calculation_mode: Option<TrueTypeFontImporterMAscentCalculationModeVariant>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_use_legacy_bounds_calculation:
        Option<TrueTypeFontImporterMUseLegacyBoundsCalculationVariant>,
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
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(TrueTypeFontImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(TrueTypeFontImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
        }
        m_font_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_font_color = Some(ColorRgbAf::parse(rdr, ver)?);
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
        m_include_font_data = Some(TrueTypeFontImporterMIncludeFontDataVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_font_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
        } else {
            m_use2x_behaviour = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_font_names = Some(TrueTypeFontImporterMFontNamesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_fallback_font_references = Some(
                TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?,
            );
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(
                TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?,
            );
            m_use_legacy_bounds_calculation =
                Some(TrueTypeFontImporterMUseLegacyBoundsCalculationVariant::parse(rdr, ver)?);
            m_should_round_advance_value = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            m_fallback_font_references = Some(
                TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?,
            );
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(
                TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?,
            );
            m_use_legacy_bounds_calculation =
                Some(TrueTypeFontImporterMUseLegacyBoundsCalculationVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_fallback_font_references = Some(
                TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?,
            );
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(
                TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?,
            );
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            m_fallback_font_references = Some(
                TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?,
            );
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(
                TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?,
            );
            m_use_legacy_bounds_calculation =
                Some(TrueTypeFontImporterMUseLegacyBoundsCalculationVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
            m_fallback_font_references = Some(
                TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?,
            );
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_ascent_calculation_mode = Some(
                TrueTypeFontImporterMAscentCalculationModeVariant::parse(rdr, ver)?,
            );
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_fallback_font_references = Some(
                TrueTypeFontImporterMFallbackFontReferencesVariant::parse(rdr, ver)?,
            );
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_custom_characters = Some(rdr.read_aligned_string()?);
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_output = Some(Output::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_custom_characters = Some(rdr.read_aligned_string()?);
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

impl crate::ToValue for TrueTypeFontImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_FileIDToRecycleName", &self.m_file_id_to_recycle_name),
            Some(("m_FontSize".to_string(), self.m_font_size.to_value())),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_OldHashIdentity", &self.m_old_hash_identity),
            Some((
                "m_ForceTextureCase".to_string(),
                self.m_force_texture_case.to_value(),
            )),
            crate::opt_field("m_FontColor", &self.m_font_color),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            crate::opt_field("m_NewHashIdentity", &self.m_new_hash_identity),
            crate::opt_field("m_CharacterSpacing", &self.m_character_spacing),
            crate::opt_field("m_RenderMode", &self.m_render_mode),
            crate::opt_field("m_CharacterPadding", &self.m_character_padding),
            crate::opt_field("m_Style", &self.m_style),
            Some((
                "m_IncludeFontData".to_string(),
                self.m_include_font_data.to_value(),
            )),
            crate::opt_field("m_Use2xBehaviour", &self.m_use2x_behaviour),
            Some(("m_FontNames".to_string(), self.m_font_names.to_value())),
            crate::opt_field("m_FontName", &self.m_font_name),
            crate::opt_field("m_FallbackFontReferences", &self.m_fallback_font_references),
            crate::opt_field("m_CustomCharacters", &self.m_custom_characters),
            crate::opt_field("m_FontRenderingMode", &self.m_font_rendering_mode),
            crate::opt_field("m_Output", &self.m_output),
            crate::opt_field("m_UserData", &self.m_user_data),
            crate::opt_field("m_AscentCalculationMode", &self.m_ascent_calculation_mode),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("m_AssetBundleVariant", &self.m_asset_bundle_variant),
            crate::opt_field(
                "m_UseLegacyBoundsCalculation",
                &self.m_use_legacy_bounds_calculation,
            ),
            crate::opt_field(
                "m_ShouldRoundAdvanceValue",
                &self.m_should_round_advance_value,
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_FileIDToRecycleName" => {
                    (self.m_file_id_to_recycle_name).inspect_value(path, offset, limit)
                }
                "m_FontSize" => (self.m_font_size).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_OldHashIdentity" => {
                    (self.m_old_hash_identity).inspect_value(path, offset, limit)
                }
                "m_ForceTextureCase" => {
                    (self.m_force_texture_case).inspect_value(path, offset, limit)
                }
                "m_FontColor" => (self.m_font_color).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_NewHashIdentity" => {
                    (self.m_new_hash_identity).inspect_value(path, offset, limit)
                }
                "m_CharacterSpacing" => {
                    (self.m_character_spacing).inspect_value(path, offset, limit)
                }
                "m_RenderMode" => (self.m_render_mode).inspect_value(path, offset, limit),
                "m_CharacterPadding" => {
                    (self.m_character_padding).inspect_value(path, offset, limit)
                }
                "m_Style" => (self.m_style).inspect_value(path, offset, limit),
                "m_IncludeFontData" => {
                    (self.m_include_font_data).inspect_value(path, offset, limit)
                }
                "m_Use2xBehaviour" => (self.m_use2x_behaviour).inspect_value(path, offset, limit),
                "m_FontNames" => (self.m_font_names).inspect_value(path, offset, limit),
                "m_FontName" => (self.m_font_name).inspect_value(path, offset, limit),
                "m_FallbackFontReferences" => {
                    (self.m_fallback_font_references).inspect_value(path, offset, limit)
                }
                "m_CustomCharacters" => {
                    (self.m_custom_characters).inspect_value(path, offset, limit)
                }
                "m_FontRenderingMode" => {
                    (self.m_font_rendering_mode).inspect_value(path, offset, limit)
                }
                "m_Output" => (self.m_output).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_AscentCalculationMode" => {
                    (self.m_ascent_calculation_mode).inspect_value(path, offset, limit)
                }
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_UseLegacyBoundsCalculation" => {
                    (self.m_use_legacy_bounds_calculation).inspect_value(path, offset, limit)
                }
                "m_ShouldRoundAdvanceValue" => {
                    (self.m_should_round_advance_value).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_file_id_to_recycle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_FileIDToRecycleName".to_string(), v)),
            ((self.m_font_size).inspect_value(path, offset, limit))
                .map(|v| ("m_FontSize".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_old_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_OldHashIdentity".to_string(), v)),
            ((self.m_force_texture_case).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceTextureCase".to_string(), v)),
            ((self.m_font_color).inspect_value(path, offset, limit))
                .map(|v| ("m_FontColor".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_new_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_NewHashIdentity".to_string(), v)),
            ((self.m_character_spacing).inspect_value(path, offset, limit))
                .map(|v| ("m_CharacterSpacing".to_string(), v)),
            ((self.m_render_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderMode".to_string(), v)),
            ((self.m_character_padding).inspect_value(path, offset, limit))
                .map(|v| ("m_CharacterPadding".to_string(), v)),
            ((self.m_style).inspect_value(path, offset, limit)).map(|v| ("m_Style".to_string(), v)),
            ((self.m_include_font_data).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeFontData".to_string(), v)),
            ((self.m_use2x_behaviour).inspect_value(path, offset, limit))
                .map(|v| ("m_Use2xBehaviour".to_string(), v)),
            ((self.m_font_names).inspect_value(path, offset, limit))
                .map(|v| ("m_FontNames".to_string(), v)),
            ((self.m_font_name).inspect_value(path, offset, limit))
                .map(|v| ("m_FontName".to_string(), v)),
            ((self.m_fallback_font_references).inspect_value(path, offset, limit))
                .map(|v| ("m_FallbackFontReferences".to_string(), v)),
            ((self.m_custom_characters).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomCharacters".to_string(), v)),
            ((self.m_font_rendering_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_FontRenderingMode".to_string(), v)),
            ((self.m_output).inspect_value(path, offset, limit))
                .map(|v| ("m_Output".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_ascent_calculation_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_AscentCalculationMode".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_use_legacy_bounds_calculation).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLegacyBoundsCalculation".to_string(), v)),
            ((self.m_should_round_advance_value).inspect_value(path, offset, limit))
                .map(|v| ("m_ShouldRoundAdvanceValue".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
