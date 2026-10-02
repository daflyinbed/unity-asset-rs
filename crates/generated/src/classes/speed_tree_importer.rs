//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::per_lod_settings::PerLodSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：SpeedTreeImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpeedTreeImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for SpeedTreeImporterMExternalObjectsVariant {
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

impl crate::ToValue for SpeedTreeImporterMExternalObjectsVariant {
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

/// 版本枚举：SpeedTreeImporterMLodSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpeedTreeImporterMLodSettingsVariant {
    Default(Vec<PerLodSettings>),
    V2017_1(Vec<PerLodSettings>),
}

impl crate::Parse for SpeedTreeImporterMLodSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PerLodSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PerLodSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpeedTreeImporterMLodSettingsVariant {
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

/// 版本枚举：SpeedTreeImporterMEnableSmoothLodTransitionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpeedTreeImporterMEnableSmoothLodTransitionVariant {
    Default(bool),
    V5_1(bool),
}

impl crate::Parse for SpeedTreeImporterMEnableSmoothLodTransitionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpeedTreeImporterMEnableSmoothLodTransitionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SpeedTreeImporterMScaleFactorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpeedTreeImporterMScaleFactorVariant {
    Default(f32),
    V5_2(f32),
}

impl crate::Parse for SpeedTreeImporterMScaleFactorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V5_2(v))
        } else {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpeedTreeImporterMScaleFactorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SpeedTreeImporterMMaterialVersionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpeedTreeImporterMMaterialVersionVariant {
    Default(i32),
    V2018_3(i32),
}

impl crate::Parse for SpeedTreeImporterMMaterialVersionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpeedTreeImporterMMaterialVersionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpeedTreeImporter {
    pub m_name: String,
    pub m_main_color: ColorRgbAf,
    pub m_external_objects: Option<SpeedTreeImporterMExternalObjectsVariant>,
    pub m_spec_color: Option<ColorRgbAf>,
    pub m_hue_variation: ColorRgbAf,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_alpha_test_ref: f32,
    pub m_shininess: Option<f32>,
    pub m_best_wind_quality: i32,
    pub m_has_billboard: bool,
    pub m_lod_settings: SpeedTreeImporterMLodSettingsVariant,
    pub m_enable_hue_variation: Option<bool>,
    pub m_enable_smooth_lod_transition: SpeedTreeImporterMEnableSmoothLodTransitionVariant,
    pub m_enable_bump_mapping: Option<bool>,
    pub m_animate_cross_fading: Option<bool>,
    pub m_enable_subsurface_scattering: Option<bool>,
    pub m_billboard_transition_cross_fade_width: f32,
    pub m_fade_out_width: f32,
    pub m_selected_wind_quality: Option<i32>,
    pub m_scale_factor: SpeedTreeImporterMScaleFactorVariant,
    pub m_enable_shadow_casting: Option<bool>,
    pub m_material_version: Option<SpeedTreeImporterMMaterialVersionVariant>,
    pub m_enable_shadow_receiving: Option<bool>,
    pub m_user_data: String,
    pub m_enable_light_probes: Option<bool>,
    pub m_asset_bundle_name: String,
    pub m_material_location: Option<i32>,
    pub m_motion_vector_mode_enum_value: Option<i32>,
    pub m_reflection_probe_enum_value: Option<i32>,
    pub m_asset_bundle_variant: String,
    pub m_materials: Option<Vec<SourceAssetIdentifier>>,
    pub m_supports_embedded_materials: Option<bool>,
    pub m_file_id_type: Option<i32>,
    pub m_unit_conversion_enum_value: Option<i32>,
    pub m_generate_colliders: Option<bool>,
    pub m_generate_rigidbody: Option<bool>,
}

impl crate::Parse for SpeedTreeImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_main_color = None;
        let mut m_external_objects = None;
        let mut m_spec_color = None;
        let mut m_hue_variation = None;
        let mut m_used_file_i_ds = None;
        let mut m_alpha_test_ref = None;
        let mut m_shininess = None;
        let mut m_best_wind_quality = None;
        let mut m_has_billboard = None;
        let mut m_lod_settings = None;
        let mut m_enable_hue_variation = None;
        let mut m_enable_smooth_lod_transition = None;
        let mut m_enable_bump_mapping = None;
        let mut m_animate_cross_fading = None;
        let mut m_enable_subsurface_scattering = None;
        let mut m_billboard_transition_cross_fade_width = None;
        let mut m_fade_out_width = None;
        let mut m_selected_wind_quality = None;
        let mut m_scale_factor = None;
        let mut m_enable_shadow_casting = None;
        let mut m_material_version = None;
        let mut m_enable_shadow_receiving = None;
        let mut m_user_data = None;
        let mut m_enable_light_probes = None;
        let mut m_asset_bundle_name = None;
        let mut m_material_location = None;
        let mut m_motion_vector_mode_enum_value = None;
        let mut m_reflection_probe_enum_value = None;
        let mut m_asset_bundle_variant = None;
        let mut m_materials = None;
        let mut m_supports_embedded_materials = None;
        let mut m_file_id_type = None;
        let mut m_unit_conversion_enum_value = None;
        let mut m_generate_colliders = None;
        let mut m_generate_rigidbody = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(SpeedTreeImporterMExternalObjectsVariant::parse(rdr, ver)?);
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
            m_external_objects = Some(SpeedTreeImporterMExternalObjectsVariant::parse(rdr, ver)?);
        } else {
        }
        m_main_color = Some(ColorRgbAf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            m_spec_color = Some(ColorRgbAf::parse(rdr, ver)?);
        }
        m_hue_variation = Some(ColorRgbAf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            m_shininess = Some(rdr.read_f32()?);
        }
        m_alpha_test_ref = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            m_enable_hue_variation = Some(rdr.read_bool()?);
            m_enable_bump_mapping = Some(rdr.read_bool()?);
            m_enable_subsurface_scattering = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_best_wind_quality = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            m_selected_wind_quality = Some(rdr.read_i32()?);
            m_enable_shadow_casting = Some(rdr.read_bool()?);
            m_enable_shadow_receiving = Some(rdr.read_bool()?);
            m_enable_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_enum_value = Some(rdr.read_i32()?);
        } else {
        }
        m_has_billboard = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_lod_settings = Some(SpeedTreeImporterMLodSettingsVariant::parse(rdr, ver)?);
        m_enable_smooth_lod_transition = Some(
            SpeedTreeImporterMEnableSmoothLodTransitionVariant::parse(rdr, ver)?,
        );
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_animate_cross_fading = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_billboard_transition_cross_fade_width = Some(rdr.read_f32()?);
        m_fade_out_width = Some(rdr.read_f32()?);
        m_scale_factor = Some(SpeedTreeImporterMScaleFactorVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_unit_conversion_enum_value = Some(rdr.read_i32()?);
            m_motion_vector_mode_enum_value = Some(rdr.read_i32()?);
            m_generate_colliders = Some(rdr.read_bool()?);
            m_generate_rigidbody = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_id_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 2) {
            m_unit_conversion_enum_value = Some(rdr.read_i32()?);
            m_motion_vector_mode_enum_value = Some(rdr.read_i32()?);
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_id_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_unit_conversion_enum_value = Some(rdr.read_i32()?);
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_id_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            m_unit_conversion_enum_value = Some(rdr.read_i32()?);
            m_motion_vector_mode_enum_value = Some(rdr.read_i32()?);
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_id_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 3) {
            m_motion_vector_mode_enum_value = Some(rdr.read_i32()?);
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_id_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_id_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_material_version = Some(SpeedTreeImporterMMaterialVersionVariant::parse(rdr, ver)?);
        } else {
        }
        m_user_data = Some(rdr.read_aligned_string()?);
        m_asset_bundle_name = Some(rdr.read_aligned_string()?);
        m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(SpeedTreeImporter {
            m_name: m_name.unwrap(),
            m_main_color: m_main_color.unwrap(),
            m_external_objects,
            m_spec_color,
            m_hue_variation: m_hue_variation.unwrap(),
            m_used_file_i_ds,
            m_alpha_test_ref: m_alpha_test_ref.unwrap(),
            m_shininess,
            m_best_wind_quality: m_best_wind_quality.unwrap(),
            m_has_billboard: m_has_billboard.unwrap(),
            m_lod_settings: m_lod_settings.unwrap(),
            m_enable_hue_variation,
            m_enable_smooth_lod_transition: m_enable_smooth_lod_transition.unwrap(),
            m_enable_bump_mapping,
            m_animate_cross_fading,
            m_enable_subsurface_scattering,
            m_billboard_transition_cross_fade_width: m_billboard_transition_cross_fade_width
                .unwrap(),
            m_fade_out_width: m_fade_out_width.unwrap(),
            m_selected_wind_quality,
            m_scale_factor: m_scale_factor.unwrap(),
            m_enable_shadow_casting,
            m_material_version,
            m_enable_shadow_receiving,
            m_user_data: m_user_data.unwrap(),
            m_enable_light_probes,
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_material_location,
            m_motion_vector_mode_enum_value,
            m_reflection_probe_enum_value,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
            m_materials,
            m_supports_embedded_materials,
            m_file_id_type,
            m_unit_conversion_enum_value,
            m_generate_colliders,
            m_generate_rigidbody,
        })
    }
}

impl crate::ToValue for SpeedTreeImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_MainColor".to_string(), self.m_main_color.to_value())),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_SpecColor", &self.m_spec_color),
            Some((
                "m_HueVariation".to_string(),
                self.m_hue_variation.to_value(),
            )),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            Some((
                "m_AlphaTestRef".to_string(),
                self.m_alpha_test_ref.to_value(),
            )),
            crate::opt_field("m_Shininess", &self.m_shininess),
            Some((
                "m_BestWindQuality".to_string(),
                self.m_best_wind_quality.to_value(),
            )),
            Some((
                "m_HasBillboard".to_string(),
                self.m_has_billboard.to_value(),
            )),
            Some(("m_LODSettings".to_string(), self.m_lod_settings.to_value())),
            crate::opt_field("m_EnableHueVariation", &self.m_enable_hue_variation),
            Some((
                "m_EnableSmoothLODTransition".to_string(),
                self.m_enable_smooth_lod_transition.to_value(),
            )),
            crate::opt_field("m_EnableBumpMapping", &self.m_enable_bump_mapping),
            crate::opt_field("m_AnimateCrossFading", &self.m_animate_cross_fading),
            crate::opt_field(
                "m_EnableSubsurfaceScattering",
                &self.m_enable_subsurface_scattering,
            ),
            Some((
                "m_BillboardTransitionCrossFadeWidth".to_string(),
                self.m_billboard_transition_cross_fade_width.to_value(),
            )),
            Some((
                "m_FadeOutWidth".to_string(),
                self.m_fade_out_width.to_value(),
            )),
            crate::opt_field("m_SelectedWindQuality", &self.m_selected_wind_quality),
            Some(("m_ScaleFactor".to_string(), self.m_scale_factor.to_value())),
            crate::opt_field("m_EnableShadowCasting", &self.m_enable_shadow_casting),
            crate::opt_field("m_MaterialVersion", &self.m_material_version),
            crate::opt_field("m_EnableShadowReceiving", &self.m_enable_shadow_receiving),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            crate::opt_field("m_EnableLightProbes", &self.m_enable_light_probes),
            Some((
                "m_AssetBundleName".to_string(),
                self.m_asset_bundle_name.to_value(),
            )),
            crate::opt_field("m_MaterialLocation", &self.m_material_location),
            crate::opt_field(
                "m_MotionVectorModeEnumValue",
                &self.m_motion_vector_mode_enum_value,
            ),
            crate::opt_field(
                "m_ReflectionProbeEnumValue",
                &self.m_reflection_probe_enum_value,
            ),
            Some((
                "m_AssetBundleVariant".to_string(),
                self.m_asset_bundle_variant.to_value(),
            )),
            crate::opt_field("m_Materials", &self.m_materials),
            crate::opt_field(
                "m_SupportsEmbeddedMaterials",
                &self.m_supports_embedded_materials,
            ),
            crate::opt_field("m_FileIDType", &self.m_file_id_type),
            crate::opt_field(
                "m_UnitConversionEnumValue",
                &self.m_unit_conversion_enum_value,
            ),
            crate::opt_field("m_GenerateColliders", &self.m_generate_colliders),
            crate::opt_field("m_GenerateRigidbody", &self.m_generate_rigidbody),
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
                "m_MainColor" => (self.m_main_color).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_SpecColor" => (self.m_spec_color).inspect_value(path, offset, limit),
                "m_HueVariation" => (self.m_hue_variation).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_AlphaTestRef" => (self.m_alpha_test_ref).inspect_value(path, offset, limit),
                "m_Shininess" => (self.m_shininess).inspect_value(path, offset, limit),
                "m_BestWindQuality" => {
                    (self.m_best_wind_quality).inspect_value(path, offset, limit)
                }
                "m_HasBillboard" => (self.m_has_billboard).inspect_value(path, offset, limit),
                "m_LODSettings" => (self.m_lod_settings).inspect_value(path, offset, limit),
                "m_EnableHueVariation" => {
                    (self.m_enable_hue_variation).inspect_value(path, offset, limit)
                }
                "m_EnableSmoothLODTransition" => {
                    (self.m_enable_smooth_lod_transition).inspect_value(path, offset, limit)
                }
                "m_EnableBumpMapping" => {
                    (self.m_enable_bump_mapping).inspect_value(path, offset, limit)
                }
                "m_AnimateCrossFading" => {
                    (self.m_animate_cross_fading).inspect_value(path, offset, limit)
                }
                "m_EnableSubsurfaceScattering" => {
                    (self.m_enable_subsurface_scattering).inspect_value(path, offset, limit)
                }
                "m_BillboardTransitionCrossFadeWidth" => (self
                    .m_billboard_transition_cross_fade_width)
                    .inspect_value(path, offset, limit),
                "m_FadeOutWidth" => (self.m_fade_out_width).inspect_value(path, offset, limit),
                "m_SelectedWindQuality" => {
                    (self.m_selected_wind_quality).inspect_value(path, offset, limit)
                }
                "m_ScaleFactor" => (self.m_scale_factor).inspect_value(path, offset, limit),
                "m_EnableShadowCasting" => {
                    (self.m_enable_shadow_casting).inspect_value(path, offset, limit)
                }
                "m_MaterialVersion" => (self.m_material_version).inspect_value(path, offset, limit),
                "m_EnableShadowReceiving" => {
                    (self.m_enable_shadow_receiving).inspect_value(path, offset, limit)
                }
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_EnableLightProbes" => {
                    (self.m_enable_light_probes).inspect_value(path, offset, limit)
                }
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_MaterialLocation" => {
                    (self.m_material_location).inspect_value(path, offset, limit)
                }
                "m_MotionVectorModeEnumValue" => {
                    (self.m_motion_vector_mode_enum_value).inspect_value(path, offset, limit)
                }
                "m_ReflectionProbeEnumValue" => {
                    (self.m_reflection_probe_enum_value).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_SupportsEmbeddedMaterials" => {
                    (self.m_supports_embedded_materials).inspect_value(path, offset, limit)
                }
                "m_FileIDType" => (self.m_file_id_type).inspect_value(path, offset, limit),
                "m_UnitConversionEnumValue" => {
                    (self.m_unit_conversion_enum_value).inspect_value(path, offset, limit)
                }
                "m_GenerateColliders" => {
                    (self.m_generate_colliders).inspect_value(path, offset, limit)
                }
                "m_GenerateRigidbody" => {
                    (self.m_generate_rigidbody).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_main_color).inspect_value(path, offset, limit))
                .map(|v| ("m_MainColor".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_spec_color).inspect_value(path, offset, limit))
                .map(|v| ("m_SpecColor".to_string(), v)),
            ((self.m_hue_variation).inspect_value(path, offset, limit))
                .map(|v| ("m_HueVariation".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_alpha_test_ref).inspect_value(path, offset, limit))
                .map(|v| ("m_AlphaTestRef".to_string(), v)),
            ((self.m_shininess).inspect_value(path, offset, limit))
                .map(|v| ("m_Shininess".to_string(), v)),
            ((self.m_best_wind_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_BestWindQuality".to_string(), v)),
            ((self.m_has_billboard).inspect_value(path, offset, limit))
                .map(|v| ("m_HasBillboard".to_string(), v)),
            ((self.m_lod_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_LODSettings".to_string(), v)),
            ((self.m_enable_hue_variation).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableHueVariation".to_string(), v)),
            ((self.m_enable_smooth_lod_transition).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableSmoothLODTransition".to_string(), v)),
            ((self.m_enable_bump_mapping).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableBumpMapping".to_string(), v)),
            ((self.m_animate_cross_fading).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimateCrossFading".to_string(), v)),
            ((self.m_enable_subsurface_scattering).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableSubsurfaceScattering".to_string(), v)),
            ((self.m_billboard_transition_cross_fade_width).inspect_value(path, offset, limit))
                .map(|v| ("m_BillboardTransitionCrossFadeWidth".to_string(), v)),
            ((self.m_fade_out_width).inspect_value(path, offset, limit))
                .map(|v| ("m_FadeOutWidth".to_string(), v)),
            ((self.m_selected_wind_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWindQuality".to_string(), v)),
            ((self.m_scale_factor).inspect_value(path, offset, limit))
                .map(|v| ("m_ScaleFactor".to_string(), v)),
            ((self.m_enable_shadow_casting).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableShadowCasting".to_string(), v)),
            ((self.m_material_version).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialVersion".to_string(), v)),
            ((self.m_enable_shadow_receiving).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableShadowReceiving".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_enable_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableLightProbes".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_material_location).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialLocation".to_string(), v)),
            ((self.m_motion_vector_mode_enum_value).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectorModeEnumValue".to_string(), v)),
            ((self.m_reflection_probe_enum_value).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionProbeEnumValue".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_supports_embedded_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_SupportsEmbeddedMaterials".to_string(), v)),
            ((self.m_file_id_type).inspect_value(path, offset, limit))
                .map(|v| ("m_FileIDType".to_string(), v)),
            ((self.m_unit_conversion_enum_value).inspect_value(path, offset, limit))
                .map(|v| ("m_UnitConversionEnumValue".to_string(), v)),
            ((self.m_generate_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateColliders".to_string(), v)),
            ((self.m_generate_rigidbody).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateRigidbody".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
