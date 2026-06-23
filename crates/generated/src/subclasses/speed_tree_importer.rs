//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::per_lod_settings::PerLodSettings;

#[derive(Debug, Clone)]
pub struct SpeedTreeImporter {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_main_color: ColorRgba,
    pub m_spec_color: ColorRgba,
    pub m_hue_variation: ColorRgba,
    pub m_shininess: f32,
    pub m_alpha_test_ref: f32,
    pub m_best_wind_quality: i32,
    pub m_has_billboard: bool,
    pub m_lod_settings: Vec<PerLodSettings>,
    pub m_enable_smooth_lod_transition: bool,
    pub m_billboard_transition_cross_fade_width: f32,
    pub m_fade_out_width: f32,
    pub m_scale_factor: f32,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for SpeedTreeImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_main_color = ColorRgba::parse(r, ver)?;
        let m_spec_color = ColorRgba::parse(r, ver)?;
        let m_hue_variation = ColorRgba::parse(r, ver)?;
        let m_shininess = r.read_f32()?;
        let m_alpha_test_ref = r.read_f32()?;
        let m_best_wind_quality = r.read_i32()?;
        let m_has_billboard = r.read_bool()?;
        r.align(4)?;
        let m_lod_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PerLodSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_enable_smooth_lod_transition = r.read_bool()?;
        r.align(4)?;
        let m_billboard_transition_cross_fade_width = r.read_f32()?;
        let m_fade_out_width = r.read_f32()?;
        let m_scale_factor = r.read_f32()?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SpeedTreeImporter {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_main_color,
            m_spec_color,
            m_hue_variation,
            m_shininess,
            m_alpha_test_ref,
            m_best_wind_quality,
            m_has_billboard,
            m_lod_settings,
            m_enable_smooth_lod_transition,
            m_billboard_transition_cross_fade_width,
            m_fade_out_width,
            m_scale_factor,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
