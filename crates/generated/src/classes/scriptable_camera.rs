//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ScriptableCamera {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgbAf,
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_clear_flags = rdr.read_u32()?;
        let m_back_ground_color = ColorRgbAf::parse(rdr, ver)?;
        let m_projection_matrix_mode = rdr.read_i32()?;
        let m_gate_fit_mode = rdr.read_i32()?;
        rdr.align(4)?;
        let m_sensor_size = Vector2f::parse(rdr, ver)?;
        let m_lens_shift = Vector2f::parse(rdr, ver)?;
        let m_focal_length = rdr.read_f32()?;
        let m_normalized_view_port_rect = Rectf::parse(rdr, ver)?;
        let near_clip_plane = rdr.read_f32()?;
        let far_clip_plane = rdr.read_f32()?;
        let field_of_view = rdr.read_f32()?;
        let orthographic = rdr.read_bool()?;
        rdr.align(4)?;
        let orthographic_size = rdr.read_f32()?;
        let m_depth = rdr.read_f32()?;
        let m_culling_mask = BitField::parse(rdr, ver)?;
        let m_rendering_path = rdr.read_i32()?;
        let m_target_texture = crate::PPtr::parse(rdr, ver)?;
        let m_target_display = rdr.read_i32()?;
        let m_target_eye = rdr.read_i32()?;
        let m_hdr = rdr.read_bool()?;
        let m_allow_msaa = rdr.read_bool()?;
        let m_allow_dynamic_resolution = rdr.read_bool()?;
        let m_force_into_rt = rdr.read_bool()?;
        let m_occlusion_culling = rdr.read_bool()?;
        rdr.align(4)?;
        let m_stereo_convergence = rdr.read_f32()?;
        let m_stereo_separation = rdr.read_f32()?;
        let m_script = crate::PPtr::parse(rdr, ver)?;

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

impl crate::ToValue for ScriptableCamera {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_ClearFlags".to_string(), self.m_clear_flags.to_value())),
            Some((
                "m_BackGroundColor".to_string(),
                self.m_back_ground_color.to_value(),
            )),
            Some((
                "m_projectionMatrixMode".to_string(),
                self.m_projection_matrix_mode.to_value(),
            )),
            Some(("m_GateFitMode".to_string(), self.m_gate_fit_mode.to_value())),
            Some(("m_SensorSize".to_string(), self.m_sensor_size.to_value())),
            Some(("m_LensShift".to_string(), self.m_lens_shift.to_value())),
            Some(("m_FocalLength".to_string(), self.m_focal_length.to_value())),
            Some((
                "m_NormalizedViewPortRect".to_string(),
                self.m_normalized_view_port_rect.to_value(),
            )),
            Some((
                "near clip plane".to_string(),
                self.near_clip_plane.to_value(),
            )),
            Some(("far clip plane".to_string(), self.far_clip_plane.to_value())),
            Some(("field of view".to_string(), self.field_of_view.to_value())),
            Some(("orthographic".to_string(), self.orthographic.to_value())),
            Some((
                "orthographic size".to_string(),
                self.orthographic_size.to_value(),
            )),
            Some(("m_Depth".to_string(), self.m_depth.to_value())),
            Some(("m_CullingMask".to_string(), self.m_culling_mask.to_value())),
            Some((
                "m_RenderingPath".to_string(),
                self.m_rendering_path.to_value(),
            )),
            Some((
                "m_TargetTexture".to_string(),
                self.m_target_texture.to_value(),
            )),
            Some((
                "m_TargetDisplay".to_string(),
                self.m_target_display.to_value(),
            )),
            Some(("m_TargetEye".to_string(), self.m_target_eye.to_value())),
            Some(("m_HDR".to_string(), self.m_hdr.to_value())),
            Some(("m_AllowMSAA".to_string(), self.m_allow_msaa.to_value())),
            Some((
                "m_AllowDynamicResolution".to_string(),
                self.m_allow_dynamic_resolution.to_value(),
            )),
            Some(("m_ForceIntoRT".to_string(), self.m_force_into_rt.to_value())),
            Some((
                "m_OcclusionCulling".to_string(),
                self.m_occlusion_culling.to_value(),
            )),
            Some((
                "m_StereoConvergence".to_string(),
                self.m_stereo_convergence.to_value(),
            )),
            Some((
                "m_StereoSeparation".to_string(),
                self.m_stereo_separation.to_value(),
            )),
            Some(("m_Script".to_string(), self.m_script.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_ClearFlags" => (self.m_clear_flags).inspect_value(path, offset, limit),
                "m_BackGroundColor" => {
                    (self.m_back_ground_color).inspect_value(path, offset, limit)
                }
                "m_projectionMatrixMode" => {
                    (self.m_projection_matrix_mode).inspect_value(path, offset, limit)
                }
                "m_GateFitMode" => (self.m_gate_fit_mode).inspect_value(path, offset, limit),
                "m_SensorSize" => (self.m_sensor_size).inspect_value(path, offset, limit),
                "m_LensShift" => (self.m_lens_shift).inspect_value(path, offset, limit),
                "m_FocalLength" => (self.m_focal_length).inspect_value(path, offset, limit),
                "m_NormalizedViewPortRect" => {
                    (self.m_normalized_view_port_rect).inspect_value(path, offset, limit)
                }
                "near clip plane" => (self.near_clip_plane).inspect_value(path, offset, limit),
                "far clip plane" => (self.far_clip_plane).inspect_value(path, offset, limit),
                "field of view" => (self.field_of_view).inspect_value(path, offset, limit),
                "orthographic" => (self.orthographic).inspect_value(path, offset, limit),
                "orthographic size" => (self.orthographic_size).inspect_value(path, offset, limit),
                "m_Depth" => (self.m_depth).inspect_value(path, offset, limit),
                "m_CullingMask" => (self.m_culling_mask).inspect_value(path, offset, limit),
                "m_RenderingPath" => (self.m_rendering_path).inspect_value(path, offset, limit),
                "m_TargetTexture" => (self.m_target_texture).inspect_value(path, offset, limit),
                "m_TargetDisplay" => (self.m_target_display).inspect_value(path, offset, limit),
                "m_TargetEye" => (self.m_target_eye).inspect_value(path, offset, limit),
                "m_HDR" => (self.m_hdr).inspect_value(path, offset, limit),
                "m_AllowMSAA" => (self.m_allow_msaa).inspect_value(path, offset, limit),
                "m_AllowDynamicResolution" => {
                    (self.m_allow_dynamic_resolution).inspect_value(path, offset, limit)
                }
                "m_ForceIntoRT" => (self.m_force_into_rt).inspect_value(path, offset, limit),
                "m_OcclusionCulling" => {
                    (self.m_occlusion_culling).inspect_value(path, offset, limit)
                }
                "m_StereoConvergence" => {
                    (self.m_stereo_convergence).inspect_value(path, offset, limit)
                }
                "m_StereoSeparation" => {
                    (self.m_stereo_separation).inspect_value(path, offset, limit)
                }
                "m_Script" => (self.m_script).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_clear_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_ClearFlags".to_string(), v)),
            ((self.m_back_ground_color).inspect_value(path, offset, limit))
                .map(|v| ("m_BackGroundColor".to_string(), v)),
            ((self.m_projection_matrix_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_projectionMatrixMode".to_string(), v)),
            ((self.m_gate_fit_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_GateFitMode".to_string(), v)),
            ((self.m_sensor_size).inspect_value(path, offset, limit))
                .map(|v| ("m_SensorSize".to_string(), v)),
            ((self.m_lens_shift).inspect_value(path, offset, limit))
                .map(|v| ("m_LensShift".to_string(), v)),
            ((self.m_focal_length).inspect_value(path, offset, limit))
                .map(|v| ("m_FocalLength".to_string(), v)),
            ((self.m_normalized_view_port_rect).inspect_value(path, offset, limit))
                .map(|v| ("m_NormalizedViewPortRect".to_string(), v)),
            ((self.near_clip_plane).inspect_value(path, offset, limit))
                .map(|v| ("near clip plane".to_string(), v)),
            ((self.far_clip_plane).inspect_value(path, offset, limit))
                .map(|v| ("far clip plane".to_string(), v)),
            ((self.field_of_view).inspect_value(path, offset, limit))
                .map(|v| ("field of view".to_string(), v)),
            ((self.orthographic).inspect_value(path, offset, limit))
                .map(|v| ("orthographic".to_string(), v)),
            ((self.orthographic_size).inspect_value(path, offset, limit))
                .map(|v| ("orthographic size".to_string(), v)),
            ((self.m_depth).inspect_value(path, offset, limit)).map(|v| ("m_Depth".to_string(), v)),
            ((self.m_culling_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingMask".to_string(), v)),
            ((self.m_rendering_path).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingPath".to_string(), v)),
            ((self.m_target_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetTexture".to_string(), v)),
            ((self.m_target_display).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetDisplay".to_string(), v)),
            ((self.m_target_eye).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetEye".to_string(), v)),
            ((self.m_hdr).inspect_value(path, offset, limit)).map(|v| ("m_HDR".to_string(), v)),
            ((self.m_allow_msaa).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowMSAA".to_string(), v)),
            ((self.m_allow_dynamic_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowDynamicResolution".to_string(), v)),
            ((self.m_force_into_rt).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceIntoRT".to_string(), v)),
            ((self.m_occlusion_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_OcclusionCulling".to_string(), v)),
            ((self.m_stereo_convergence).inspect_value(path, offset, limit))
                .map(|v| ("m_StereoConvergence".to_string(), v)),
            ((self.m_stereo_separation).inspect_value(path, offset, limit))
                .map(|v| ("m_StereoSeparation".to_string(), v)),
            ((self.m_script).inspect_value(path, offset, limit))
                .map(|v| ("m_Script".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
