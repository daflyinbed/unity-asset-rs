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

impl crate::ToValue for SpeedTreeWindConfig8 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_fStrengthResponse".to_string(),
                self.m_f_strength_response.to_value(),
            )),
            Some((
                "m_fDirectionResponse".to_string(),
                self.m_f_direction_response.to_value(),
            )),
            Some((
                "m_fGustFrequency".to_string(),
                self.m_f_gust_frequency.to_value(),
            )),
            Some((
                "m_fGustStrengthMin".to_string(),
                self.m_f_gust_strength_min.to_value(),
            )),
            Some((
                "m_fGustStrengthMax".to_string(),
                self.m_f_gust_strength_max.to_value(),
            )),
            Some((
                "m_fGustDurationMin".to_string(),
                self.m_f_gust_duration_min.to_value(),
            )),
            Some((
                "m_fGustDurationMax".to_string(),
                self.m_f_gust_duration_max.to_value(),
            )),
            Some((
                "m_fGustRiseScalar".to_string(),
                self.m_f_gust_rise_scalar.to_value(),
            )),
            Some((
                "m_fGustFallScalar".to_string(),
                self.m_f_gust_fall_scalar.to_value(),
            )),
            Some((
                "m_fAnchorOffset".to_string(),
                self.m_f_anchor_offset.to_value(),
            )),
            Some((
                "m_fAnchorDistanceScale".to_string(),
                self.m_f_anchor_distance_scale.to_value(),
            )),
            Some(("Oscillation0_0".to_string(), self.oscillation0_0.to_value())),
            Some(("Oscillation0_1".to_string(), self.oscillation0_1.to_value())),
            Some(("Oscillation0_2".to_string(), self.oscillation0_2.to_value())),
            Some(("Oscillation0_3".to_string(), self.oscillation0_3.to_value())),
            Some(("Oscillation0_4".to_string(), self.oscillation0_4.to_value())),
            Some(("Oscillation0_5".to_string(), self.oscillation0_5.to_value())),
            Some(("Oscillation0_6".to_string(), self.oscillation0_6.to_value())),
            Some(("Oscillation0_7".to_string(), self.oscillation0_7.to_value())),
            Some(("Oscillation0_8".to_string(), self.oscillation0_8.to_value())),
            Some(("Oscillation0_9".to_string(), self.oscillation0_9.to_value())),
            Some(("Oscillation1_0".to_string(), self.oscillation1_0.to_value())),
            Some(("Oscillation1_1".to_string(), self.oscillation1_1.to_value())),
            Some(("Oscillation1_2".to_string(), self.oscillation1_2.to_value())),
            Some(("Oscillation1_3".to_string(), self.oscillation1_3.to_value())),
            Some(("Oscillation1_4".to_string(), self.oscillation1_4.to_value())),
            Some(("Oscillation1_5".to_string(), self.oscillation1_5.to_value())),
            Some(("Oscillation1_6".to_string(), self.oscillation1_6.to_value())),
            Some(("Oscillation1_7".to_string(), self.oscillation1_7.to_value())),
            Some(("Oscillation1_8".to_string(), self.oscillation1_8.to_value())),
            Some(("Oscillation1_9".to_string(), self.oscillation1_9.to_value())),
            Some(("Oscillation2_0".to_string(), self.oscillation2_0.to_value())),
            Some(("Oscillation2_1".to_string(), self.oscillation2_1.to_value())),
            Some(("Oscillation2_2".to_string(), self.oscillation2_2.to_value())),
            Some(("Oscillation2_3".to_string(), self.oscillation2_3.to_value())),
            Some(("Oscillation2_4".to_string(), self.oscillation2_4.to_value())),
            Some(("Oscillation2_5".to_string(), self.oscillation2_5.to_value())),
            Some(("Oscillation2_6".to_string(), self.oscillation2_6.to_value())),
            Some(("Oscillation2_7".to_string(), self.oscillation2_7.to_value())),
            Some(("Oscillation2_8".to_string(), self.oscillation2_8.to_value())),
            Some(("Oscillation2_9".to_string(), self.oscillation2_9.to_value())),
            Some(("Oscillation3_0".to_string(), self.oscillation3_0.to_value())),
            Some(("Oscillation3_1".to_string(), self.oscillation3_1.to_value())),
            Some(("Oscillation3_2".to_string(), self.oscillation3_2.to_value())),
            Some(("Oscillation3_3".to_string(), self.oscillation3_3.to_value())),
            Some(("Oscillation3_4".to_string(), self.oscillation3_4.to_value())),
            Some(("Oscillation3_5".to_string(), self.oscillation3_5.to_value())),
            Some(("Oscillation3_6".to_string(), self.oscillation3_6.to_value())),
            Some(("Oscillation3_7".to_string(), self.oscillation3_7.to_value())),
            Some(("Oscillation3_8".to_string(), self.oscillation3_8.to_value())),
            Some(("Oscillation3_9".to_string(), self.oscillation3_9.to_value())),
            Some(("Oscillation4_0".to_string(), self.oscillation4_0.to_value())),
            Some(("Oscillation4_1".to_string(), self.oscillation4_1.to_value())),
            Some(("Oscillation4_2".to_string(), self.oscillation4_2.to_value())),
            Some(("Oscillation4_3".to_string(), self.oscillation4_3.to_value())),
            Some(("Oscillation4_4".to_string(), self.oscillation4_4.to_value())),
            Some(("Oscillation4_5".to_string(), self.oscillation4_5.to_value())),
            Some(("Oscillation4_6".to_string(), self.oscillation4_6.to_value())),
            Some(("Oscillation4_7".to_string(), self.oscillation4_7.to_value())),
            Some(("Oscillation4_8".to_string(), self.oscillation4_8.to_value())),
            Some(("Oscillation4_9".to_string(), self.oscillation4_9.to_value())),
            Some(("Oscillation5_0".to_string(), self.oscillation5_0.to_value())),
            Some(("Oscillation5_1".to_string(), self.oscillation5_1.to_value())),
            Some(("Oscillation5_2".to_string(), self.oscillation5_2.to_value())),
            Some(("Oscillation5_3".to_string(), self.oscillation5_3.to_value())),
            Some(("Oscillation5_4".to_string(), self.oscillation5_4.to_value())),
            Some(("Oscillation5_5".to_string(), self.oscillation5_5.to_value())),
            Some(("Oscillation5_6".to_string(), self.oscillation5_6.to_value())),
            Some(("Oscillation5_7".to_string(), self.oscillation5_7.to_value())),
            Some(("Oscillation5_8".to_string(), self.oscillation5_8.to_value())),
            Some(("Oscillation5_9".to_string(), self.oscillation5_9.to_value())),
            Some(("Oscillation6_0".to_string(), self.oscillation6_0.to_value())),
            Some(("Oscillation6_1".to_string(), self.oscillation6_1.to_value())),
            Some(("Oscillation6_2".to_string(), self.oscillation6_2.to_value())),
            Some(("Oscillation6_3".to_string(), self.oscillation6_3.to_value())),
            Some(("Oscillation6_4".to_string(), self.oscillation6_4.to_value())),
            Some(("Oscillation6_5".to_string(), self.oscillation6_5.to_value())),
            Some(("Oscillation6_6".to_string(), self.oscillation6_6.to_value())),
            Some(("Oscillation6_7".to_string(), self.oscillation6_7.to_value())),
            Some(("Oscillation6_8".to_string(), self.oscillation6_8.to_value())),
            Some(("Oscillation6_9".to_string(), self.oscillation6_9.to_value())),
            Some(("Oscillation7_0".to_string(), self.oscillation7_0.to_value())),
            Some(("Oscillation7_1".to_string(), self.oscillation7_1.to_value())),
            Some(("Oscillation7_2".to_string(), self.oscillation7_2.to_value())),
            Some(("Oscillation7_3".to_string(), self.oscillation7_3.to_value())),
            Some(("Oscillation7_4".to_string(), self.oscillation7_4.to_value())),
            Some(("Oscillation7_5".to_string(), self.oscillation7_5.to_value())),
            Some(("Oscillation7_6".to_string(), self.oscillation7_6.to_value())),
            Some(("Oscillation7_7".to_string(), self.oscillation7_7.to_value())),
            Some(("Oscillation7_8".to_string(), self.oscillation7_8.to_value())),
            Some(("Oscillation7_9".to_string(), self.oscillation7_9.to_value())),
            Some(("Oscillation8_0".to_string(), self.oscillation8_0.to_value())),
            Some(("Oscillation8_1".to_string(), self.oscillation8_1.to_value())),
            Some(("Oscillation8_2".to_string(), self.oscillation8_2.to_value())),
            Some(("Oscillation8_3".to_string(), self.oscillation8_3.to_value())),
            Some(("Oscillation8_4".to_string(), self.oscillation8_4.to_value())),
            Some(("Oscillation8_5".to_string(), self.oscillation8_5.to_value())),
            Some(("Oscillation8_6".to_string(), self.oscillation8_6.to_value())),
            Some(("Oscillation8_7".to_string(), self.oscillation8_7.to_value())),
            Some(("Oscillation8_8".to_string(), self.oscillation8_8.to_value())),
            Some(("Oscillation8_9".to_string(), self.oscillation8_9.to_value())),
            Some(("Oscillation9_0".to_string(), self.oscillation9_0.to_value())),
            Some(("Oscillation9_1".to_string(), self.oscillation9_1.to_value())),
            Some(("Oscillation9_2".to_string(), self.oscillation9_2.to_value())),
            Some(("Oscillation9_3".to_string(), self.oscillation9_3.to_value())),
            Some(("Oscillation9_4".to_string(), self.oscillation9_4.to_value())),
            Some(("Oscillation9_5".to_string(), self.oscillation9_5.to_value())),
            Some(("Oscillation9_6".to_string(), self.oscillation9_6.to_value())),
            Some(("Oscillation9_7".to_string(), self.oscillation9_7.to_value())),
            Some(("Oscillation9_8".to_string(), self.oscillation9_8.to_value())),
            Some(("Oscillation9_9".to_string(), self.oscillation9_9.to_value())),
            Some((
                "m_fGlobalHeight".to_string(),
                self.m_f_global_height.to_value(),
            )),
            Some((
                "m_fGlobalHeightExponent".to_string(),
                self.m_f_global_height_exponent.to_value(),
            )),
            Some((
                "m_afGlobalDistance_0".to_string(),
                self.m_af_global_distance_0.to_value(),
            )),
            Some((
                "m_afGlobalDistance_1".to_string(),
                self.m_af_global_distance_1.to_value(),
            )),
            Some((
                "m_afGlobalDistance_2".to_string(),
                self.m_af_global_distance_2.to_value(),
            )),
            Some((
                "m_afGlobalDistance_3".to_string(),
                self.m_af_global_distance_3.to_value(),
            )),
            Some((
                "m_afGlobalDistance_4".to_string(),
                self.m_af_global_distance_4.to_value(),
            )),
            Some((
                "m_afGlobalDistance_5".to_string(),
                self.m_af_global_distance_5.to_value(),
            )),
            Some((
                "m_afGlobalDistance_6".to_string(),
                self.m_af_global_distance_6.to_value(),
            )),
            Some((
                "m_afGlobalDistance_7".to_string(),
                self.m_af_global_distance_7.to_value(),
            )),
            Some((
                "m_afGlobalDistance_8".to_string(),
                self.m_af_global_distance_8.to_value(),
            )),
            Some((
                "m_afGlobalDistance_9".to_string(),
                self.m_af_global_distance_9.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_0".to_string(),
                self.m_af_global_direction_adherence_0.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_1".to_string(),
                self.m_af_global_direction_adherence_1.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_2".to_string(),
                self.m_af_global_direction_adherence_2.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_3".to_string(),
                self.m_af_global_direction_adherence_3.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_4".to_string(),
                self.m_af_global_direction_adherence_4.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_5".to_string(),
                self.m_af_global_direction_adherence_5.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_6".to_string(),
                self.m_af_global_direction_adherence_6.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_7".to_string(),
                self.m_af_global_direction_adherence_7.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_8".to_string(),
                self.m_af_global_direction_adherence_8.to_value(),
            )),
            Some((
                "m_afGlobalDirectionAdherence_9".to_string(),
                self.m_af_global_direction_adherence_9.to_value(),
            )),
            Some(("BranchLevel1".to_string(), self.branch_level1.to_value())),
            Some(("BranchLevel2".to_string(), self.branch_level2.to_value())),
            Some(("LeafGroup1".to_string(), self.leaf_group1.to_value())),
            Some(("LeafGroup2".to_string(), self.leaf_group2.to_value())),
            Some((
                "m_afFrondRippleDistance_0".to_string(),
                self.m_af_frond_ripple_distance_0.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_1".to_string(),
                self.m_af_frond_ripple_distance_1.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_2".to_string(),
                self.m_af_frond_ripple_distance_2.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_3".to_string(),
                self.m_af_frond_ripple_distance_3.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_4".to_string(),
                self.m_af_frond_ripple_distance_4.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_5".to_string(),
                self.m_af_frond_ripple_distance_5.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_6".to_string(),
                self.m_af_frond_ripple_distance_6.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_7".to_string(),
                self.m_af_frond_ripple_distance_7.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_8".to_string(),
                self.m_af_frond_ripple_distance_8.to_value(),
            )),
            Some((
                "m_afFrondRippleDistance_9".to_string(),
                self.m_af_frond_ripple_distance_9.to_value(),
            )),
            Some((
                "m_fFrondRippleTile".to_string(),
                self.m_f_frond_ripple_tile.to_value(),
            )),
            Some((
                "m_fFrondRippleLightingScalar".to_string(),
                self.m_f_frond_ripple_lighting_scalar.to_value(),
            )),
            Some((
                "m_fRollingNoiseSize".to_string(),
                self.m_f_rolling_noise_size.to_value(),
            )),
            Some((
                "m_fRollingNoiseTwist".to_string(),
                self.m_f_rolling_noise_twist.to_value(),
            )),
            Some((
                "m_fRollingNoiseTurbulence".to_string(),
                self.m_f_rolling_noise_turbulence.to_value(),
            )),
            Some((
                "m_fRollingNoisePeriod".to_string(),
                self.m_f_rolling_noise_period.to_value(),
            )),
            Some((
                "m_fRollingNoiseSpeed".to_string(),
                self.m_f_rolling_noise_speed.to_value(),
            )),
            Some((
                "m_fRollingBranchFieldMin".to_string(),
                self.m_f_rolling_branch_field_min.to_value(),
            )),
            Some((
                "m_fRollingBranchLightingAdjust".to_string(),
                self.m_f_rolling_branch_lighting_adjust.to_value(),
            )),
            Some((
                "m_fRollingBranchVerticalOffset".to_string(),
                self.m_f_rolling_branch_vertical_offset.to_value(),
            )),
            Some((
                "m_fRollingLeafRippleMin".to_string(),
                self.m_f_rolling_leaf_ripple_min.to_value(),
            )),
            Some((
                "m_fRollingLeafTumbleMin".to_string(),
                self.m_f_rolling_leaf_tumble_min.to_value(),
            )),
            Some((
                "BranchWindAnchor0".to_string(),
                self.branch_wind_anchor0.to_value(),
            )),
            Some((
                "BranchWindAnchor1".to_string(),
                self.branch_wind_anchor1.to_value(),
            )),
            Some((
                "BranchWindAnchor2".to_string(),
                self.branch_wind_anchor2.to_value(),
            )),
            Some((
                "m_fMaxBranchLevel1Length".to_string(),
                self.m_f_max_branch_level1_length.to_value(),
            )),
            Some(("GLOBAL_WIND".to_string(), self.global_wind.to_value())),
            Some((
                "GLOBAL_PRESERVE_SHAPE".to_string(),
                self.global_preserve_shape.to_value(),
            )),
            Some((
                "BRANCH_SIMPLE_1".to_string(),
                self.branch_simple_1.to_value(),
            )),
            Some((
                "BRANCH_DIRECTIONAL_1".to_string(),
                self.branch_directional_1.to_value(),
            )),
            Some((
                "BRANCH_DIRECTIONAL_FROND_1".to_string(),
                self.branch_directional_frond_1.to_value(),
            )),
            Some((
                "BRANCH_TURBULENCE_1".to_string(),
                self.branch_turbulence_1.to_value(),
            )),
            Some(("BRANCH_WHIP_1".to_string(), self.branch_whip_1.to_value())),
            Some((
                "BRANCH_OSC_COMPLEX_1".to_string(),
                self.branch_osc_complex_1.to_value(),
            )),
            Some((
                "BRANCH_SIMPLE_2".to_string(),
                self.branch_simple_2.to_value(),
            )),
            Some((
                "BRANCH_DIRECTIONAL_2".to_string(),
                self.branch_directional_2.to_value(),
            )),
            Some((
                "BRANCH_DIRECTIONAL_FROND_2".to_string(),
                self.branch_directional_frond_2.to_value(),
            )),
            Some((
                "BRANCH_TURBULENCE_2".to_string(),
                self.branch_turbulence_2.to_value(),
            )),
            Some(("BRANCH_WHIP_2".to_string(), self.branch_whip_2.to_value())),
            Some((
                "BRANCH_OSC_COMPLEX_2".to_string(),
                self.branch_osc_complex_2.to_value(),
            )),
            Some((
                "LEAF_RIPPLE_VERTEX_NORMAL_1".to_string(),
                self.leaf_ripple_vertex_normal_1.to_value(),
            )),
            Some((
                "LEAF_RIPPLE_COMPUTED_1".to_string(),
                self.leaf_ripple_computed_1.to_value(),
            )),
            Some(("LEAF_TUMBLE_1".to_string(), self.leaf_tumble_1.to_value())),
            Some(("LEAF_TWITCH_1".to_string(), self.leaf_twitch_1.to_value())),
            Some((
                "LEAF_OCCLUSION_1".to_string(),
                self.leaf_occlusion_1.to_value(),
            )),
            Some((
                "LEAF_RIPPLE_VERTEX_NORMAL_2".to_string(),
                self.leaf_ripple_vertex_normal_2.to_value(),
            )),
            Some((
                "LEAF_RIPPLE_COMPUTED_2".to_string(),
                self.leaf_ripple_computed_2.to_value(),
            )),
            Some(("LEAF_TUMBLE_2".to_string(), self.leaf_tumble_2.to_value())),
            Some(("LEAF_TWITCH_2".to_string(), self.leaf_twitch_2.to_value())),
            Some((
                "LEAF_OCCLUSION_2".to_string(),
                self.leaf_occlusion_2.to_value(),
            )),
            Some((
                "FROND_RIPPLE_ONE_SIDED".to_string(),
                self.frond_ripple_one_sided.to_value(),
            )),
            Some((
                "FROND_RIPPLE_TWO_SIDED".to_string(),
                self.frond_ripple_two_sided.to_value(),
            )),
            Some((
                "FROND_RIPPLE_ADJUST_LIGHTING".to_string(),
                self.frond_ripple_adjust_lighting.to_value(),
            )),
            Some(("ROLLING".to_string(), self.rolling.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_fStrengthResponse" => {
                    (self.m_f_strength_response).inspect_value(path, offset, limit)
                }
                "m_fDirectionResponse" => {
                    (self.m_f_direction_response).inspect_value(path, offset, limit)
                }
                "m_fGustFrequency" => (self.m_f_gust_frequency).inspect_value(path, offset, limit),
                "m_fGustStrengthMin" => {
                    (self.m_f_gust_strength_min).inspect_value(path, offset, limit)
                }
                "m_fGustStrengthMax" => {
                    (self.m_f_gust_strength_max).inspect_value(path, offset, limit)
                }
                "m_fGustDurationMin" => {
                    (self.m_f_gust_duration_min).inspect_value(path, offset, limit)
                }
                "m_fGustDurationMax" => {
                    (self.m_f_gust_duration_max).inspect_value(path, offset, limit)
                }
                "m_fGustRiseScalar" => {
                    (self.m_f_gust_rise_scalar).inspect_value(path, offset, limit)
                }
                "m_fGustFallScalar" => {
                    (self.m_f_gust_fall_scalar).inspect_value(path, offset, limit)
                }
                "m_fAnchorOffset" => (self.m_f_anchor_offset).inspect_value(path, offset, limit),
                "m_fAnchorDistanceScale" => {
                    (self.m_f_anchor_distance_scale).inspect_value(path, offset, limit)
                }
                "Oscillation0_0" => (self.oscillation0_0).inspect_value(path, offset, limit),
                "Oscillation0_1" => (self.oscillation0_1).inspect_value(path, offset, limit),
                "Oscillation0_2" => (self.oscillation0_2).inspect_value(path, offset, limit),
                "Oscillation0_3" => (self.oscillation0_3).inspect_value(path, offset, limit),
                "Oscillation0_4" => (self.oscillation0_4).inspect_value(path, offset, limit),
                "Oscillation0_5" => (self.oscillation0_5).inspect_value(path, offset, limit),
                "Oscillation0_6" => (self.oscillation0_6).inspect_value(path, offset, limit),
                "Oscillation0_7" => (self.oscillation0_7).inspect_value(path, offset, limit),
                "Oscillation0_8" => (self.oscillation0_8).inspect_value(path, offset, limit),
                "Oscillation0_9" => (self.oscillation0_9).inspect_value(path, offset, limit),
                "Oscillation1_0" => (self.oscillation1_0).inspect_value(path, offset, limit),
                "Oscillation1_1" => (self.oscillation1_1).inspect_value(path, offset, limit),
                "Oscillation1_2" => (self.oscillation1_2).inspect_value(path, offset, limit),
                "Oscillation1_3" => (self.oscillation1_3).inspect_value(path, offset, limit),
                "Oscillation1_4" => (self.oscillation1_4).inspect_value(path, offset, limit),
                "Oscillation1_5" => (self.oscillation1_5).inspect_value(path, offset, limit),
                "Oscillation1_6" => (self.oscillation1_6).inspect_value(path, offset, limit),
                "Oscillation1_7" => (self.oscillation1_7).inspect_value(path, offset, limit),
                "Oscillation1_8" => (self.oscillation1_8).inspect_value(path, offset, limit),
                "Oscillation1_9" => (self.oscillation1_9).inspect_value(path, offset, limit),
                "Oscillation2_0" => (self.oscillation2_0).inspect_value(path, offset, limit),
                "Oscillation2_1" => (self.oscillation2_1).inspect_value(path, offset, limit),
                "Oscillation2_2" => (self.oscillation2_2).inspect_value(path, offset, limit),
                "Oscillation2_3" => (self.oscillation2_3).inspect_value(path, offset, limit),
                "Oscillation2_4" => (self.oscillation2_4).inspect_value(path, offset, limit),
                "Oscillation2_5" => (self.oscillation2_5).inspect_value(path, offset, limit),
                "Oscillation2_6" => (self.oscillation2_6).inspect_value(path, offset, limit),
                "Oscillation2_7" => (self.oscillation2_7).inspect_value(path, offset, limit),
                "Oscillation2_8" => (self.oscillation2_8).inspect_value(path, offset, limit),
                "Oscillation2_9" => (self.oscillation2_9).inspect_value(path, offset, limit),
                "Oscillation3_0" => (self.oscillation3_0).inspect_value(path, offset, limit),
                "Oscillation3_1" => (self.oscillation3_1).inspect_value(path, offset, limit),
                "Oscillation3_2" => (self.oscillation3_2).inspect_value(path, offset, limit),
                "Oscillation3_3" => (self.oscillation3_3).inspect_value(path, offset, limit),
                "Oscillation3_4" => (self.oscillation3_4).inspect_value(path, offset, limit),
                "Oscillation3_5" => (self.oscillation3_5).inspect_value(path, offset, limit),
                "Oscillation3_6" => (self.oscillation3_6).inspect_value(path, offset, limit),
                "Oscillation3_7" => (self.oscillation3_7).inspect_value(path, offset, limit),
                "Oscillation3_8" => (self.oscillation3_8).inspect_value(path, offset, limit),
                "Oscillation3_9" => (self.oscillation3_9).inspect_value(path, offset, limit),
                "Oscillation4_0" => (self.oscillation4_0).inspect_value(path, offset, limit),
                "Oscillation4_1" => (self.oscillation4_1).inspect_value(path, offset, limit),
                "Oscillation4_2" => (self.oscillation4_2).inspect_value(path, offset, limit),
                "Oscillation4_3" => (self.oscillation4_3).inspect_value(path, offset, limit),
                "Oscillation4_4" => (self.oscillation4_4).inspect_value(path, offset, limit),
                "Oscillation4_5" => (self.oscillation4_5).inspect_value(path, offset, limit),
                "Oscillation4_6" => (self.oscillation4_6).inspect_value(path, offset, limit),
                "Oscillation4_7" => (self.oscillation4_7).inspect_value(path, offset, limit),
                "Oscillation4_8" => (self.oscillation4_8).inspect_value(path, offset, limit),
                "Oscillation4_9" => (self.oscillation4_9).inspect_value(path, offset, limit),
                "Oscillation5_0" => (self.oscillation5_0).inspect_value(path, offset, limit),
                "Oscillation5_1" => (self.oscillation5_1).inspect_value(path, offset, limit),
                "Oscillation5_2" => (self.oscillation5_2).inspect_value(path, offset, limit),
                "Oscillation5_3" => (self.oscillation5_3).inspect_value(path, offset, limit),
                "Oscillation5_4" => (self.oscillation5_4).inspect_value(path, offset, limit),
                "Oscillation5_5" => (self.oscillation5_5).inspect_value(path, offset, limit),
                "Oscillation5_6" => (self.oscillation5_6).inspect_value(path, offset, limit),
                "Oscillation5_7" => (self.oscillation5_7).inspect_value(path, offset, limit),
                "Oscillation5_8" => (self.oscillation5_8).inspect_value(path, offset, limit),
                "Oscillation5_9" => (self.oscillation5_9).inspect_value(path, offset, limit),
                "Oscillation6_0" => (self.oscillation6_0).inspect_value(path, offset, limit),
                "Oscillation6_1" => (self.oscillation6_1).inspect_value(path, offset, limit),
                "Oscillation6_2" => (self.oscillation6_2).inspect_value(path, offset, limit),
                "Oscillation6_3" => (self.oscillation6_3).inspect_value(path, offset, limit),
                "Oscillation6_4" => (self.oscillation6_4).inspect_value(path, offset, limit),
                "Oscillation6_5" => (self.oscillation6_5).inspect_value(path, offset, limit),
                "Oscillation6_6" => (self.oscillation6_6).inspect_value(path, offset, limit),
                "Oscillation6_7" => (self.oscillation6_7).inspect_value(path, offset, limit),
                "Oscillation6_8" => (self.oscillation6_8).inspect_value(path, offset, limit),
                "Oscillation6_9" => (self.oscillation6_9).inspect_value(path, offset, limit),
                "Oscillation7_0" => (self.oscillation7_0).inspect_value(path, offset, limit),
                "Oscillation7_1" => (self.oscillation7_1).inspect_value(path, offset, limit),
                "Oscillation7_2" => (self.oscillation7_2).inspect_value(path, offset, limit),
                "Oscillation7_3" => (self.oscillation7_3).inspect_value(path, offset, limit),
                "Oscillation7_4" => (self.oscillation7_4).inspect_value(path, offset, limit),
                "Oscillation7_5" => (self.oscillation7_5).inspect_value(path, offset, limit),
                "Oscillation7_6" => (self.oscillation7_6).inspect_value(path, offset, limit),
                "Oscillation7_7" => (self.oscillation7_7).inspect_value(path, offset, limit),
                "Oscillation7_8" => (self.oscillation7_8).inspect_value(path, offset, limit),
                "Oscillation7_9" => (self.oscillation7_9).inspect_value(path, offset, limit),
                "Oscillation8_0" => (self.oscillation8_0).inspect_value(path, offset, limit),
                "Oscillation8_1" => (self.oscillation8_1).inspect_value(path, offset, limit),
                "Oscillation8_2" => (self.oscillation8_2).inspect_value(path, offset, limit),
                "Oscillation8_3" => (self.oscillation8_3).inspect_value(path, offset, limit),
                "Oscillation8_4" => (self.oscillation8_4).inspect_value(path, offset, limit),
                "Oscillation8_5" => (self.oscillation8_5).inspect_value(path, offset, limit),
                "Oscillation8_6" => (self.oscillation8_6).inspect_value(path, offset, limit),
                "Oscillation8_7" => (self.oscillation8_7).inspect_value(path, offset, limit),
                "Oscillation8_8" => (self.oscillation8_8).inspect_value(path, offset, limit),
                "Oscillation8_9" => (self.oscillation8_9).inspect_value(path, offset, limit),
                "Oscillation9_0" => (self.oscillation9_0).inspect_value(path, offset, limit),
                "Oscillation9_1" => (self.oscillation9_1).inspect_value(path, offset, limit),
                "Oscillation9_2" => (self.oscillation9_2).inspect_value(path, offset, limit),
                "Oscillation9_3" => (self.oscillation9_3).inspect_value(path, offset, limit),
                "Oscillation9_4" => (self.oscillation9_4).inspect_value(path, offset, limit),
                "Oscillation9_5" => (self.oscillation9_5).inspect_value(path, offset, limit),
                "Oscillation9_6" => (self.oscillation9_6).inspect_value(path, offset, limit),
                "Oscillation9_7" => (self.oscillation9_7).inspect_value(path, offset, limit),
                "Oscillation9_8" => (self.oscillation9_8).inspect_value(path, offset, limit),
                "Oscillation9_9" => (self.oscillation9_9).inspect_value(path, offset, limit),
                "m_fGlobalHeight" => (self.m_f_global_height).inspect_value(path, offset, limit),
                "m_fGlobalHeightExponent" => {
                    (self.m_f_global_height_exponent).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_0" => {
                    (self.m_af_global_distance_0).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_1" => {
                    (self.m_af_global_distance_1).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_2" => {
                    (self.m_af_global_distance_2).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_3" => {
                    (self.m_af_global_distance_3).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_4" => {
                    (self.m_af_global_distance_4).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_5" => {
                    (self.m_af_global_distance_5).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_6" => {
                    (self.m_af_global_distance_6).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_7" => {
                    (self.m_af_global_distance_7).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_8" => {
                    (self.m_af_global_distance_8).inspect_value(path, offset, limit)
                }
                "m_afGlobalDistance_9" => {
                    (self.m_af_global_distance_9).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_0" => {
                    (self.m_af_global_direction_adherence_0).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_1" => {
                    (self.m_af_global_direction_adherence_1).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_2" => {
                    (self.m_af_global_direction_adherence_2).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_3" => {
                    (self.m_af_global_direction_adherence_3).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_4" => {
                    (self.m_af_global_direction_adherence_4).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_5" => {
                    (self.m_af_global_direction_adherence_5).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_6" => {
                    (self.m_af_global_direction_adherence_6).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_7" => {
                    (self.m_af_global_direction_adherence_7).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_8" => {
                    (self.m_af_global_direction_adherence_8).inspect_value(path, offset, limit)
                }
                "m_afGlobalDirectionAdherence_9" => {
                    (self.m_af_global_direction_adherence_9).inspect_value(path, offset, limit)
                }
                "BranchLevel1" => (self.branch_level1).inspect_value(path, offset, limit),
                "BranchLevel2" => (self.branch_level2).inspect_value(path, offset, limit),
                "LeafGroup1" => (self.leaf_group1).inspect_value(path, offset, limit),
                "LeafGroup2" => (self.leaf_group2).inspect_value(path, offset, limit),
                "m_afFrondRippleDistance_0" => {
                    (self.m_af_frond_ripple_distance_0).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_1" => {
                    (self.m_af_frond_ripple_distance_1).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_2" => {
                    (self.m_af_frond_ripple_distance_2).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_3" => {
                    (self.m_af_frond_ripple_distance_3).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_4" => {
                    (self.m_af_frond_ripple_distance_4).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_5" => {
                    (self.m_af_frond_ripple_distance_5).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_6" => {
                    (self.m_af_frond_ripple_distance_6).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_7" => {
                    (self.m_af_frond_ripple_distance_7).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_8" => {
                    (self.m_af_frond_ripple_distance_8).inspect_value(path, offset, limit)
                }
                "m_afFrondRippleDistance_9" => {
                    (self.m_af_frond_ripple_distance_9).inspect_value(path, offset, limit)
                }
                "m_fFrondRippleTile" => {
                    (self.m_f_frond_ripple_tile).inspect_value(path, offset, limit)
                }
                "m_fFrondRippleLightingScalar" => {
                    (self.m_f_frond_ripple_lighting_scalar).inspect_value(path, offset, limit)
                }
                "m_fRollingNoiseSize" => {
                    (self.m_f_rolling_noise_size).inspect_value(path, offset, limit)
                }
                "m_fRollingNoiseTwist" => {
                    (self.m_f_rolling_noise_twist).inspect_value(path, offset, limit)
                }
                "m_fRollingNoiseTurbulence" => {
                    (self.m_f_rolling_noise_turbulence).inspect_value(path, offset, limit)
                }
                "m_fRollingNoisePeriod" => {
                    (self.m_f_rolling_noise_period).inspect_value(path, offset, limit)
                }
                "m_fRollingNoiseSpeed" => {
                    (self.m_f_rolling_noise_speed).inspect_value(path, offset, limit)
                }
                "m_fRollingBranchFieldMin" => {
                    (self.m_f_rolling_branch_field_min).inspect_value(path, offset, limit)
                }
                "m_fRollingBranchLightingAdjust" => {
                    (self.m_f_rolling_branch_lighting_adjust).inspect_value(path, offset, limit)
                }
                "m_fRollingBranchVerticalOffset" => {
                    (self.m_f_rolling_branch_vertical_offset).inspect_value(path, offset, limit)
                }
                "m_fRollingLeafRippleMin" => {
                    (self.m_f_rolling_leaf_ripple_min).inspect_value(path, offset, limit)
                }
                "m_fRollingLeafTumbleMin" => {
                    (self.m_f_rolling_leaf_tumble_min).inspect_value(path, offset, limit)
                }
                "BranchWindAnchor0" => {
                    (self.branch_wind_anchor0).inspect_value(path, offset, limit)
                }
                "BranchWindAnchor1" => {
                    (self.branch_wind_anchor1).inspect_value(path, offset, limit)
                }
                "BranchWindAnchor2" => {
                    (self.branch_wind_anchor2).inspect_value(path, offset, limit)
                }
                "m_fMaxBranchLevel1Length" => {
                    (self.m_f_max_branch_level1_length).inspect_value(path, offset, limit)
                }
                "GLOBAL_WIND" => (self.global_wind).inspect_value(path, offset, limit),
                "GLOBAL_PRESERVE_SHAPE" => {
                    (self.global_preserve_shape).inspect_value(path, offset, limit)
                }
                "BRANCH_SIMPLE_1" => (self.branch_simple_1).inspect_value(path, offset, limit),
                "BRANCH_DIRECTIONAL_1" => {
                    (self.branch_directional_1).inspect_value(path, offset, limit)
                }
                "BRANCH_DIRECTIONAL_FROND_1" => {
                    (self.branch_directional_frond_1).inspect_value(path, offset, limit)
                }
                "BRANCH_TURBULENCE_1" => {
                    (self.branch_turbulence_1).inspect_value(path, offset, limit)
                }
                "BRANCH_WHIP_1" => (self.branch_whip_1).inspect_value(path, offset, limit),
                "BRANCH_OSC_COMPLEX_1" => {
                    (self.branch_osc_complex_1).inspect_value(path, offset, limit)
                }
                "BRANCH_SIMPLE_2" => (self.branch_simple_2).inspect_value(path, offset, limit),
                "BRANCH_DIRECTIONAL_2" => {
                    (self.branch_directional_2).inspect_value(path, offset, limit)
                }
                "BRANCH_DIRECTIONAL_FROND_2" => {
                    (self.branch_directional_frond_2).inspect_value(path, offset, limit)
                }
                "BRANCH_TURBULENCE_2" => {
                    (self.branch_turbulence_2).inspect_value(path, offset, limit)
                }
                "BRANCH_WHIP_2" => (self.branch_whip_2).inspect_value(path, offset, limit),
                "BRANCH_OSC_COMPLEX_2" => {
                    (self.branch_osc_complex_2).inspect_value(path, offset, limit)
                }
                "LEAF_RIPPLE_VERTEX_NORMAL_1" => {
                    (self.leaf_ripple_vertex_normal_1).inspect_value(path, offset, limit)
                }
                "LEAF_RIPPLE_COMPUTED_1" => {
                    (self.leaf_ripple_computed_1).inspect_value(path, offset, limit)
                }
                "LEAF_TUMBLE_1" => (self.leaf_tumble_1).inspect_value(path, offset, limit),
                "LEAF_TWITCH_1" => (self.leaf_twitch_1).inspect_value(path, offset, limit),
                "LEAF_OCCLUSION_1" => (self.leaf_occlusion_1).inspect_value(path, offset, limit),
                "LEAF_RIPPLE_VERTEX_NORMAL_2" => {
                    (self.leaf_ripple_vertex_normal_2).inspect_value(path, offset, limit)
                }
                "LEAF_RIPPLE_COMPUTED_2" => {
                    (self.leaf_ripple_computed_2).inspect_value(path, offset, limit)
                }
                "LEAF_TUMBLE_2" => (self.leaf_tumble_2).inspect_value(path, offset, limit),
                "LEAF_TWITCH_2" => (self.leaf_twitch_2).inspect_value(path, offset, limit),
                "LEAF_OCCLUSION_2" => (self.leaf_occlusion_2).inspect_value(path, offset, limit),
                "FROND_RIPPLE_ONE_SIDED" => {
                    (self.frond_ripple_one_sided).inspect_value(path, offset, limit)
                }
                "FROND_RIPPLE_TWO_SIDED" => {
                    (self.frond_ripple_two_sided).inspect_value(path, offset, limit)
                }
                "FROND_RIPPLE_ADJUST_LIGHTING" => {
                    (self.frond_ripple_adjust_lighting).inspect_value(path, offset, limit)
                }
                "ROLLING" => (self.rolling).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_f_strength_response).inspect_value(path, offset, limit))
                .map(|v| ("m_fStrengthResponse".to_string(), v)),
            ((self.m_f_direction_response).inspect_value(path, offset, limit))
                .map(|v| ("m_fDirectionResponse".to_string(), v)),
            ((self.m_f_gust_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustFrequency".to_string(), v)),
            ((self.m_f_gust_strength_min).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustStrengthMin".to_string(), v)),
            ((self.m_f_gust_strength_max).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustStrengthMax".to_string(), v)),
            ((self.m_f_gust_duration_min).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustDurationMin".to_string(), v)),
            ((self.m_f_gust_duration_max).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustDurationMax".to_string(), v)),
            ((self.m_f_gust_rise_scalar).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustRiseScalar".to_string(), v)),
            ((self.m_f_gust_fall_scalar).inspect_value(path, offset, limit))
                .map(|v| ("m_fGustFallScalar".to_string(), v)),
            ((self.m_f_anchor_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_fAnchorOffset".to_string(), v)),
            ((self.m_f_anchor_distance_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_fAnchorDistanceScale".to_string(), v)),
            ((self.oscillation0_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_0".to_string(), v)),
            ((self.oscillation0_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_1".to_string(), v)),
            ((self.oscillation0_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_2".to_string(), v)),
            ((self.oscillation0_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_3".to_string(), v)),
            ((self.oscillation0_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_4".to_string(), v)),
            ((self.oscillation0_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_5".to_string(), v)),
            ((self.oscillation0_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_6".to_string(), v)),
            ((self.oscillation0_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_7".to_string(), v)),
            ((self.oscillation0_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_8".to_string(), v)),
            ((self.oscillation0_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation0_9".to_string(), v)),
            ((self.oscillation1_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_0".to_string(), v)),
            ((self.oscillation1_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_1".to_string(), v)),
            ((self.oscillation1_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_2".to_string(), v)),
            ((self.oscillation1_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_3".to_string(), v)),
            ((self.oscillation1_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_4".to_string(), v)),
            ((self.oscillation1_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_5".to_string(), v)),
            ((self.oscillation1_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_6".to_string(), v)),
            ((self.oscillation1_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_7".to_string(), v)),
            ((self.oscillation1_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_8".to_string(), v)),
            ((self.oscillation1_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation1_9".to_string(), v)),
            ((self.oscillation2_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_0".to_string(), v)),
            ((self.oscillation2_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_1".to_string(), v)),
            ((self.oscillation2_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_2".to_string(), v)),
            ((self.oscillation2_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_3".to_string(), v)),
            ((self.oscillation2_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_4".to_string(), v)),
            ((self.oscillation2_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_5".to_string(), v)),
            ((self.oscillation2_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_6".to_string(), v)),
            ((self.oscillation2_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_7".to_string(), v)),
            ((self.oscillation2_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_8".to_string(), v)),
            ((self.oscillation2_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation2_9".to_string(), v)),
            ((self.oscillation3_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_0".to_string(), v)),
            ((self.oscillation3_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_1".to_string(), v)),
            ((self.oscillation3_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_2".to_string(), v)),
            ((self.oscillation3_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_3".to_string(), v)),
            ((self.oscillation3_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_4".to_string(), v)),
            ((self.oscillation3_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_5".to_string(), v)),
            ((self.oscillation3_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_6".to_string(), v)),
            ((self.oscillation3_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_7".to_string(), v)),
            ((self.oscillation3_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_8".to_string(), v)),
            ((self.oscillation3_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation3_9".to_string(), v)),
            ((self.oscillation4_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_0".to_string(), v)),
            ((self.oscillation4_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_1".to_string(), v)),
            ((self.oscillation4_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_2".to_string(), v)),
            ((self.oscillation4_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_3".to_string(), v)),
            ((self.oscillation4_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_4".to_string(), v)),
            ((self.oscillation4_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_5".to_string(), v)),
            ((self.oscillation4_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_6".to_string(), v)),
            ((self.oscillation4_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_7".to_string(), v)),
            ((self.oscillation4_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_8".to_string(), v)),
            ((self.oscillation4_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation4_9".to_string(), v)),
            ((self.oscillation5_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_0".to_string(), v)),
            ((self.oscillation5_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_1".to_string(), v)),
            ((self.oscillation5_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_2".to_string(), v)),
            ((self.oscillation5_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_3".to_string(), v)),
            ((self.oscillation5_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_4".to_string(), v)),
            ((self.oscillation5_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_5".to_string(), v)),
            ((self.oscillation5_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_6".to_string(), v)),
            ((self.oscillation5_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_7".to_string(), v)),
            ((self.oscillation5_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_8".to_string(), v)),
            ((self.oscillation5_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation5_9".to_string(), v)),
            ((self.oscillation6_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_0".to_string(), v)),
            ((self.oscillation6_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_1".to_string(), v)),
            ((self.oscillation6_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_2".to_string(), v)),
            ((self.oscillation6_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_3".to_string(), v)),
            ((self.oscillation6_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_4".to_string(), v)),
            ((self.oscillation6_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_5".to_string(), v)),
            ((self.oscillation6_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_6".to_string(), v)),
            ((self.oscillation6_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_7".to_string(), v)),
            ((self.oscillation6_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_8".to_string(), v)),
            ((self.oscillation6_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation6_9".to_string(), v)),
            ((self.oscillation7_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_0".to_string(), v)),
            ((self.oscillation7_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_1".to_string(), v)),
            ((self.oscillation7_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_2".to_string(), v)),
            ((self.oscillation7_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_3".to_string(), v)),
            ((self.oscillation7_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_4".to_string(), v)),
            ((self.oscillation7_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_5".to_string(), v)),
            ((self.oscillation7_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_6".to_string(), v)),
            ((self.oscillation7_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_7".to_string(), v)),
            ((self.oscillation7_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_8".to_string(), v)),
            ((self.oscillation7_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation7_9".to_string(), v)),
            ((self.oscillation8_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_0".to_string(), v)),
            ((self.oscillation8_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_1".to_string(), v)),
            ((self.oscillation8_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_2".to_string(), v)),
            ((self.oscillation8_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_3".to_string(), v)),
            ((self.oscillation8_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_4".to_string(), v)),
            ((self.oscillation8_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_5".to_string(), v)),
            ((self.oscillation8_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_6".to_string(), v)),
            ((self.oscillation8_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_7".to_string(), v)),
            ((self.oscillation8_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_8".to_string(), v)),
            ((self.oscillation8_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation8_9".to_string(), v)),
            ((self.oscillation9_0).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_0".to_string(), v)),
            ((self.oscillation9_1).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_1".to_string(), v)),
            ((self.oscillation9_2).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_2".to_string(), v)),
            ((self.oscillation9_3).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_3".to_string(), v)),
            ((self.oscillation9_4).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_4".to_string(), v)),
            ((self.oscillation9_5).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_5".to_string(), v)),
            ((self.oscillation9_6).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_6".to_string(), v)),
            ((self.oscillation9_7).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_7".to_string(), v)),
            ((self.oscillation9_8).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_8".to_string(), v)),
            ((self.oscillation9_9).inspect_value(path, offset, limit))
                .map(|v| ("Oscillation9_9".to_string(), v)),
            ((self.m_f_global_height).inspect_value(path, offset, limit))
                .map(|v| ("m_fGlobalHeight".to_string(), v)),
            ((self.m_f_global_height_exponent).inspect_value(path, offset, limit))
                .map(|v| ("m_fGlobalHeightExponent".to_string(), v)),
            ((self.m_af_global_distance_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_0".to_string(), v)),
            ((self.m_af_global_distance_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_1".to_string(), v)),
            ((self.m_af_global_distance_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_2".to_string(), v)),
            ((self.m_af_global_distance_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_3".to_string(), v)),
            ((self.m_af_global_distance_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_4".to_string(), v)),
            ((self.m_af_global_distance_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_5".to_string(), v)),
            ((self.m_af_global_distance_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_6".to_string(), v)),
            ((self.m_af_global_distance_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_7".to_string(), v)),
            ((self.m_af_global_distance_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_8".to_string(), v)),
            ((self.m_af_global_distance_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDistance_9".to_string(), v)),
            ((self.m_af_global_direction_adherence_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_0".to_string(), v)),
            ((self.m_af_global_direction_adherence_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_1".to_string(), v)),
            ((self.m_af_global_direction_adherence_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_2".to_string(), v)),
            ((self.m_af_global_direction_adherence_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_3".to_string(), v)),
            ((self.m_af_global_direction_adherence_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_4".to_string(), v)),
            ((self.m_af_global_direction_adherence_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_5".to_string(), v)),
            ((self.m_af_global_direction_adherence_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_6".to_string(), v)),
            ((self.m_af_global_direction_adherence_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_7".to_string(), v)),
            ((self.m_af_global_direction_adherence_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_8".to_string(), v)),
            ((self.m_af_global_direction_adherence_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afGlobalDirectionAdherence_9".to_string(), v)),
            ((self.branch_level1).inspect_value(path, offset, limit))
                .map(|v| ("BranchLevel1".to_string(), v)),
            ((self.branch_level2).inspect_value(path, offset, limit))
                .map(|v| ("BranchLevel2".to_string(), v)),
            ((self.leaf_group1).inspect_value(path, offset, limit))
                .map(|v| ("LeafGroup1".to_string(), v)),
            ((self.leaf_group2).inspect_value(path, offset, limit))
                .map(|v| ("LeafGroup2".to_string(), v)),
            ((self.m_af_frond_ripple_distance_0).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_0".to_string(), v)),
            ((self.m_af_frond_ripple_distance_1).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_1".to_string(), v)),
            ((self.m_af_frond_ripple_distance_2).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_2".to_string(), v)),
            ((self.m_af_frond_ripple_distance_3).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_3".to_string(), v)),
            ((self.m_af_frond_ripple_distance_4).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_4".to_string(), v)),
            ((self.m_af_frond_ripple_distance_5).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_5".to_string(), v)),
            ((self.m_af_frond_ripple_distance_6).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_6".to_string(), v)),
            ((self.m_af_frond_ripple_distance_7).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_7".to_string(), v)),
            ((self.m_af_frond_ripple_distance_8).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_8".to_string(), v)),
            ((self.m_af_frond_ripple_distance_9).inspect_value(path, offset, limit))
                .map(|v| ("m_afFrondRippleDistance_9".to_string(), v)),
            ((self.m_f_frond_ripple_tile).inspect_value(path, offset, limit))
                .map(|v| ("m_fFrondRippleTile".to_string(), v)),
            ((self.m_f_frond_ripple_lighting_scalar).inspect_value(path, offset, limit))
                .map(|v| ("m_fFrondRippleLightingScalar".to_string(), v)),
            ((self.m_f_rolling_noise_size).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingNoiseSize".to_string(), v)),
            ((self.m_f_rolling_noise_twist).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingNoiseTwist".to_string(), v)),
            ((self.m_f_rolling_noise_turbulence).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingNoiseTurbulence".to_string(), v)),
            ((self.m_f_rolling_noise_period).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingNoisePeriod".to_string(), v)),
            ((self.m_f_rolling_noise_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingNoiseSpeed".to_string(), v)),
            ((self.m_f_rolling_branch_field_min).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingBranchFieldMin".to_string(), v)),
            ((self.m_f_rolling_branch_lighting_adjust).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingBranchLightingAdjust".to_string(), v)),
            ((self.m_f_rolling_branch_vertical_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingBranchVerticalOffset".to_string(), v)),
            ((self.m_f_rolling_leaf_ripple_min).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingLeafRippleMin".to_string(), v)),
            ((self.m_f_rolling_leaf_tumble_min).inspect_value(path, offset, limit))
                .map(|v| ("m_fRollingLeafTumbleMin".to_string(), v)),
            ((self.branch_wind_anchor0).inspect_value(path, offset, limit))
                .map(|v| ("BranchWindAnchor0".to_string(), v)),
            ((self.branch_wind_anchor1).inspect_value(path, offset, limit))
                .map(|v| ("BranchWindAnchor1".to_string(), v)),
            ((self.branch_wind_anchor2).inspect_value(path, offset, limit))
                .map(|v| ("BranchWindAnchor2".to_string(), v)),
            ((self.m_f_max_branch_level1_length).inspect_value(path, offset, limit))
                .map(|v| ("m_fMaxBranchLevel1Length".to_string(), v)),
            ((self.global_wind).inspect_value(path, offset, limit))
                .map(|v| ("GLOBAL_WIND".to_string(), v)),
            ((self.global_preserve_shape).inspect_value(path, offset, limit))
                .map(|v| ("GLOBAL_PRESERVE_SHAPE".to_string(), v)),
            ((self.branch_simple_1).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_SIMPLE_1".to_string(), v)),
            ((self.branch_directional_1).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_DIRECTIONAL_1".to_string(), v)),
            ((self.branch_directional_frond_1).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_DIRECTIONAL_FROND_1".to_string(), v)),
            ((self.branch_turbulence_1).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_TURBULENCE_1".to_string(), v)),
            ((self.branch_whip_1).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_WHIP_1".to_string(), v)),
            ((self.branch_osc_complex_1).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_OSC_COMPLEX_1".to_string(), v)),
            ((self.branch_simple_2).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_SIMPLE_2".to_string(), v)),
            ((self.branch_directional_2).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_DIRECTIONAL_2".to_string(), v)),
            ((self.branch_directional_frond_2).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_DIRECTIONAL_FROND_2".to_string(), v)),
            ((self.branch_turbulence_2).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_TURBULENCE_2".to_string(), v)),
            ((self.branch_whip_2).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_WHIP_2".to_string(), v)),
            ((self.branch_osc_complex_2).inspect_value(path, offset, limit))
                .map(|v| ("BRANCH_OSC_COMPLEX_2".to_string(), v)),
            ((self.leaf_ripple_vertex_normal_1).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_RIPPLE_VERTEX_NORMAL_1".to_string(), v)),
            ((self.leaf_ripple_computed_1).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_RIPPLE_COMPUTED_1".to_string(), v)),
            ((self.leaf_tumble_1).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_TUMBLE_1".to_string(), v)),
            ((self.leaf_twitch_1).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_TWITCH_1".to_string(), v)),
            ((self.leaf_occlusion_1).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_OCCLUSION_1".to_string(), v)),
            ((self.leaf_ripple_vertex_normal_2).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_RIPPLE_VERTEX_NORMAL_2".to_string(), v)),
            ((self.leaf_ripple_computed_2).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_RIPPLE_COMPUTED_2".to_string(), v)),
            ((self.leaf_tumble_2).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_TUMBLE_2".to_string(), v)),
            ((self.leaf_twitch_2).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_TWITCH_2".to_string(), v)),
            ((self.leaf_occlusion_2).inspect_value(path, offset, limit))
                .map(|v| ("LEAF_OCCLUSION_2".to_string(), v)),
            ((self.frond_ripple_one_sided).inspect_value(path, offset, limit))
                .map(|v| ("FROND_RIPPLE_ONE_SIDED".to_string(), v)),
            ((self.frond_ripple_two_sided).inspect_value(path, offset, limit))
                .map(|v| ("FROND_RIPPLE_TWO_SIDED".to_string(), v)),
            ((self.frond_ripple_adjust_lighting).inspect_value(path, offset, limit))
                .map(|v| ("FROND_RIPPLE_ADJUST_LIGHTING".to_string(), v)),
            ((self.rolling).inspect_value(path, offset, limit)).map(|v| ("ROLLING".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
