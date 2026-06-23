//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::vector2f::Vector2f;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Camera {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgba,
    pub m_normalized_view_port_rect: Rectf,
    pub m_projection_matrix_mode: Option<i32>,
    pub near_clip_plane: f32,
    pub m_sensor_size: Option<Vector2f>,
    pub m_gate_fit_mode: Option<i32>,
    pub far_clip_plane: f32,
    pub m_lens_shift: Option<Vector2f>,
    pub m_iso: Option<i32>,
    pub field_of_view: f32,
    pub m_focal_length: Option<f32>,
    pub m_shutter_speed: Option<f32>,
    pub orthographic: bool,
    pub m_aperture: Option<f32>,
    pub orthographic_size: f32,
    pub m_focus_distance: Option<f32>,
    pub m_depth: f32,
    pub m_culling_mask: BitField,
    pub m_blade_count: Option<i32>,
    pub m_rendering_path: i32,
    pub m_curvature: Option<Vector2f>,
    pub m_target_texture: PPtr,
    pub m_barrel_clipping: Option<f32>,
    pub m_hdr: Option<bool>,
    pub m_target_display: Option<u32>,
    pub m_anamorphism: Option<f32>,
    pub m_occlusion_culling: Option<bool>,
    pub m_target_eye: Option<i32>,
    pub m_stereo_convergence: Option<f32>,
    pub m_allow_msaa: Option<bool>,
    pub m_stereo_separation: Option<f32>,
    pub m_force_into_rt: Option<bool>,
    pub m_allow_dynamic_resolution: Option<bool>,
    pub m_stereo_mirror_mode: Option<bool>,
}

impl crate::Parse for Camera {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_clear_flags = r.read_u32()?;
        let m_back_ground_color = ColorRgba::parse(r, ver)?;
        let m_normalized_view_port_rect = Rectf::parse(r, ver)?;
        let m_projection_matrix_mode = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let near_clip_plane = r.read_f32()?;
        let m_sensor_size = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_gate_fit_mode = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let far_clip_plane = r.read_f32()?;
        let m_lens_shift = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_iso = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let field_of_view = r.read_f32()?;
        let m_focal_length = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_shutter_speed = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let orthographic = r.read_bool()?;
        r.align(4)?;
        let m_aperture = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let orthographic_size = r.read_f32()?;
        let m_focus_distance = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_depth = r.read_f32()?;
        let m_culling_mask = BitField::parse(r, ver)?;
        let m_blade_count = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_rendering_path = r.read_i32()?;
        let m_curvature = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_target_texture = crate::PPtr::parse(r, ver)?;
        let m_barrel_clipping = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_hdr = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_target_display = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_anamorphism = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_occlusion_culling = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_target_eye = if *ver >= crate::UnityVersion::new(5, 1, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_stereo_convergence = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_allow_msaa = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_stereo_separation = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_force_into_rt = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_allow_dynamic_resolution = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_stereo_mirror_mode = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(Camera {
            parent,
            m_game_object,
            m_enabled,
            m_clear_flags,
            m_back_ground_color,
            m_normalized_view_port_rect,
            m_projection_matrix_mode,
            near_clip_plane,
            m_sensor_size,
            m_gate_fit_mode,
            far_clip_plane,
            m_lens_shift,
            m_iso,
            field_of_view,
            m_focal_length,
            m_shutter_speed,
            orthographic,
            m_aperture,
            orthographic_size,
            m_focus_distance,
            m_depth,
            m_culling_mask,
            m_blade_count,
            m_rendering_path,
            m_curvature,
            m_target_texture,
            m_barrel_clipping,
            m_hdr,
            m_target_display,
            m_anamorphism,
            m_occlusion_culling,
            m_target_eye,
            m_stereo_convergence,
            m_allow_msaa,
            m_stereo_separation,
            m_force_into_rt,
            m_allow_dynamic_resolution,
            m_stereo_mirror_mode,
        })
    }
}
