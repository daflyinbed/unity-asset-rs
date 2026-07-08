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
    pub pad: Option<i32>,
    pub m_f_import_scaling: Option<f32>,
}

impl crate::Parse for SpeedTreeWindConfig9 {
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
        let m_f_branch1_stretch_limit = rdr.read_f32()?;
        let m_f_branch2_stretch_limit = rdr.read_f32()?;
        let m_f_shared_height_start = rdr.read_f32()?;
        let m_s_shared = BranchWindLevel::parse(rdr, ver)?;
        let m_s_branch1 = BranchWindLevel::parse(rdr, ver)?;
        let m_s_branch2 = BranchWindLevel::parse(rdr, ver)?;
        let m_s_ripple = RippleGroup::parse(rdr, ver)?;
        let m_v_tree_extents = Vector3f::parse(rdr, ver)?;
        let m_f_wind_independence = rdr.read_f32()?;
        let m_b_do_shared = rdr.read_i32()?;
        let m_b_do_branch1 = rdr.read_i32()?;
        let m_b_do_branch2 = rdr.read_i32()?;
        let m_b_do_ripple = rdr.read_i32()?;
        let m_b_do_shimmer = rdr.read_i32()?;
        let m_b_lod_fade = rdr.read_i32()?;
        let pad = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_f_import_scaling = if *ver >= crate::UnityVersion::new(6000, 0, 8) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

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
            m_f_import_scaling,
        })
    }
}
