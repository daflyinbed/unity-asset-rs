//! 自动生成。请勿手动修改。

use crate::subclasses::s_params::SParams;

#[derive(Debug, Clone)]
pub struct SpeedTreeWind {
    pub m_s_params: SParams,
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

impl crate::Parse for SpeedTreeWind {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_s_params = SParams::parse(rdr, ver)?;
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

        Ok(SpeedTreeWind {
            m_s_params,
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

impl crate::ToValue for SpeedTreeWind {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_sParams".to_string(), self.m_s_params.to_value())),
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
                "m_sParams" => (self.m_s_params).inspect_value(path, offset, limit),
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
            ((self.m_s_params).inspect_value(path, offset, limit))
                .map(|v| ("m_sParams".to_string(), v)),
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
