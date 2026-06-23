//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightingSettings {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_gi_workflow_mode: i32,
    pub m_enable_baked_lightmaps: bool,
    pub m_enable_realtime_lightmaps: bool,
    pub m_realtime_environment_lighting: bool,
    pub m_bounce_scale: f32,
    pub m_albedo_boost: f32,
    pub m_using_shadowmask: bool,
    pub m_bake_backend: i32,
    pub m_lightmap_max_size: i32,
    pub m_bake_resolution: f32,
    pub m_padding: i32,
    pub m_texture_compression: bool,
    pub m_ao: bool,
    pub m_ao_max_distance: f32,
    pub m_comp_ao_exponent: f32,
    pub m_comp_ao_exponent_direct: f32,
    pub m_extract_ao: bool,
    pub m_mixed_bake_mode: i32,
    pub m_lightmaps_bake_mode: i32,
    pub m_filter_mode: i32,
    pub m_lightmap_parameters: PPtr,
    pub m_export_training_data: bool,
    pub m_training_data_destination: String,
    pub m_realtime_resolution: f32,
    pub m_force_white_albedo: bool,
    pub m_force_updates: bool,
    pub m_final_gather: bool,
    pub m_final_gather_ray_count: i32,
    pub m_final_gather_filtering: bool,
    pub m_pvr_culling: bool,
    pub m_pvr_sampling: i32,
    pub m_pvr_direct_sample_count: i32,
    pub m_pvr_sample_count: i32,
    pub m_pvr_environment_sample_count: i32,
    pub m_pvr_bounces: i32,
    pub m_pvr_environment_mis: i32,
    pub m_pvr_environment_reference_point_count: i32,
    pub m_pvr_filtering_mode: i32,
    pub m_pvr_denoiser_type_direct: i32,
    pub m_pvr_denoiser_type_indirect: i32,
    pub m_pvr_denoiser_type_ao: i32,
    pub m_pvr_filter_type_direct: i32,
    pub m_pvr_filter_type_indirect: i32,
    pub m_pvr_filter_type_ao: i32,
    pub m_pvr_filtering_gauss_radius_direct: i32,
    pub m_pvr_filtering_gauss_radius_indirect: i32,
    pub m_pvr_filtering_gauss_radius_ao: i32,
    pub m_pvr_filtering_atrous_position_sigma_direct: f32,
    pub m_pvr_filtering_atrous_position_sigma_indirect: f32,
    pub m_pvr_filtering_atrous_position_sigma_ao: f32,
}

impl crate::Parse for LightingSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_gi_workflow_mode = r.read_i32()?;
        let m_enable_baked_lightmaps = r.read_bool()?;
        let m_enable_realtime_lightmaps = r.read_bool()?;
        r.align(4)?;
        let m_realtime_environment_lighting = r.read_bool()?;
        r.align(4)?;
        let m_bounce_scale = r.read_f32()?;
        let m_albedo_boost = r.read_f32()?;
        let m_using_shadowmask = r.read_bool()?;
        r.align(4)?;
        let m_bake_backend = r.read_i32()?;
        let m_lightmap_max_size = r.read_i32()?;
        let m_bake_resolution = r.read_f32()?;
        let m_padding = r.read_i32()?;
        let m_texture_compression = r.read_bool()?;
        let m_ao = r.read_bool()?;
        r.align(4)?;
        let m_ao_max_distance = r.read_f32()?;
        let m_comp_ao_exponent = r.read_f32()?;
        let m_comp_ao_exponent_direct = r.read_f32()?;
        let m_extract_ao = r.read_bool()?;
        r.align(4)?;
        let m_mixed_bake_mode = r.read_i32()?;
        let m_lightmaps_bake_mode = r.read_i32()?;
        let m_filter_mode = r.read_i32()?;
        let m_lightmap_parameters = crate::PPtr::parse(r, ver)?;
        let m_export_training_data = r.read_bool()?;
        r.align(4)?;
        let m_training_data_destination = r.read_aligned_string()?;
        r.align(4)?;
        let m_realtime_resolution = r.read_f32()?;
        let m_force_white_albedo = r.read_bool()?;
        let m_force_updates = r.read_bool()?;
        let m_final_gather = r.read_bool()?;
        r.align(4)?;
        let m_final_gather_ray_count = r.read_i32()?;
        let m_final_gather_filtering = r.read_bool()?;
        r.align(4)?;
        let m_pvr_culling = r.read_bool()?;
        r.align(4)?;
        let m_pvr_sampling = r.read_i32()?;
        let m_pvr_direct_sample_count = r.read_i32()?;
        let m_pvr_sample_count = r.read_i32()?;
        let m_pvr_environment_sample_count = r.read_i32()?;
        let m_pvr_bounces = r.read_i32()?;
        let m_pvr_environment_mis = r.read_i32()?;
        let m_pvr_environment_reference_point_count = r.read_i32()?;
        let m_pvr_filtering_mode = r.read_i32()?;
        let m_pvr_denoiser_type_direct = r.read_i32()?;
        let m_pvr_denoiser_type_indirect = r.read_i32()?;
        let m_pvr_denoiser_type_ao = r.read_i32()?;
        let m_pvr_filter_type_direct = r.read_i32()?;
        let m_pvr_filter_type_indirect = r.read_i32()?;
        let m_pvr_filter_type_ao = r.read_i32()?;
        let m_pvr_filtering_gauss_radius_direct = r.read_i32()?;
        let m_pvr_filtering_gauss_radius_indirect = r.read_i32()?;
        let m_pvr_filtering_gauss_radius_ao = r.read_i32()?;
        let m_pvr_filtering_atrous_position_sigma_direct = r.read_f32()?;
        let m_pvr_filtering_atrous_position_sigma_indirect = r.read_f32()?;
        let m_pvr_filtering_atrous_position_sigma_ao = r.read_f32()?;

        Ok(LightingSettings {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_gi_workflow_mode,
            m_enable_baked_lightmaps,
            m_enable_realtime_lightmaps,
            m_realtime_environment_lighting,
            m_bounce_scale,
            m_albedo_boost,
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
