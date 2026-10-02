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
        let mut m_f_strength_response = None;
        let mut m_f_direction_response = None;
        let mut m_f_gust_frequency = None;
        let mut m_f_gust_strength_min = None;
        let mut m_f_gust_strength_max = None;
        let mut m_f_gust_duration_min = None;
        let mut m_f_gust_duration_max = None;
        let mut m_f_gust_rise_scalar = None;
        let mut m_f_gust_fall_scalar = None;
        let mut m_f_branch1_stretch_limit = None;
        let mut m_f_branch2_stretch_limit = None;
        let mut m_f_shared_height_start = None;
        let mut m_s_shared = None;
        let mut m_s_branch1 = None;
        let mut m_s_branch2 = None;
        let mut m_s_ripple = None;
        let mut m_v_tree_extents = None;
        let mut m_f_wind_independence = None;
        let mut m_b_do_shared = None;
        let mut m_b_do_branch1 = None;
        let mut m_b_do_branch2 = None;
        let mut m_b_do_ripple = None;
        let mut m_b_do_shimmer = None;
        let mut m_b_lod_fade = None;
        let mut pad = None;
        let mut m_f_import_scaling = None;
        m_f_strength_response = Some(rdr.read_f32()?);
        m_f_direction_response = Some(rdr.read_f32()?);
        m_f_gust_frequency = Some(rdr.read_f32()?);
        m_f_gust_strength_min = Some(rdr.read_f32()?);
        m_f_gust_strength_max = Some(rdr.read_f32()?);
        m_f_gust_duration_min = Some(rdr.read_f32()?);
        m_f_gust_duration_max = Some(rdr.read_f32()?);
        m_f_gust_rise_scalar = Some(rdr.read_f32()?);
        m_f_gust_fall_scalar = Some(rdr.read_f32()?);
        m_f_branch1_stretch_limit = Some(rdr.read_f32()?);
        m_f_branch2_stretch_limit = Some(rdr.read_f32()?);
        m_f_shared_height_start = Some(rdr.read_f32()?);
        m_s_shared = Some(BranchWindLevel::parse(rdr, ver)?);
        m_s_branch1 = Some(BranchWindLevel::parse(rdr, ver)?);
        m_s_branch2 = Some(BranchWindLevel::parse(rdr, ver)?);
        m_s_ripple = Some(RippleGroup::parse(rdr, ver)?);
        m_v_tree_extents = Some(Vector3f::parse(rdr, ver)?);
        m_f_wind_independence = Some(rdr.read_f32()?);
        m_b_do_shared = Some(rdr.read_i32()?);
        m_b_do_branch1 = Some(rdr.read_i32()?);
        m_b_do_branch2 = Some(rdr.read_i32()?);
        m_b_do_ripple = Some(rdr.read_i32()?);
        m_b_do_shimmer = Some(rdr.read_i32()?);
        m_b_lod_fade = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 8) {
            m_f_import_scaling = Some(rdr.read_f32()?);
        } else {
            pad = Some(rdr.read_i32()?);
        }

        Ok(SpeedTreeWindConfig9 {
            m_f_strength_response: m_f_strength_response.unwrap(),
            m_f_direction_response: m_f_direction_response.unwrap(),
            m_f_gust_frequency: m_f_gust_frequency.unwrap(),
            m_f_gust_strength_min: m_f_gust_strength_min.unwrap(),
            m_f_gust_strength_max: m_f_gust_strength_max.unwrap(),
            m_f_gust_duration_min: m_f_gust_duration_min.unwrap(),
            m_f_gust_duration_max: m_f_gust_duration_max.unwrap(),
            m_f_gust_rise_scalar: m_f_gust_rise_scalar.unwrap(),
            m_f_gust_fall_scalar: m_f_gust_fall_scalar.unwrap(),
            m_f_branch1_stretch_limit: m_f_branch1_stretch_limit.unwrap(),
            m_f_branch2_stretch_limit: m_f_branch2_stretch_limit.unwrap(),
            m_f_shared_height_start: m_f_shared_height_start.unwrap(),
            m_s_shared: m_s_shared.unwrap(),
            m_s_branch1: m_s_branch1.unwrap(),
            m_s_branch2: m_s_branch2.unwrap(),
            m_s_ripple: m_s_ripple.unwrap(),
            m_v_tree_extents: m_v_tree_extents.unwrap(),
            m_f_wind_independence: m_f_wind_independence.unwrap(),
            m_b_do_shared: m_b_do_shared.unwrap(),
            m_b_do_branch1: m_b_do_branch1.unwrap(),
            m_b_do_branch2: m_b_do_branch2.unwrap(),
            m_b_do_ripple: m_b_do_ripple.unwrap(),
            m_b_do_shimmer: m_b_do_shimmer.unwrap(),
            m_b_lod_fade: m_b_lod_fade.unwrap(),
            pad,
            m_f_import_scaling,
        })
    }
}

impl crate::ToValue for SpeedTreeWindConfig9 {
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
                "m_fBranch1StretchLimit".to_string(),
                self.m_f_branch1_stretch_limit.to_value(),
            )),
            Some((
                "m_fBranch2StretchLimit".to_string(),
                self.m_f_branch2_stretch_limit.to_value(),
            )),
            Some((
                "m_fSharedHeightStart".to_string(),
                self.m_f_shared_height_start.to_value(),
            )),
            Some(("m_sShared".to_string(), self.m_s_shared.to_value())),
            Some(("m_sBranch1".to_string(), self.m_s_branch1.to_value())),
            Some(("m_sBranch2".to_string(), self.m_s_branch2.to_value())),
            Some(("m_sRipple".to_string(), self.m_s_ripple.to_value())),
            Some((
                "m_vTreeExtents".to_string(),
                self.m_v_tree_extents.to_value(),
            )),
            Some((
                "m_fWindIndependence".to_string(),
                self.m_f_wind_independence.to_value(),
            )),
            Some(("m_bDoShared".to_string(), self.m_b_do_shared.to_value())),
            Some(("m_bDoBranch1".to_string(), self.m_b_do_branch1.to_value())),
            Some(("m_bDoBranch2".to_string(), self.m_b_do_branch2.to_value())),
            Some(("m_bDoRipple".to_string(), self.m_b_do_ripple.to_value())),
            Some(("m_bDoShimmer".to_string(), self.m_b_do_shimmer.to_value())),
            Some(("m_bLodFade".to_string(), self.m_b_lod_fade.to_value())),
            crate::opt_field("pad", &self.pad),
            crate::opt_field("m_fImportScaling", &self.m_f_import_scaling),
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
                "m_fBranch1StretchLimit" => {
                    (self.m_f_branch1_stretch_limit).inspect_value(path, offset, limit)
                }
                "m_fBranch2StretchLimit" => {
                    (self.m_f_branch2_stretch_limit).inspect_value(path, offset, limit)
                }
                "m_fSharedHeightStart" => {
                    (self.m_f_shared_height_start).inspect_value(path, offset, limit)
                }
                "m_sShared" => (self.m_s_shared).inspect_value(path, offset, limit),
                "m_sBranch1" => (self.m_s_branch1).inspect_value(path, offset, limit),
                "m_sBranch2" => (self.m_s_branch2).inspect_value(path, offset, limit),
                "m_sRipple" => (self.m_s_ripple).inspect_value(path, offset, limit),
                "m_vTreeExtents" => (self.m_v_tree_extents).inspect_value(path, offset, limit),
                "m_fWindIndependence" => {
                    (self.m_f_wind_independence).inspect_value(path, offset, limit)
                }
                "m_bDoShared" => (self.m_b_do_shared).inspect_value(path, offset, limit),
                "m_bDoBranch1" => (self.m_b_do_branch1).inspect_value(path, offset, limit),
                "m_bDoBranch2" => (self.m_b_do_branch2).inspect_value(path, offset, limit),
                "m_bDoRipple" => (self.m_b_do_ripple).inspect_value(path, offset, limit),
                "m_bDoShimmer" => (self.m_b_do_shimmer).inspect_value(path, offset, limit),
                "m_bLodFade" => (self.m_b_lod_fade).inspect_value(path, offset, limit),
                "pad" => (self.pad).inspect_value(path, offset, limit),
                "m_fImportScaling" => (self.m_f_import_scaling).inspect_value(path, offset, limit),
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
            ((self.m_f_branch1_stretch_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_fBranch1StretchLimit".to_string(), v)),
            ((self.m_f_branch2_stretch_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_fBranch2StretchLimit".to_string(), v)),
            ((self.m_f_shared_height_start).inspect_value(path, offset, limit))
                .map(|v| ("m_fSharedHeightStart".to_string(), v)),
            ((self.m_s_shared).inspect_value(path, offset, limit))
                .map(|v| ("m_sShared".to_string(), v)),
            ((self.m_s_branch1).inspect_value(path, offset, limit))
                .map(|v| ("m_sBranch1".to_string(), v)),
            ((self.m_s_branch2).inspect_value(path, offset, limit))
                .map(|v| ("m_sBranch2".to_string(), v)),
            ((self.m_s_ripple).inspect_value(path, offset, limit))
                .map(|v| ("m_sRipple".to_string(), v)),
            ((self.m_v_tree_extents).inspect_value(path, offset, limit))
                .map(|v| ("m_vTreeExtents".to_string(), v)),
            ((self.m_f_wind_independence).inspect_value(path, offset, limit))
                .map(|v| ("m_fWindIndependence".to_string(), v)),
            ((self.m_b_do_shared).inspect_value(path, offset, limit))
                .map(|v| ("m_bDoShared".to_string(), v)),
            ((self.m_b_do_branch1).inspect_value(path, offset, limit))
                .map(|v| ("m_bDoBranch1".to_string(), v)),
            ((self.m_b_do_branch2).inspect_value(path, offset, limit))
                .map(|v| ("m_bDoBranch2".to_string(), v)),
            ((self.m_b_do_ripple).inspect_value(path, offset, limit))
                .map(|v| ("m_bDoRipple".to_string(), v)),
            ((self.m_b_do_shimmer).inspect_value(path, offset, limit))
                .map(|v| ("m_bDoShimmer".to_string(), v)),
            ((self.m_b_lod_fade).inspect_value(path, offset, limit))
                .map(|v| ("m_bLodFade".to_string(), v)),
            ((self.pad).inspect_value(path, offset, limit)).map(|v| ("pad".to_string(), v)),
            ((self.m_f_import_scaling).inspect_value(path, offset, limit))
                .map(|v| ("m_fImportScaling".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
