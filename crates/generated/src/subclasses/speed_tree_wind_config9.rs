//! 自动生成。请勿手动修改。

use crate::subclasses::branch_wind_level::BranchWindLevel;
use crate::subclasses::ripple_group::RippleGroup;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpeedTreeWindConfig9 {
    pub m_f_strength_response: f32,
    pub m_f_direction_response: f32,
    pub m_f_gust_frequency: f32,
    pub m_f_gust_strength_min: f32,
    pub m_f_gust_strength_max: f32,
    pub m_f_gust_duration_min: f32,
    pub m_f_gust_duration_max: f32,
    pub m_f_gust_rise_scalar: f32,
    pub m_f_gust_fall_scalar: f32,
    pub m_f_branch1_stretch_limit: f32,
    pub m_f_branch2_stretch_limit: f32,
    pub m_f_shared_height_start: f32,
    pub m_s_shared: BranchWindLevel,
    pub m_s_branch1: BranchWindLevel,
    pub m_s_branch2: BranchWindLevel,
    pub m_s_ripple: RippleGroup,
    pub m_v_tree_extents: Vector3f,
    pub m_f_wind_independence: f32,
    pub m_b_do_shared: i32,
    pub m_b_do_branch1: i32,
    pub m_b_do_branch2: i32,
    pub m_b_do_ripple: i32,
    pub m_b_do_shimmer: i32,
    pub m_b_lod_fade: i32,
    pub pad: i32,
}

impl crate::Parse for SpeedTreeWindConfig9 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_f_strength_response = r.read_f32()?;
        let m_f_direction_response = r.read_f32()?;
        let m_f_gust_frequency = r.read_f32()?;
        let m_f_gust_strength_min = r.read_f32()?;
        let m_f_gust_strength_max = r.read_f32()?;
        let m_f_gust_duration_min = r.read_f32()?;
        let m_f_gust_duration_max = r.read_f32()?;
        let m_f_gust_rise_scalar = r.read_f32()?;
        let m_f_gust_fall_scalar = r.read_f32()?;
        let m_f_branch1_stretch_limit = r.read_f32()?;
        let m_f_branch2_stretch_limit = r.read_f32()?;
        let m_f_shared_height_start = r.read_f32()?;
        let m_s_shared = BranchWindLevel::parse(r, ver)?;
        let m_s_branch1 = BranchWindLevel::parse(r, ver)?;
        let m_s_branch2 = BranchWindLevel::parse(r, ver)?;
        let m_s_ripple = RippleGroup::parse(r, ver)?;
        let m_v_tree_extents = Vector3f::parse(r, ver)?;
        let m_f_wind_independence = r.read_f32()?;
        let m_b_do_shared = r.read_i32()?;
        let m_b_do_branch1 = r.read_i32()?;
        let m_b_do_branch2 = r.read_i32()?;
        let m_b_do_ripple = r.read_i32()?;
        let m_b_do_shimmer = r.read_i32()?;
        let m_b_lod_fade = r.read_i32()?;
        let pad = r.read_i32()?;

        Ok(SpeedTreeWindConfig9 {
            m_f_strength_response,
            m_f_direction_response,
            m_f_gust_frequency,
            m_f_gust_strength_min,
            m_f_gust_strength_max,
            m_f_gust_duration_min,
            m_f_gust_duration_max,
            m_f_gust_rise_scalar,
            m_f_gust_fall_scalar,
            m_f_branch1_stretch_limit,
            m_f_branch2_stretch_limit,
            m_f_shared_height_start,
            m_s_shared,
            m_s_branch1,
            m_s_branch2,
            m_s_ripple,
            m_v_tree_extents,
            m_f_wind_independence,
            m_b_do_shared,
            m_b_do_branch1,
            m_b_do_branch2,
            m_b_do_ripple,
            m_b_do_shimmer,
            m_b_lod_fade,
            pad,
        })
    }
}
