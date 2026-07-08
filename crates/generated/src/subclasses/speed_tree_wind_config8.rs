//! 自动生成。请勿手动修改。

use crate::subclasses::s_branch_wind_level::SBranchWindLevel;
use crate::subclasses::s_wind_group::SWindGroup;

#[derive(Debug, Clone)]
pub struct SpeedTreeWindConfig8 {
    pub m_f_strength_response: f32,
    pub m_f_direction_response: f32,
    pub m_f_gust_frequency: f32,
    pub m_f_gust_strength_min: f32,
    pub m_f_gust_strength_max: f32,
    pub m_f_gust_duration_min: f32,
    pub m_f_gust_duration_max: f32,
    pub m_f_gust_rise_scalar: f32,
    pub m_f_gust_fall_scalar: f32,
    pub m_f_anchor_offset: f32,
    pub m_f_anchor_distance_scale: f32,
    pub oscillation0_0: f32,
    pub oscillation0_1: f32,
    pub oscillation0_2: f32,
    pub oscillation0_3: f32,
    pub oscillation0_4: f32,
    pub oscillation0_5: f32,
    pub oscillation0_6: f32,
    pub oscillation0_7: f32,
    pub oscillation0_8: f32,
    pub oscillation0_9: f32,
    pub oscillation1_0: f32,
    pub oscillation1_1: f32,
    pub oscillation1_2: f32,
    pub oscillation1_3: f32,
    pub oscillation1_4: f32,
    pub oscillation1_5: f32,
    pub oscillation1_6: f32,
    pub oscillation1_7: f32,
    pub oscillation1_8: f32,
    pub oscillation1_9: f32,
    pub oscillation2_0: f32,
    pub oscillation2_1: f32,
    pub oscillation2_2: f32,
    pub oscillation2_3: f32,
    pub oscillation2_4: f32,
    pub oscillation2_5: f32,
    pub oscillation2_6: f32,
    pub oscillation2_7: f32,
    pub oscillation2_8: f32,
    pub oscillation2_9: f32,
    pub oscillation3_0: f32,
    pub oscillation3_1: f32,
    pub oscillation3_2: f32,
    pub oscillation3_3: f32,
    pub oscillation3_4: f32,
    pub oscillation3_5: f32,
    pub oscillation3_6: f32,
    pub oscillation3_7: f32,
    pub oscillation3_8: f32,
    pub oscillation3_9: f32,
    pub oscillation4_0: f32,
    pub oscillation4_1: f32,
    pub oscillation4_2: f32,
    pub oscillation4_3: f32,
    pub oscillation4_4: f32,
    pub oscillation4_5: f32,
    pub oscillation4_6: f32,
    pub oscillation4_7: f32,
    pub oscillation4_8: f32,
    pub oscillation4_9: f32,
    pub oscillation5_0: f32,
    pub oscillation5_1: f32,
    pub oscillation5_2: f32,
    pub oscillation5_3: f32,
    pub oscillation5_4: f32,
    pub oscillation5_5: f32,
    pub oscillation5_6: f32,
    pub oscillation5_7: f32,
    pub oscillation5_8: f32,
    pub oscillation5_9: f32,
    pub oscillation6_0: f32,
    pub oscillation6_1: f32,
    pub oscillation6_2: f32,
    pub oscillation6_3: f32,
    pub oscillation6_4: f32,
    pub oscillation6_5: f32,
    pub oscillation6_6: f32,
    pub oscillation6_7: f32,
    pub oscillation6_8: f32,
    pub oscillation6_9: f32,
    pub oscillation7_0: f32,
    pub oscillation7_1: f32,
    pub oscillation7_2: f32,
    pub oscillation7_3: f32,
    pub oscillation7_4: f32,
    pub oscillation7_5: f32,
    pub oscillation7_6: f32,
    pub oscillation7_7: f32,
    pub oscillation7_8: f32,
    pub oscillation7_9: f32,
    pub oscillation8_0: f32,
    pub oscillation8_1: f32,
    pub oscillation8_2: f32,
    pub oscillation8_3: f32,
    pub oscillation8_4: f32,
    pub oscillation8_5: f32,
    pub oscillation8_6: f32,
    pub oscillation8_7: f32,
    pub oscillation8_8: f32,
    pub oscillation8_9: f32,
    pub oscillation9_0: f32,
    pub oscillation9_1: f32,
    pub oscillation9_2: f32,
    pub oscillation9_3: f32,
    pub oscillation9_4: f32,
    pub oscillation9_5: f32,
    pub oscillation9_6: f32,
    pub oscillation9_7: f32,
    pub oscillation9_8: f32,
    pub oscillation9_9: f32,
    pub m_f_global_height: f32,
    pub m_f_global_height_exponent: f32,
    pub m_af_global_distance_0: f32,
    pub m_af_global_distance_1: f32,
    pub m_af_global_distance_2: f32,
    pub m_af_global_distance_3: f32,
    pub m_af_global_distance_4: f32,
    pub m_af_global_distance_5: f32,
    pub m_af_global_distance_6: f32,
    pub m_af_global_distance_7: f32,
    pub m_af_global_distance_8: f32,
    pub m_af_global_distance_9: f32,
    pub m_af_global_direction_adherence_0: f32,
    pub m_af_global_direction_adherence_1: f32,
    pub m_af_global_direction_adherence_2: f32,
    pub m_af_global_direction_adherence_3: f32,
    pub m_af_global_direction_adherence_4: f32,
    pub m_af_global_direction_adherence_5: f32,
    pub m_af_global_direction_adherence_6: f32,
    pub m_af_global_direction_adherence_7: f32,
    pub m_af_global_direction_adherence_8: f32,
    pub m_af_global_direction_adherence_9: f32,
    pub branch_level1: SBranchWindLevel,
    pub branch_level2: SBranchWindLevel,
    pub leaf_group1: SWindGroup,
    pub leaf_group2: SWindGroup,
    pub m_af_frond_ripple_distance_0: f32,
    pub m_af_frond_ripple_distance_1: f32,
    pub m_af_frond_ripple_distance_2: f32,
    pub m_af_frond_ripple_distance_3: f32,
    pub m_af_frond_ripple_distance_4: f32,
    pub m_af_frond_ripple_distance_5: f32,
    pub m_af_frond_ripple_distance_6: f32,
    pub m_af_frond_ripple_distance_7: f32,
    pub m_af_frond_ripple_distance_8: f32,
    pub m_af_frond_ripple_distance_9: f32,
    pub m_f_frond_ripple_tile: f32,
    pub m_f_frond_ripple_lighting_scalar: f32,
    pub m_f_rolling_noise_size: f32,
    pub m_f_rolling_noise_twist: f32,
    pub m_f_rolling_noise_turbulence: f32,
    pub m_f_rolling_noise_period: f32,
    pub m_f_rolling_noise_speed: f32,
    pub m_f_rolling_branch_field_min: f32,
    pub m_f_rolling_branch_lighting_adjust: f32,
    pub m_f_rolling_branch_vertical_offset: f32,
    pub m_f_rolling_leaf_ripple_min: f32,
    pub m_f_rolling_leaf_tumble_min: f32,
    pub branch_wind_anchor0: f32,
    pub branch_wind_anchor1: f32,
    pub branch_wind_anchor2: f32,
    pub m_f_max_branch_level1_length: f32,
    pub global_wind: bool,
    pub global_preserve_shape: bool,
    pub branch_simple_1: bool,
    pub branch_directional_1: bool,
    pub branch_directional_frond_1: bool,
    pub branch_turbulence_1: bool,
    pub branch_whip_1: bool,
    pub branch_osc_complex_1: bool,
    pub branch_simple_2: bool,
    pub branch_directional_2: bool,
    pub branch_directional_frond_2: bool,
    pub branch_turbulence_2: bool,
    pub branch_whip_2: bool,
    pub branch_osc_complex_2: bool,
    pub leaf_ripple_vertex_normal_1: bool,
    pub leaf_ripple_computed_1: bool,
    pub leaf_tumble_1: bool,
    pub leaf_twitch_1: bool,
    pub leaf_occlusion_1: bool,
    pub leaf_ripple_vertex_normal_2: bool,
    pub leaf_ripple_computed_2: bool,
    pub leaf_tumble_2: bool,
    pub leaf_twitch_2: bool,
    pub leaf_occlusion_2: bool,
    pub frond_ripple_one_sided: bool,
    pub frond_ripple_two_sided: bool,
    pub frond_ripple_adjust_lighting: bool,
    pub rolling: bool,
}

impl crate::Parse for SpeedTreeWindConfig8 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_f_strength_response = rdr.read_f32()?;
        let m_f_direction_response = rdr.read_f32()?;
        let m_f_gust_frequency = rdr.read_f32()?;
        let m_f_gust_strength_min = rdr.read_f32()?;
        let m_f_gust_strength_max = rdr.read_f32()?;
        let m_f_gust_duration_min = rdr.read_f32()?;
        let m_f_gust_duration_max = rdr.read_f32()?;
        let m_f_gust_rise_scalar = rdr.read_f32()?;
        let m_f_gust_fall_scalar = rdr.read_f32()?;
        let m_f_anchor_offset = rdr.read_f32()?;
        let m_f_anchor_distance_scale = rdr.read_f32()?;
        let oscillation0_0 = rdr.read_f32()?;
        let oscillation0_1 = rdr.read_f32()?;
        let oscillation0_2 = rdr.read_f32()?;
        let oscillation0_3 = rdr.read_f32()?;
        let oscillation0_4 = rdr.read_f32()?;
        let oscillation0_5 = rdr.read_f32()?;
        let oscillation0_6 = rdr.read_f32()?;
        let oscillation0_7 = rdr.read_f32()?;
        let oscillation0_8 = rdr.read_f32()?;
        let oscillation0_9 = rdr.read_f32()?;
        let oscillation1_0 = rdr.read_f32()?;
        let oscillation1_1 = rdr.read_f32()?;
        let oscillation1_2 = rdr.read_f32()?;
        let oscillation1_3 = rdr.read_f32()?;
        let oscillation1_4 = rdr.read_f32()?;
        let oscillation1_5 = rdr.read_f32()?;
        let oscillation1_6 = rdr.read_f32()?;
        let oscillation1_7 = rdr.read_f32()?;
        let oscillation1_8 = rdr.read_f32()?;
        let oscillation1_9 = rdr.read_f32()?;
        let oscillation2_0 = rdr.read_f32()?;
        let oscillation2_1 = rdr.read_f32()?;
        let oscillation2_2 = rdr.read_f32()?;
        let oscillation2_3 = rdr.read_f32()?;
        let oscillation2_4 = rdr.read_f32()?;
        let oscillation2_5 = rdr.read_f32()?;
        let oscillation2_6 = rdr.read_f32()?;
        let oscillation2_7 = rdr.read_f32()?;
        let oscillation2_8 = rdr.read_f32()?;
        let oscillation2_9 = rdr.read_f32()?;
        let oscillation3_0 = rdr.read_f32()?;
        let oscillation3_1 = rdr.read_f32()?;
        let oscillation3_2 = rdr.read_f32()?;
        let oscillation3_3 = rdr.read_f32()?;
        let oscillation3_4 = rdr.read_f32()?;
        let oscillation3_5 = rdr.read_f32()?;
        let oscillation3_6 = rdr.read_f32()?;
        let oscillation3_7 = rdr.read_f32()?;
        let oscillation3_8 = rdr.read_f32()?;
        let oscillation3_9 = rdr.read_f32()?;
        let oscillation4_0 = rdr.read_f32()?;
        let oscillation4_1 = rdr.read_f32()?;
        let oscillation4_2 = rdr.read_f32()?;
        let oscillation4_3 = rdr.read_f32()?;
        let oscillation4_4 = rdr.read_f32()?;
        let oscillation4_5 = rdr.read_f32()?;
        let oscillation4_6 = rdr.read_f32()?;
        let oscillation4_7 = rdr.read_f32()?;
        let oscillation4_8 = rdr.read_f32()?;
        let oscillation4_9 = rdr.read_f32()?;
        let oscillation5_0 = rdr.read_f32()?;
        let oscillation5_1 = rdr.read_f32()?;
        let oscillation5_2 = rdr.read_f32()?;
        let oscillation5_3 = rdr.read_f32()?;
        let oscillation5_4 = rdr.read_f32()?;
        let oscillation5_5 = rdr.read_f32()?;
        let oscillation5_6 = rdr.read_f32()?;
        let oscillation5_7 = rdr.read_f32()?;
        let oscillation5_8 = rdr.read_f32()?;
        let oscillation5_9 = rdr.read_f32()?;
        let oscillation6_0 = rdr.read_f32()?;
        let oscillation6_1 = rdr.read_f32()?;
        let oscillation6_2 = rdr.read_f32()?;
        let oscillation6_3 = rdr.read_f32()?;
        let oscillation6_4 = rdr.read_f32()?;
        let oscillation6_5 = rdr.read_f32()?;
        let oscillation6_6 = rdr.read_f32()?;
        let oscillation6_7 = rdr.read_f32()?;
        let oscillation6_8 = rdr.read_f32()?;
        let oscillation6_9 = rdr.read_f32()?;
        let oscillation7_0 = rdr.read_f32()?;
        let oscillation7_1 = rdr.read_f32()?;
        let oscillation7_2 = rdr.read_f32()?;
        let oscillation7_3 = rdr.read_f32()?;
        let oscillation7_4 = rdr.read_f32()?;
        let oscillation7_5 = rdr.read_f32()?;
        let oscillation7_6 = rdr.read_f32()?;
        let oscillation7_7 = rdr.read_f32()?;
        let oscillation7_8 = rdr.read_f32()?;
        let oscillation7_9 = rdr.read_f32()?;
        let oscillation8_0 = rdr.read_f32()?;
        let oscillation8_1 = rdr.read_f32()?;
        let oscillation8_2 = rdr.read_f32()?;
        let oscillation8_3 = rdr.read_f32()?;
        let oscillation8_4 = rdr.read_f32()?;
        let oscillation8_5 = rdr.read_f32()?;
        let oscillation8_6 = rdr.read_f32()?;
        let oscillation8_7 = rdr.read_f32()?;
        let oscillation8_8 = rdr.read_f32()?;
        let oscillation8_9 = rdr.read_f32()?;
        let oscillation9_0 = rdr.read_f32()?;
        let oscillation9_1 = rdr.read_f32()?;
        let oscillation9_2 = rdr.read_f32()?;
        let oscillation9_3 = rdr.read_f32()?;
        let oscillation9_4 = rdr.read_f32()?;
        let oscillation9_5 = rdr.read_f32()?;
        let oscillation9_6 = rdr.read_f32()?;
        let oscillation9_7 = rdr.read_f32()?;
        let oscillation9_8 = rdr.read_f32()?;
        let oscillation9_9 = rdr.read_f32()?;
        let m_f_global_height = rdr.read_f32()?;
        let m_f_global_height_exponent = rdr.read_f32()?;
        let m_af_global_distance_0 = rdr.read_f32()?;
        let m_af_global_distance_1 = rdr.read_f32()?;
        let m_af_global_distance_2 = rdr.read_f32()?;
        let m_af_global_distance_3 = rdr.read_f32()?;
        let m_af_global_distance_4 = rdr.read_f32()?;
        let m_af_global_distance_5 = rdr.read_f32()?;
        let m_af_global_distance_6 = rdr.read_f32()?;
        let m_af_global_distance_7 = rdr.read_f32()?;
        let m_af_global_distance_8 = rdr.read_f32()?;
        let m_af_global_distance_9 = rdr.read_f32()?;
        let m_af_global_direction_adherence_0 = rdr.read_f32()?;
        let m_af_global_direction_adherence_1 = rdr.read_f32()?;
        let m_af_global_direction_adherence_2 = rdr.read_f32()?;
        let m_af_global_direction_adherence_3 = rdr.read_f32()?;
        let m_af_global_direction_adherence_4 = rdr.read_f32()?;
        let m_af_global_direction_adherence_5 = rdr.read_f32()?;
        let m_af_global_direction_adherence_6 = rdr.read_f32()?;
        let m_af_global_direction_adherence_7 = rdr.read_f32()?;
        let m_af_global_direction_adherence_8 = rdr.read_f32()?;
        let m_af_global_direction_adherence_9 = rdr.read_f32()?;
        let branch_level1 = SBranchWindLevel::parse(rdr, ver)?;
        let branch_level2 = SBranchWindLevel::parse(rdr, ver)?;
        let leaf_group1 = SWindGroup::parse(rdr, ver)?;
        let leaf_group2 = SWindGroup::parse(rdr, ver)?;
        let m_af_frond_ripple_distance_0 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_1 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_2 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_3 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_4 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_5 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_6 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_7 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_8 = rdr.read_f32()?;
        let m_af_frond_ripple_distance_9 = rdr.read_f32()?;
        let m_f_frond_ripple_tile = rdr.read_f32()?;
        let m_f_frond_ripple_lighting_scalar = rdr.read_f32()?;
        let m_f_rolling_noise_size = rdr.read_f32()?;
        let m_f_rolling_noise_twist = rdr.read_f32()?;
        let m_f_rolling_noise_turbulence = rdr.read_f32()?;
        let m_f_rolling_noise_period = rdr.read_f32()?;
        let m_f_rolling_noise_speed = rdr.read_f32()?;
        let m_f_rolling_branch_field_min = rdr.read_f32()?;
        let m_f_rolling_branch_lighting_adjust = rdr.read_f32()?;
        let m_f_rolling_branch_vertical_offset = rdr.read_f32()?;
        let m_f_rolling_leaf_ripple_min = rdr.read_f32()?;
        let m_f_rolling_leaf_tumble_min = rdr.read_f32()?;
        let branch_wind_anchor0 = rdr.read_f32()?;
        let branch_wind_anchor1 = rdr.read_f32()?;
        let branch_wind_anchor2 = rdr.read_f32()?;
        let m_f_max_branch_level1_length = rdr.read_f32()?;
        let global_wind = rdr.read_bool()?;
        let global_preserve_shape = rdr.read_bool()?;
        let branch_simple_1 = rdr.read_bool()?;
        let branch_directional_1 = rdr.read_bool()?;
        let branch_directional_frond_1 = rdr.read_bool()?;
        let branch_turbulence_1 = rdr.read_bool()?;
        let branch_whip_1 = rdr.read_bool()?;
        let branch_osc_complex_1 = rdr.read_bool()?;
        let branch_simple_2 = rdr.read_bool()?;
        let branch_directional_2 = rdr.read_bool()?;
        let branch_directional_frond_2 = rdr.read_bool()?;
        let branch_turbulence_2 = rdr.read_bool()?;
        let branch_whip_2 = rdr.read_bool()?;
        let branch_osc_complex_2 = rdr.read_bool()?;
        let leaf_ripple_vertex_normal_1 = rdr.read_bool()?;
        let leaf_ripple_computed_1 = rdr.read_bool()?;
        let leaf_tumble_1 = rdr.read_bool()?;
        let leaf_twitch_1 = rdr.read_bool()?;
        let leaf_occlusion_1 = rdr.read_bool()?;
        let leaf_ripple_vertex_normal_2 = rdr.read_bool()?;
        let leaf_ripple_computed_2 = rdr.read_bool()?;
        let leaf_tumble_2 = rdr.read_bool()?;
        let leaf_twitch_2 = rdr.read_bool()?;
        let leaf_occlusion_2 = rdr.read_bool()?;
        let frond_ripple_one_sided = rdr.read_bool()?;
        let frond_ripple_two_sided = rdr.read_bool()?;
        let frond_ripple_adjust_lighting = rdr.read_bool()?;
        let rolling = rdr.read_bool()?;

        Ok(SpeedTreeWindConfig8 {
            m_f_strength_response,
            m_f_direction_response,
            m_f_gust_frequency,
            m_f_gust_strength_min,
            m_f_gust_strength_max,
            m_f_gust_duration_min,
            m_f_gust_duration_max,
            m_f_gust_rise_scalar,
            m_f_gust_fall_scalar,
            m_f_anchor_offset,
            m_f_anchor_distance_scale,
            oscillation0_0,
            oscillation0_1,
            oscillation0_2,
            oscillation0_3,
            oscillation0_4,
            oscillation0_5,
            oscillation0_6,
            oscillation0_7,
            oscillation0_8,
            oscillation0_9,
            oscillation1_0,
            oscillation1_1,
            oscillation1_2,
            oscillation1_3,
            oscillation1_4,
            oscillation1_5,
            oscillation1_6,
            oscillation1_7,
            oscillation1_8,
            oscillation1_9,
            oscillation2_0,
            oscillation2_1,
            oscillation2_2,
            oscillation2_3,
            oscillation2_4,
            oscillation2_5,
            oscillation2_6,
            oscillation2_7,
            oscillation2_8,
            oscillation2_9,
            oscillation3_0,
            oscillation3_1,
            oscillation3_2,
            oscillation3_3,
            oscillation3_4,
            oscillation3_5,
            oscillation3_6,
            oscillation3_7,
            oscillation3_8,
            oscillation3_9,
            oscillation4_0,
            oscillation4_1,
            oscillation4_2,
            oscillation4_3,
            oscillation4_4,
            oscillation4_5,
            oscillation4_6,
            oscillation4_7,
            oscillation4_8,
            oscillation4_9,
            oscillation5_0,
            oscillation5_1,
            oscillation5_2,
            oscillation5_3,
            oscillation5_4,
            oscillation5_5,
            oscillation5_6,
            oscillation5_7,
            oscillation5_8,
            oscillation5_9,
            oscillation6_0,
            oscillation6_1,
            oscillation6_2,
            oscillation6_3,
            oscillation6_4,
            oscillation6_5,
            oscillation6_6,
            oscillation6_7,
            oscillation6_8,
            oscillation6_9,
            oscillation7_0,
            oscillation7_1,
            oscillation7_2,
            oscillation7_3,
            oscillation7_4,
            oscillation7_5,
            oscillation7_6,
            oscillation7_7,
            oscillation7_8,
            oscillation7_9,
            oscillation8_0,
            oscillation8_1,
            oscillation8_2,
            oscillation8_3,
            oscillation8_4,
            oscillation8_5,
            oscillation8_6,
            oscillation8_7,
            oscillation8_8,
            oscillation8_9,
            oscillation9_0,
            oscillation9_1,
            oscillation9_2,
            oscillation9_3,
            oscillation9_4,
            oscillation9_5,
            oscillation9_6,
            oscillation9_7,
            oscillation9_8,
            oscillation9_9,
            m_f_global_height,
            m_f_global_height_exponent,
            m_af_global_distance_0,
            m_af_global_distance_1,
            m_af_global_distance_2,
            m_af_global_distance_3,
            m_af_global_distance_4,
            m_af_global_distance_5,
            m_af_global_distance_6,
            m_af_global_distance_7,
            m_af_global_distance_8,
            m_af_global_distance_9,
            m_af_global_direction_adherence_0,
            m_af_global_direction_adherence_1,
            m_af_global_direction_adherence_2,
            m_af_global_direction_adherence_3,
            m_af_global_direction_adherence_4,
            m_af_global_direction_adherence_5,
            m_af_global_direction_adherence_6,
            m_af_global_direction_adherence_7,
            m_af_global_direction_adherence_8,
            m_af_global_direction_adherence_9,
            branch_level1,
            branch_level2,
            leaf_group1,
            leaf_group2,
            m_af_frond_ripple_distance_0,
            m_af_frond_ripple_distance_1,
            m_af_frond_ripple_distance_2,
            m_af_frond_ripple_distance_3,
            m_af_frond_ripple_distance_4,
            m_af_frond_ripple_distance_5,
            m_af_frond_ripple_distance_6,
            m_af_frond_ripple_distance_7,
            m_af_frond_ripple_distance_8,
            m_af_frond_ripple_distance_9,
            m_f_frond_ripple_tile,
            m_f_frond_ripple_lighting_scalar,
            m_f_rolling_noise_size,
            m_f_rolling_noise_twist,
            m_f_rolling_noise_turbulence,
            m_f_rolling_noise_period,
            m_f_rolling_noise_speed,
            m_f_rolling_branch_field_min,
            m_f_rolling_branch_lighting_adjust,
            m_f_rolling_branch_vertical_offset,
            m_f_rolling_leaf_ripple_min,
            m_f_rolling_leaf_tumble_min,
            branch_wind_anchor0,
            branch_wind_anchor1,
            branch_wind_anchor2,
            m_f_max_branch_level1_length,
            global_wind,
            global_preserve_shape,
            branch_simple_1,
            branch_directional_1,
            branch_directional_frond_1,
            branch_turbulence_1,
            branch_whip_1,
            branch_osc_complex_1,
            branch_simple_2,
            branch_directional_2,
            branch_directional_frond_2,
            branch_turbulence_2,
            branch_whip_2,
            branch_osc_complex_2,
            leaf_ripple_vertex_normal_1,
            leaf_ripple_computed_1,
            leaf_tumble_1,
            leaf_twitch_1,
            leaf_occlusion_1,
            leaf_ripple_vertex_normal_2,
            leaf_ripple_computed_2,
            leaf_tumble_2,
            leaf_twitch_2,
            leaf_occlusion_2,
            frond_ripple_one_sided,
            frond_ripple_two_sided,
            frond_ripple_adjust_lighting,
            rolling,
        })
    }
}
