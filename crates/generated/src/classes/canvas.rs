//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_alpha: Option<f32>,
    pub m_render_mode: i32,
    pub m_camera: PPtr,
    pub m_plane_distance: Option<f32>,
    pub m_normals: Option<bool>,
    pub m_pixel_perfect: bool,
    pub m_position_u_vs: Option<bool>,
    pub m_receives_events: Option<bool>,
    pub m_override_sorting: Option<bool>,
    pub m_override_pixel_perfect: Option<bool>,
    pub m_sorting_layer_id: Option<u32>,
    pub m_sorting_bucket_normalized_size: Option<f32>,
    pub m_sorting_order: Option<i16>,
    pub m_additional_shader_channels_flag: Option<i32>,
    pub m_vertex_color_always_gamma_space: Option<bool>,
    pub m_target_display: Option<i8>,
    pub m_update_rect_transform_for_standalone: Option<i32>,
    pub m_use_reflection_probes: Option<bool>,
}

impl crate::Parse for Canvas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_alpha = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_render_mode = r.read_i32()?;
        let m_camera = crate::PPtr::parse(r, ver)?;
        let m_plane_distance = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_normals = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_pixel_perfect = r.read_bool()?;
        let m_position_u_vs = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_receives_events = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_override_sorting = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_override_pixel_perfect = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sorting_layer_id = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_sorting_bucket_normalized_size = if *ver >= crate::UnityVersion::new(5, 3, 4) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sorting_order = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(r.read_i16()?)
        } else {
            None
        };
        let m_additional_shader_channels_flag = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_vertex_color_always_gamma_space = if *ver >= crate::UnityVersion::new(2021, 3, 34) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_target_display = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_i8()?)
        } else {
            None
        };
        let m_update_rect_transform_for_standalone = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_reflection_probes = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Canvas {
            parent,
            m_game_object,
            m_enabled,
            m_alpha,
            m_render_mode,
            m_camera,
            m_plane_distance,
            m_normals,
            m_pixel_perfect,
            m_position_u_vs,
            m_receives_events,
            m_override_sorting,
            m_override_pixel_perfect,
            m_sorting_layer_id,
            m_sorting_bucket_normalized_size,
            m_sorting_order,
            m_additional_shader_channels_flag,
            m_vertex_color_always_gamma_space,
            m_target_display,
            m_update_rect_transform_for_standalone,
            m_use_reflection_probes,
        })
    }
}
