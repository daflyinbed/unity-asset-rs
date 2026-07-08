//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：CameraMProjectionMatrixModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CameraMProjectionMatrixModeVariant {
    Default(i32),
    V2019_1(i32),
}

impl crate::Parse for CameraMProjectionMatrixModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CameraMGateFitModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CameraMGateFitModeVariant {
    Default(i32),
    V2019_1(i32),
}

impl crate::Parse for CameraMGateFitModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CameraMTargetDisplayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CameraMTargetDisplayVariant {
    Default(u32),
    V5_3(i32),
}

impl crate::Parse for CameraMTargetDisplayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CameraMOcclusionCullingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CameraMOcclusionCullingVariant {
    Default(bool),
    V4_5(bool),
}

impl crate::Parse for CameraMOcclusionCullingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Camera {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgbAf,
    pub m_normalized_view_port_rect: Rectf,
    pub m_projection_matrix_mode: Option<CameraMProjectionMatrixModeVariant>,
    pub near_clip_plane: f32,
    pub m_sensor_size: Option<Vector2f>,
    pub m_gate_fit_mode: Option<CameraMGateFitModeVariant>,
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
    pub m_target_display: Option<CameraMTargetDisplayVariant>,
    pub m_anamorphism: Option<f32>,
    pub m_occlusion_culling: Option<CameraMOcclusionCullingVariant>,
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_clear_flags = None;
        let mut m_back_ground_color = None;
        let mut m_normalized_view_port_rect = None;
        let mut m_projection_matrix_mode = None;
        let mut near_clip_plane = None;
        let mut m_sensor_size = None;
        let mut m_gate_fit_mode = None;
        let mut far_clip_plane = None;
        let mut m_lens_shift = None;
        let mut m_iso = None;
        let mut field_of_view = None;
        let mut m_focal_length = None;
        let mut m_shutter_speed = None;
        let mut orthographic = None;
        let mut m_aperture = None;
        let mut orthographic_size = None;
        let mut m_focus_distance = None;
        let mut m_depth = None;
        let mut m_culling_mask = None;
        let mut m_blade_count = None;
        let mut m_rendering_path = None;
        let mut m_curvature = None;
        let mut m_target_texture = None;
        let mut m_barrel_clipping = None;
        let mut m_hdr = None;
        let mut m_target_display = None;
        let mut m_anamorphism = None;
        let mut m_occlusion_culling = None;
        let mut m_target_eye = None;
        let mut m_stereo_convergence = None;
        let mut m_allow_msaa = None;
        let mut m_stereo_separation = None;
        let mut m_force_into_rt = None;
        let mut m_allow_dynamic_resolution = None;
        let mut m_stereo_mirror_mode = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_clear_flags = Some(rdr.read_u32()?);
            m_back_ground_color = Some(ColorRgbAf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_projection_matrix_mode = Some(CameraMProjectionMatrixModeVariant::parse(rdr, ver)?);
            m_gate_fit_mode = Some(CameraMGateFitModeVariant::parse(rdr, ver)?);
            m_iso = Some(rdr.read_i32()?);
            m_shutter_speed = Some(rdr.read_f32()?);
            m_aperture = Some(rdr.read_f32()?);
            m_focus_distance = Some(rdr.read_f32()?);
            m_focal_length = Some(rdr.read_f32()?);
            m_blade_count = Some(rdr.read_i32()?);
            m_curvature = Some(Vector2f::parse(rdr, ver)?);
            m_barrel_clipping = Some(rdr.read_f32()?);
            m_anamorphism = Some(rdr.read_f32()?);
            m_sensor_size = Some(Vector2f::parse(rdr, ver)?);
            m_lens_shift = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_projection_matrix_mode = Some(CameraMProjectionMatrixModeVariant::parse(rdr, ver)?);
            m_gate_fit_mode = Some(CameraMGateFitModeVariant::parse(rdr, ver)?);
            m_sensor_size = Some(Vector2f::parse(rdr, ver)?);
            m_lens_shift = Some(Vector2f::parse(rdr, ver)?);
            m_focal_length = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_projection_matrix_mode = Some(CameraMProjectionMatrixModeVariant::parse(rdr, ver)?);
            m_sensor_size = Some(Vector2f::parse(rdr, ver)?);
            m_lens_shift = Some(Vector2f::parse(rdr, ver)?);
            m_gate_fit_mode = Some(CameraMGateFitModeVariant::parse(rdr, ver)?);
            m_focal_length = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_projection_matrix_mode = Some(CameraMProjectionMatrixModeVariant::parse(rdr, ver)?);
            m_sensor_size = Some(Vector2f::parse(rdr, ver)?);
            m_lens_shift = Some(Vector2f::parse(rdr, ver)?);
            m_focal_length = Some(rdr.read_f32()?);
        } else {
        }
            m_normalized_view_port_rect = Some(Rectf::parse(rdr, ver)?);
            near_clip_plane = Some(rdr.read_f32()?);
            far_clip_plane = Some(rdr.read_f32()?);
            field_of_view = Some(rdr.read_f32()?);
            orthographic = Some(rdr.read_bool()?);
            rdr.align(4)?;
            orthographic_size = Some(rdr.read_f32()?);
            m_depth = Some(rdr.read_f32()?);
            m_culling_mask = Some(BitField::parse(rdr, ver)?);
            m_rendering_path = Some(rdr.read_i32()?);
            m_target_texture = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_target_display = Some(CameraMTargetDisplayVariant::parse(rdr, ver)?);
            m_target_eye = Some(rdr.read_i32()?);
            m_hdr = Some(rdr.read_bool()?);
            m_allow_msaa = Some(rdr.read_bool()?);
            m_allow_dynamic_resolution = Some(rdr.read_bool()?);
            m_force_into_rt = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
            m_stereo_convergence = Some(rdr.read_f32()?);
            m_stereo_separation = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_target_display = Some(CameraMTargetDisplayVariant::parse(rdr, ver)?);
            m_target_eye = Some(rdr.read_i32()?);
            m_hdr = Some(rdr.read_bool()?);
            m_allow_msaa = Some(rdr.read_bool()?);
            m_force_into_rt = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
            m_stereo_convergence = Some(rdr.read_f32()?);
            m_stereo_separation = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_target_display = Some(CameraMTargetDisplayVariant::parse(rdr, ver)?);
            m_target_eye = Some(rdr.read_i32()?);
            m_hdr = Some(rdr.read_bool()?);
            m_allow_msaa = Some(rdr.read_bool()?);
            m_force_into_rt = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
            m_stereo_convergence = Some(rdr.read_f32()?);
            m_stereo_separation = Some(rdr.read_f32()?);
            m_stereo_mirror_mode = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 3) {
            m_target_display = Some(CameraMTargetDisplayVariant::parse(rdr, ver)?);
            m_target_eye = Some(rdr.read_i32()?);
            m_hdr = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
            m_stereo_convergence = Some(rdr.read_f32()?);
            m_stereo_separation = Some(rdr.read_f32()?);
            m_stereo_mirror_mode = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_target_display = Some(CameraMTargetDisplayVariant::parse(rdr, ver)?);
            m_hdr = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
            m_stereo_convergence = Some(rdr.read_f32()?);
            m_stereo_separation = Some(rdr.read_f32()?);
            m_stereo_mirror_mode = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_target_display = Some(CameraMTargetDisplayVariant::parse(rdr, ver)?);
            m_hdr = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
            m_stereo_convergence = Some(rdr.read_f32()?);
            m_stereo_separation = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_hdr = Some(rdr.read_bool()?);
            m_occlusion_culling = Some(CameraMOcclusionCullingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_hdr = Some(rdr.read_bool()?);
        } else {
        }

        Ok(Camera {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_clear_flags: m_clear_flags.unwrap(),
            m_back_ground_color: m_back_ground_color.unwrap(),
            m_normalized_view_port_rect: m_normalized_view_port_rect.unwrap(),
            m_projection_matrix_mode,
            near_clip_plane: near_clip_plane.unwrap(),
            m_sensor_size,
            m_gate_fit_mode,
            far_clip_plane: far_clip_plane.unwrap(),
            m_lens_shift,
            m_iso,
            field_of_view: field_of_view.unwrap(),
            m_focal_length,
            m_shutter_speed,
            orthographic: orthographic.unwrap(),
            m_aperture,
            orthographic_size: orthographic_size.unwrap(),
            m_focus_distance,
            m_depth: m_depth.unwrap(),
            m_culling_mask: m_culling_mask.unwrap(),
            m_blade_count,
            m_rendering_path: m_rendering_path.unwrap(),
            m_curvature,
            m_target_texture: m_target_texture.unwrap(),
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
