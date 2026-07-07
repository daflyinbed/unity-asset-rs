//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightingSettings {
    pub m_name: String,
    pub m_gi_workflow_mode: Option<i32>,
    pub m_enable_baked_lightmaps: bool,
    pub m_enable_realtime_lightmaps: bool,
    pub m_realtime_environment_lighting: bool,
    pub m_bounce_scale: f32,
    pub m_albedo_boost: f32,
    pub m_indirect_output_scale: Option<f32>,
    pub m_using_shadowmask: bool,
    pub m_bake_backend: Option<i32>,
    pub m_lightmap_max_size: Option<i32>,
    pub m_bake_resolution: Option<f32>,
    pub m_padding: Option<i32>,
    pub m_texture_compression: Option<bool>,
    pub m_ao: Option<bool>,
    pub m_ao_max_distance: Option<f32>,
    pub m_comp_ao_exponent: Option<f32>,
    pub m_comp_ao_exponent_direct: Option<f32>,
    pub m_extract_ao: Option<bool>,
    pub m_mixed_bake_mode: Option<i32>,
    pub m_lightmaps_bake_mode: Option<i32>,
    pub m_filter_mode: Option<i32>,
    pub m_lightmap_parameters: Option<PPtr>,
    pub m_export_training_data: Option<bool>,
    pub m_training_data_destination: Option<String>,
    pub m_realtime_resolution: Option<f32>,
    pub m_force_white_albedo: Option<bool>,
    pub m_force_updates: Option<bool>,
    pub m_final_gather: Option<bool>,
    pub m_final_gather_ray_count: Option<i32>,
    pub m_final_gather_filtering: Option<bool>,
    pub m_pvr_culling: Option<bool>,
    pub m_pvr_sampling: Option<i32>,
    pub m_pvr_direct_sample_count: Option<i32>,
    pub m_pvr_sample_count: Option<i32>,
    pub m_pvr_environment_sample_count: Option<i32>,
    pub m_pvr_bounces: Option<i32>,
    pub m_pvr_environment_mis: Option<i32>,
    pub m_pvr_environment_reference_point_count: Option<i32>,
    pub m_pvr_filtering_mode: Option<i32>,
    pub m_pvr_denoiser_type_direct: Option<i32>,
    pub m_pvr_denoiser_type_indirect: Option<i32>,
    pub m_pvr_denoiser_type_ao: Option<i32>,
    pub m_pvr_filter_type_direct: Option<i32>,
    pub m_pvr_filter_type_indirect: Option<i32>,
    pub m_pvr_filter_type_ao: Option<i32>,
    pub m_pvr_filtering_gauss_radius_direct: Option<i32>,
    pub m_pvr_filtering_gauss_radius_indirect: Option<i32>,
    pub m_pvr_filtering_gauss_radius_ao: Option<i32>,
    pub m_pvr_filtering_atrous_position_sigma_direct: Option<f32>,
    pub m_pvr_filtering_atrous_position_sigma_indirect: Option<f32>,
    pub m_pvr_filtering_atrous_position_sigma_ao: Option<f32>,
}

impl crate::Parse for LightingSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_gi_workflow_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_baked_lightmaps = r.read_bool()?;
        let m_enable_realtime_lightmaps = r.read_bool()?;
        let m_realtime_environment_lighting = r.read_bool()?;
        r.align(4)?;
        let m_bounce_scale = r.read_f32()?;
        let m_albedo_boost = r.read_f32()?;
        let m_indirect_output_scale = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_using_shadowmask = r.read_bool()?;
        r.align(4)?;
        let m_bake_backend = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmap_max_size = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_bake_resolution = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_padding = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_compression = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_ao = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ao_max_distance = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_comp_ao_exponent = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_comp_ao_exponent_direct = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_extract_ao = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mixed_bake_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmaps_bake_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_filter_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmap_parameters = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_export_training_data = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_training_data_destination = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_realtime_resolution = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_force_white_albedo = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_force_updates = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_final_gather = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_final_gather_ray_count = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_final_gather_filtering = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_pvr_culling = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_pvr_sampling = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_direct_sample_count = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_sample_count = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_environment_sample_count = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_bounces = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_environment_mis = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_environment_reference_point_count = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filtering_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_denoiser_type_direct = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_denoiser_type_indirect = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_denoiser_type_ao = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filter_type_direct = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filter_type_indirect = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filter_type_ao = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filtering_gauss_radius_direct = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filtering_gauss_radius_indirect = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filtering_gauss_radius_ao = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_pvr_filtering_atrous_position_sigma_direct = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_pvr_filtering_atrous_position_sigma_indirect = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_pvr_filtering_atrous_position_sigma_ao = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };

        Ok(LightingSettings {
            m_name,
            m_gi_workflow_mode,
            m_enable_baked_lightmaps,
            m_enable_realtime_lightmaps,
            m_realtime_environment_lighting,
            m_bounce_scale,
            m_albedo_boost,
            m_indirect_output_scale,
            m_using_shadowmask,
            m_bake_backend,
            m_lightmap_max_size,
            m_bake_resolution,
            m_padding,
            m_texture_compression,
            m_ao,
            m_ao_max_distance,
            m_comp_ao_exponent,
            m_comp_ao_exponent_direct,
            m_extract_ao,
            m_mixed_bake_mode,
            m_lightmaps_bake_mode,
            m_filter_mode,
            m_lightmap_parameters,
            m_export_training_data,
            m_training_data_destination,
            m_realtime_resolution,
            m_force_white_albedo,
            m_force_updates,
            m_final_gather,
            m_final_gather_ray_count,
            m_final_gather_filtering,
            m_pvr_culling,
            m_pvr_sampling,
            m_pvr_direct_sample_count,
            m_pvr_sample_count,
            m_pvr_environment_sample_count,
            m_pvr_bounces,
            m_pvr_environment_mis,
            m_pvr_environment_reference_point_count,
            m_pvr_filtering_mode,
            m_pvr_denoiser_type_direct,
            m_pvr_denoiser_type_indirect,
            m_pvr_denoiser_type_ao,
            m_pvr_filter_type_direct,
            m_pvr_filter_type_indirect,
            m_pvr_filter_type_ao,
            m_pvr_filtering_gauss_radius_direct,
            m_pvr_filtering_gauss_radius_indirect,
            m_pvr_filtering_gauss_radius_ao,
            m_pvr_filtering_atrous_position_sigma_direct,
            m_pvr_filtering_atrous_position_sigma_indirect,
            m_pvr_filtering_atrous_position_sigma_ao,
        })
    }
}
