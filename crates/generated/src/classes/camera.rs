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

impl crate::ToValue for CameraMProjectionMatrixModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for CameraMGateFitModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for CameraMTargetDisplayVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for CameraMOcclusionCullingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for Camera {
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
                "m_NormalizedViewPortRect".to_string(),
                self.m_normalized_view_port_rect.to_value(),
            )),
            crate::opt_field("m_projectionMatrixMode", &self.m_projection_matrix_mode),
            Some((
                "near clip plane".to_string(),
                self.near_clip_plane.to_value(),
            )),
            crate::opt_field("m_SensorSize", &self.m_sensor_size),
            crate::opt_field("m_GateFitMode", &self.m_gate_fit_mode),
            Some(("far clip plane".to_string(), self.far_clip_plane.to_value())),
            crate::opt_field("m_LensShift", &self.m_lens_shift),
            crate::opt_field("m_Iso", &self.m_iso),
            Some(("field of view".to_string(), self.field_of_view.to_value())),
            crate::opt_field("m_FocalLength", &self.m_focal_length),
            crate::opt_field("m_ShutterSpeed", &self.m_shutter_speed),
            Some(("orthographic".to_string(), self.orthographic.to_value())),
            crate::opt_field("m_Aperture", &self.m_aperture),
            Some((
                "orthographic size".to_string(),
                self.orthographic_size.to_value(),
            )),
            crate::opt_field("m_FocusDistance", &self.m_focus_distance),
            Some(("m_Depth".to_string(), self.m_depth.to_value())),
            Some(("m_CullingMask".to_string(), self.m_culling_mask.to_value())),
            crate::opt_field("m_BladeCount", &self.m_blade_count),
            Some((
                "m_RenderingPath".to_string(),
                self.m_rendering_path.to_value(),
            )),
            crate::opt_field("m_Curvature", &self.m_curvature),
            Some((
                "m_TargetTexture".to_string(),
                self.m_target_texture.to_value(),
            )),
            crate::opt_field("m_BarrelClipping", &self.m_barrel_clipping),
            crate::opt_field("m_HDR", &self.m_hdr),
            crate::opt_field("m_TargetDisplay", &self.m_target_display),
            crate::opt_field("m_Anamorphism", &self.m_anamorphism),
            crate::opt_field("m_OcclusionCulling", &self.m_occlusion_culling),
            crate::opt_field("m_TargetEye", &self.m_target_eye),
            crate::opt_field("m_StereoConvergence", &self.m_stereo_convergence),
            crate::opt_field("m_AllowMSAA", &self.m_allow_msaa),
            crate::opt_field("m_StereoSeparation", &self.m_stereo_separation),
            crate::opt_field("m_ForceIntoRT", &self.m_force_into_rt),
            crate::opt_field("m_AllowDynamicResolution", &self.m_allow_dynamic_resolution),
            crate::opt_field("m_StereoMirrorMode", &self.m_stereo_mirror_mode),
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
                "m_NormalizedViewPortRect" => {
                    (self.m_normalized_view_port_rect).inspect_value(path, offset, limit)
                }
                "m_projectionMatrixMode" => {
                    (self.m_projection_matrix_mode).inspect_value(path, offset, limit)
                }
                "near clip plane" => (self.near_clip_plane).inspect_value(path, offset, limit),
                "m_SensorSize" => (self.m_sensor_size).inspect_value(path, offset, limit),
                "m_GateFitMode" => (self.m_gate_fit_mode).inspect_value(path, offset, limit),
                "far clip plane" => (self.far_clip_plane).inspect_value(path, offset, limit),
                "m_LensShift" => (self.m_lens_shift).inspect_value(path, offset, limit),
                "m_Iso" => (self.m_iso).inspect_value(path, offset, limit),
                "field of view" => (self.field_of_view).inspect_value(path, offset, limit),
                "m_FocalLength" => (self.m_focal_length).inspect_value(path, offset, limit),
                "m_ShutterSpeed" => (self.m_shutter_speed).inspect_value(path, offset, limit),
                "orthographic" => (self.orthographic).inspect_value(path, offset, limit),
                "m_Aperture" => (self.m_aperture).inspect_value(path, offset, limit),
                "orthographic size" => (self.orthographic_size).inspect_value(path, offset, limit),
                "m_FocusDistance" => (self.m_focus_distance).inspect_value(path, offset, limit),
                "m_Depth" => (self.m_depth).inspect_value(path, offset, limit),
                "m_CullingMask" => (self.m_culling_mask).inspect_value(path, offset, limit),
                "m_BladeCount" => (self.m_blade_count).inspect_value(path, offset, limit),
                "m_RenderingPath" => (self.m_rendering_path).inspect_value(path, offset, limit),
                "m_Curvature" => (self.m_curvature).inspect_value(path, offset, limit),
                "m_TargetTexture" => (self.m_target_texture).inspect_value(path, offset, limit),
                "m_BarrelClipping" => (self.m_barrel_clipping).inspect_value(path, offset, limit),
                "m_HDR" => (self.m_hdr).inspect_value(path, offset, limit),
                "m_TargetDisplay" => (self.m_target_display).inspect_value(path, offset, limit),
                "m_Anamorphism" => (self.m_anamorphism).inspect_value(path, offset, limit),
                "m_OcclusionCulling" => {
                    (self.m_occlusion_culling).inspect_value(path, offset, limit)
                }
                "m_TargetEye" => (self.m_target_eye).inspect_value(path, offset, limit),
                "m_StereoConvergence" => {
                    (self.m_stereo_convergence).inspect_value(path, offset, limit)
                }
                "m_AllowMSAA" => (self.m_allow_msaa).inspect_value(path, offset, limit),
                "m_StereoSeparation" => {
                    (self.m_stereo_separation).inspect_value(path, offset, limit)
                }
                "m_ForceIntoRT" => (self.m_force_into_rt).inspect_value(path, offset, limit),
                "m_AllowDynamicResolution" => {
                    (self.m_allow_dynamic_resolution).inspect_value(path, offset, limit)
                }
                "m_StereoMirrorMode" => {
                    (self.m_stereo_mirror_mode).inspect_value(path, offset, limit)
                }
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
            ((self.m_normalized_view_port_rect).inspect_value(path, offset, limit))
                .map(|v| ("m_NormalizedViewPortRect".to_string(), v)),
            ((self.m_projection_matrix_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_projectionMatrixMode".to_string(), v)),
            ((self.near_clip_plane).inspect_value(path, offset, limit))
                .map(|v| ("near clip plane".to_string(), v)),
            ((self.m_sensor_size).inspect_value(path, offset, limit))
                .map(|v| ("m_SensorSize".to_string(), v)),
            ((self.m_gate_fit_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_GateFitMode".to_string(), v)),
            ((self.far_clip_plane).inspect_value(path, offset, limit))
                .map(|v| ("far clip plane".to_string(), v)),
            ((self.m_lens_shift).inspect_value(path, offset, limit))
                .map(|v| ("m_LensShift".to_string(), v)),
            ((self.m_iso).inspect_value(path, offset, limit)).map(|v| ("m_Iso".to_string(), v)),
            ((self.field_of_view).inspect_value(path, offset, limit))
                .map(|v| ("field of view".to_string(), v)),
            ((self.m_focal_length).inspect_value(path, offset, limit))
                .map(|v| ("m_FocalLength".to_string(), v)),
            ((self.m_shutter_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_ShutterSpeed".to_string(), v)),
            ((self.orthographic).inspect_value(path, offset, limit))
                .map(|v| ("orthographic".to_string(), v)),
            ((self.m_aperture).inspect_value(path, offset, limit))
                .map(|v| ("m_Aperture".to_string(), v)),
            ((self.orthographic_size).inspect_value(path, offset, limit))
                .map(|v| ("orthographic size".to_string(), v)),
            ((self.m_focus_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_FocusDistance".to_string(), v)),
            ((self.m_depth).inspect_value(path, offset, limit)).map(|v| ("m_Depth".to_string(), v)),
            ((self.m_culling_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingMask".to_string(), v)),
            ((self.m_blade_count).inspect_value(path, offset, limit))
                .map(|v| ("m_BladeCount".to_string(), v)),
            ((self.m_rendering_path).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingPath".to_string(), v)),
            ((self.m_curvature).inspect_value(path, offset, limit))
                .map(|v| ("m_Curvature".to_string(), v)),
            ((self.m_target_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetTexture".to_string(), v)),
            ((self.m_barrel_clipping).inspect_value(path, offset, limit))
                .map(|v| ("m_BarrelClipping".to_string(), v)),
            ((self.m_hdr).inspect_value(path, offset, limit)).map(|v| ("m_HDR".to_string(), v)),
            ((self.m_target_display).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetDisplay".to_string(), v)),
            ((self.m_anamorphism).inspect_value(path, offset, limit))
                .map(|v| ("m_Anamorphism".to_string(), v)),
            ((self.m_occlusion_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_OcclusionCulling".to_string(), v)),
            ((self.m_target_eye).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetEye".to_string(), v)),
            ((self.m_stereo_convergence).inspect_value(path, offset, limit))
                .map(|v| ("m_StereoConvergence".to_string(), v)),
            ((self.m_allow_msaa).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowMSAA".to_string(), v)),
            ((self.m_stereo_separation).inspect_value(path, offset, limit))
                .map(|v| ("m_StereoSeparation".to_string(), v)),
            ((self.m_force_into_rt).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceIntoRT".to_string(), v)),
            ((self.m_allow_dynamic_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowDynamicResolution".to_string(), v)),
            ((self.m_stereo_mirror_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_StereoMirrorMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
