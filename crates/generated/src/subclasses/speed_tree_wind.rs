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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_s_params = SParams::parse(r, ver)?;
        let branch_wind_anchor0 = r.read_f32()?;
        let branch_wind_anchor1 = r.read_f32()?;
        let branch_wind_anchor2 = r.read_f32()?;
        let m_f_max_branch_level1_length = r.read_f32()?;
        let global_wind = r.read_bool()?;
        let global_preserve_shape = r.read_bool()?;
        let branch_simple_1 = r.read_bool()?;
        let branch_directional_1 = r.read_bool()?;
        let branch_directional_frond_1 = r.read_bool()?;
        let branch_turbulence_1 = r.read_bool()?;
        let branch_whip_1 = r.read_bool()?;
        let branch_osc_complex_1 = r.read_bool()?;
        let branch_simple_2 = r.read_bool()?;
        let branch_directional_2 = r.read_bool()?;
        let branch_directional_frond_2 = r.read_bool()?;
        let branch_turbulence_2 = r.read_bool()?;
        let branch_whip_2 = r.read_bool()?;
        let branch_osc_complex_2 = r.read_bool()?;
        let leaf_ripple_vertex_normal_1 = r.read_bool()?;
        let leaf_ripple_computed_1 = r.read_bool()?;
        let leaf_tumble_1 = r.read_bool()?;
        let leaf_twitch_1 = r.read_bool()?;
        let leaf_occlusion_1 = r.read_bool()?;
        let leaf_ripple_vertex_normal_2 = r.read_bool()?;
        let leaf_ripple_computed_2 = r.read_bool()?;
        let leaf_tumble_2 = r.read_bool()?;
        let leaf_twitch_2 = r.read_bool()?;
        let leaf_occlusion_2 = r.read_bool()?;
        let frond_ripple_one_sided = r.read_bool()?;
        let frond_ripple_two_sided = r.read_bool()?;
        let frond_ripple_adjust_lighting = r.read_bool()?;
        let rolling = r.read_bool()?;

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
