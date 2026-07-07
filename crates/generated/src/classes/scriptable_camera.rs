//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ScriptableCamera {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgba,
    pub m_projection_matrix_mode: i32,
    pub m_gate_fit_mode: i32,
    pub m_sensor_size: Vector2f,
    pub m_lens_shift: Vector2f,
    pub m_focal_length: f32,
    pub m_normalized_view_port_rect: Rectf,
    pub near_clip_plane: f32,
    pub far_clip_plane: f32,
    pub field_of_view: f32,
    pub orthographic: bool,
    pub orthographic_size: f32,
    pub m_depth: f32,
    pub m_culling_mask: BitField,
    pub m_rendering_path: i32,
    pub m_target_texture: PPtr,
    pub m_target_display: i32,
    pub m_target_eye: i32,
    pub m_hdr: bool,
    pub m_allow_msaa: bool,
    pub m_allow_dynamic_resolution: bool,
    pub m_force_into_rt: bool,
    pub m_occlusion_culling: bool,
    pub m_stereo_convergence: f32,
    pub m_stereo_separation: f32,
    pub m_script: PPtr,
}

impl crate::Parse for ScriptableCamera {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_clear_flags = r.read_u32()?;
        let m_back_ground_color = ColorRgba::parse(r, ver)?;
        let m_projection_matrix_mode = r.read_i32()?;
        let m_gate_fit_mode = r.read_i32()?;
        r.align(4)?;
        let m_sensor_size = Vector2f::parse(r, ver)?;
        let m_lens_shift = Vector2f::parse(r, ver)?;
        let m_focal_length = r.read_f32()?;
        let m_normalized_view_port_rect = Rectf::parse(r, ver)?;
        let near_clip_plane = r.read_f32()?;
        let far_clip_plane = r.read_f32()?;
        let field_of_view = r.read_f32()?;
        let orthographic = r.read_bool()?;
        r.align(4)?;
        let orthographic_size = r.read_f32()?;
        let m_depth = r.read_f32()?;
        let m_culling_mask = BitField::parse(r, ver)?;
        let m_rendering_path = r.read_i32()?;
        let m_target_texture = crate::PPtr::parse(r, ver)?;
        let m_target_display = r.read_i32()?;
        let m_target_eye = r.read_i32()?;
        let m_hdr = r.read_bool()?;
        let m_allow_msaa = r.read_bool()?;
        let m_allow_dynamic_resolution = r.read_bool()?;
        let m_force_into_rt = r.read_bool()?;
        let m_occlusion_culling = r.read_bool()?;
        r.align(4)?;
        let m_stereo_convergence = r.read_f32()?;
        let m_stereo_separation = r.read_f32()?;
        let m_script = crate::PPtr::parse(r, ver)?;

        Ok(ScriptableCamera {
            m_game_object,
            m_enabled,
            m_clear_flags,
            m_back_ground_color,
            m_projection_matrix_mode,
            m_gate_fit_mode,
            m_sensor_size,
            m_lens_shift,
            m_focal_length,
            m_normalized_view_port_rect,
            near_clip_plane,
            far_clip_plane,
            field_of_view,
            orthographic,
            orthographic_size,
            m_depth,
            m_culling_mask,
            m_rendering_path,
            m_target_texture,
            m_target_display,
            m_target_eye,
            m_hdr,
            m_allow_msaa,
            m_allow_dynamic_resolution,
            m_force_into_rt,
            m_occlusion_culling,
            m_stereo_convergence,
            m_stereo_separation,
            m_script,
        })
    }
}
