//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;
use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::gradient::Gradient;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct LineParameters {
    pub start_width: Option<f32>,
    pub width_multiplier: Option<f32>,
    pub end_width: Option<f32>,
    pub width_curve: Option<AnimationCurve>,
    pub m_start_color: Option<ColorRgba32>,
    pub color_gradient: Option<Gradient>,
    pub m_end_color: Option<ColorRgba32>,
    pub num_corner_vertices: Option<i32>,
    pub num_cap_vertices: Option<i32>,
    pub alignment: Option<i32>,
    pub texture_mode: Option<i32>,
    pub generate_lighting_data: Option<bool>,
    pub shadow_bias: Option<f32>,
    pub texture_scale: Option<Vector2f>,
}

impl crate::Parse for LineParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut start_width = None;
        let mut width_multiplier = None;
        let mut end_width = None;
        let mut width_curve = None;
        let mut m_start_color = None;
        let mut color_gradient = None;
        let mut m_end_color = None;
        let mut num_corner_vertices = None;
        let mut num_cap_vertices = None;
        let mut alignment = None;
        let mut texture_mode = None;
        let mut generate_lighting_data = None;
        let mut shadow_bias = None;
        let mut texture_scale = None;
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            width_multiplier = Some(rdr.read_f32()?);
            width_curve = Some(AnimationCurve::parse(rdr, ver)?);
            color_gradient = Some(Gradient::parse(rdr, ver)?);
            num_corner_vertices = Some(rdr.read_i32()?);
            num_cap_vertices = Some(rdr.read_i32()?);
            alignment = Some(rdr.read_i32()?);
            texture_mode = Some(rdr.read_i32()?);
            texture_scale = Some(Vector2f::parse(rdr, ver)?);
            shadow_bias = Some(rdr.read_f32()?);
            generate_lighting_data = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            width_multiplier = Some(rdr.read_f32()?);
            width_curve = Some(AnimationCurve::parse(rdr, ver)?);
            color_gradient = Some(Gradient::parse(rdr, ver)?);
            num_corner_vertices = Some(rdr.read_i32()?);
            num_cap_vertices = Some(rdr.read_i32()?);
            alignment = Some(rdr.read_i32()?);
            texture_mode = Some(rdr.read_i32()?);
            shadow_bias = Some(rdr.read_f32()?);
            generate_lighting_data = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            width_multiplier = Some(rdr.read_f32()?);
            width_curve = Some(AnimationCurve::parse(rdr, ver)?);
            color_gradient = Some(Gradient::parse(rdr, ver)?);
            num_corner_vertices = Some(rdr.read_i32()?);
            num_cap_vertices = Some(rdr.read_i32()?);
            alignment = Some(rdr.read_i32()?);
            texture_mode = Some(rdr.read_i32()?);
            generate_lighting_data = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            width_multiplier = Some(rdr.read_f32()?);
            width_curve = Some(AnimationCurve::parse(rdr, ver)?);
            color_gradient = Some(Gradient::parse(rdr, ver)?);
            num_corner_vertices = Some(rdr.read_i32()?);
            num_cap_vertices = Some(rdr.read_i32()?);
            alignment = Some(rdr.read_i32()?);
            texture_mode = Some(rdr.read_i32()?);
        } else {
            start_width = Some(rdr.read_f32()?);
            end_width = Some(rdr.read_f32()?);
            m_start_color = Some(ColorRgba32::parse(rdr, ver)?);
            m_end_color = Some(ColorRgba32::parse(rdr, ver)?);
        }

        Ok(LineParameters {
            start_width,
            width_multiplier,
            end_width,
            width_curve,
            m_start_color,
            color_gradient,
            m_end_color,
            num_corner_vertices,
            num_cap_vertices,
            alignment,
            texture_mode,
            generate_lighting_data,
            shadow_bias,
            texture_scale,
        })
    }
}
