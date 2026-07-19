//! 自动生成。请勿手动修改。

use crate::subclasses::matrix4x4f::Matrix4x4f;

#[derive(Debug, Clone)]
pub struct ShadowSettings {
    pub m_type: i32,
    pub m_resolution: i32,
    pub m_strength: f32,
    pub m_custom_resolution: Option<i32>,
    pub m_bias: f32,
    pub m_softness: Option<f32>,
    pub m_normal_bias: Option<f32>,
    pub m_softness_fade: Option<f32>,
    pub m_near_plane: Option<f32>,
    pub m_culling_matrix_override: Option<Matrix4x4f>,
    pub m_use_culling_matrix_override: Option<bool>,
}

impl crate::Parse for ShadowSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_type = None;
        let mut m_resolution = None;
        let mut m_strength = None;
        let mut m_custom_resolution = None;
        let mut m_bias = None;
        let mut m_softness = None;
        let mut m_normal_bias = None;
        let mut m_softness_fade = None;
        let mut m_near_plane = None;
        let mut m_culling_matrix_override = None;
        let mut m_use_culling_matrix_override = None;
        m_type = Some(rdr.read_i32()?);
        m_resolution = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_custom_resolution = Some(rdr.read_i32()?);
        } else {
        }
        m_strength = Some(rdr.read_f32()?);
        m_bias = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_normal_bias = Some(rdr.read_f32()?);
            m_near_plane = Some(rdr.read_f32()?);
            m_culling_matrix_override = Some(Matrix4x4f::parse(rdr, ver)?);
            m_use_culling_matrix_override = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_normal_bias = Some(rdr.read_f32()?);
            m_near_plane = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_normal_bias = Some(rdr.read_f32()?);
        } else {
            m_softness = Some(rdr.read_f32()?);
            m_softness_fade = Some(rdr.read_f32()?);
        }

        Ok(ShadowSettings {
            m_type: m_type.unwrap(),
            m_resolution: m_resolution.unwrap(),
            m_strength: m_strength.unwrap(),
            m_custom_resolution,
            m_bias: m_bias.unwrap(),
            m_softness,
            m_normal_bias,
            m_softness_fade,
            m_near_plane,
            m_culling_matrix_override,
            m_use_culling_matrix_override,
        })
    }
}
