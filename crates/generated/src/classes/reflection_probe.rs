//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：ReflectionProbeMImportanceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ReflectionProbeMImportanceVariant {
    Default(i32),
    V5_4(i16),
}

impl crate::Parse for ReflectionProbeMImportanceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ReflectionProbeMImportanceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ReflectionProbeMUseOcclusionCullingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ReflectionProbeMUseOcclusionCullingVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for ReflectionProbeMUseOcclusionCullingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ReflectionProbeMUseOcclusionCullingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ReflectionProbe {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_type: i32,
    pub m_mode: i32,
    pub m_refresh_mode: i32,
    pub m_time_slicing_mode: i32,
    pub m_resolution: i32,
    pub m_update_frequency: i32,
    pub m_importance: ReflectionProbeMImportanceVariant,
    pub m_box_size: Vector3f,
    pub m_box_offset: Vector3f,
    pub m_near_clip: f32,
    pub m_far_clip: f32,
    pub m_shadow_distance: f32,
    pub m_clear_flags: u32,
    pub m_back_ground_color: ColorRgbAf,
    pub m_culling_mask: BitField,
    pub m_intensity_multiplier: f32,
    pub m_blend_distance: Option<f32>,
    pub m_hdr: bool,
    pub m_box_projection: bool,
    pub m_render_dynamic_objects: bool,
    pub m_use_occlusion_culling: ReflectionProbeMUseOcclusionCullingVariant,
    pub m_custom_baked_texture: PPtr,
    pub m_baked_texture: PPtr,
}

impl crate::Parse for ReflectionProbe {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_type = None;
        let mut m_mode = None;
        let mut m_refresh_mode = None;
        let mut m_time_slicing_mode = None;
        let mut m_resolution = None;
        let mut m_update_frequency = None;
        let mut m_importance = None;
        let mut m_box_size = None;
        let mut m_box_offset = None;
        let mut m_near_clip = None;
        let mut m_far_clip = None;
        let mut m_shadow_distance = None;
        let mut m_clear_flags = None;
        let mut m_back_ground_color = None;
        let mut m_culling_mask = None;
        let mut m_intensity_multiplier = None;
        let mut m_blend_distance = None;
        let mut m_hdr = None;
        let mut m_box_projection = None;
        let mut m_render_dynamic_objects = None;
        let mut m_use_occlusion_culling = None;
        let mut m_custom_baked_texture = None;
        let mut m_baked_texture = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_type = Some(rdr.read_i32()?);
        m_mode = Some(rdr.read_i32()?);
        m_refresh_mode = Some(rdr.read_i32()?);
        m_time_slicing_mode = Some(rdr.read_i32()?);
        m_resolution = Some(rdr.read_i32()?);
        m_update_frequency = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            m_importance = Some(ReflectionProbeMImportanceVariant::parse(rdr, ver)?);
        }
        m_box_size = Some(Vector3f::parse(rdr, ver)?);
        m_box_offset = Some(Vector3f::parse(rdr, ver)?);
        m_near_clip = Some(rdr.read_f32()?);
        m_far_clip = Some(rdr.read_f32()?);
        m_shadow_distance = Some(rdr.read_f32()?);
        m_clear_flags = Some(rdr.read_u32()?);
        m_back_ground_color = Some(ColorRgbAf::parse(rdr, ver)?);
        m_culling_mask = Some(BitField::parse(rdr, ver)?);
        m_intensity_multiplier = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_blend_distance = Some(rdr.read_f32()?);
        } else {
        }
        m_hdr = Some(rdr.read_bool()?);
        m_box_projection = Some(rdr.read_bool()?);
        m_render_dynamic_objects = Some(rdr.read_bool()?);
        m_use_occlusion_culling =
            Some(ReflectionProbeMUseOcclusionCullingVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_importance = Some(ReflectionProbeMImportanceVariant::parse(rdr, ver)?);
        } else {
        }
        m_custom_baked_texture = Some(crate::PPtr::parse(rdr, ver)?);
        m_baked_texture = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(ReflectionProbe {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_type: m_type.unwrap(),
            m_mode: m_mode.unwrap(),
            m_refresh_mode: m_refresh_mode.unwrap(),
            m_time_slicing_mode: m_time_slicing_mode.unwrap(),
            m_resolution: m_resolution.unwrap(),
            m_update_frequency: m_update_frequency.unwrap(),
            m_importance: m_importance.unwrap(),
            m_box_size: m_box_size.unwrap(),
            m_box_offset: m_box_offset.unwrap(),
            m_near_clip: m_near_clip.unwrap(),
            m_far_clip: m_far_clip.unwrap(),
            m_shadow_distance: m_shadow_distance.unwrap(),
            m_clear_flags: m_clear_flags.unwrap(),
            m_back_ground_color: m_back_ground_color.unwrap(),
            m_culling_mask: m_culling_mask.unwrap(),
            m_intensity_multiplier: m_intensity_multiplier.unwrap(),
            m_blend_distance,
            m_hdr: m_hdr.unwrap(),
            m_box_projection: m_box_projection.unwrap(),
            m_render_dynamic_objects: m_render_dynamic_objects.unwrap(),
            m_use_occlusion_culling: m_use_occlusion_culling.unwrap(),
            m_custom_baked_texture: m_custom_baked_texture.unwrap(),
            m_baked_texture: m_baked_texture.unwrap(),
        })
    }
}

impl crate::ToValue for ReflectionProbe {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_Mode".to_string(), self.m_mode.to_value())),
            Some(("m_RefreshMode".to_string(), self.m_refresh_mode.to_value())),
            Some((
                "m_TimeSlicingMode".to_string(),
                self.m_time_slicing_mode.to_value(),
            )),
            Some(("m_Resolution".to_string(), self.m_resolution.to_value())),
            Some((
                "m_UpdateFrequency".to_string(),
                self.m_update_frequency.to_value(),
            )),
            Some(("m_Importance".to_string(), self.m_importance.to_value())),
            Some(("m_BoxSize".to_string(), self.m_box_size.to_value())),
            Some(("m_BoxOffset".to_string(), self.m_box_offset.to_value())),
            Some(("m_NearClip".to_string(), self.m_near_clip.to_value())),
            Some(("m_FarClip".to_string(), self.m_far_clip.to_value())),
            Some((
                "m_ShadowDistance".to_string(),
                self.m_shadow_distance.to_value(),
            )),
            Some(("m_ClearFlags".to_string(), self.m_clear_flags.to_value())),
            Some((
                "m_BackGroundColor".to_string(),
                self.m_back_ground_color.to_value(),
            )),
            Some(("m_CullingMask".to_string(), self.m_culling_mask.to_value())),
            Some((
                "m_IntensityMultiplier".to_string(),
                self.m_intensity_multiplier.to_value(),
            )),
            crate::opt_field("m_BlendDistance", &self.m_blend_distance),
            Some(("m_HDR".to_string(), self.m_hdr.to_value())),
            Some((
                "m_BoxProjection".to_string(),
                self.m_box_projection.to_value(),
            )),
            Some((
                "m_RenderDynamicObjects".to_string(),
                self.m_render_dynamic_objects.to_value(),
            )),
            Some((
                "m_UseOcclusionCulling".to_string(),
                self.m_use_occlusion_culling.to_value(),
            )),
            Some((
                "m_CustomBakedTexture".to_string(),
                self.m_custom_baked_texture.to_value(),
            )),
            Some((
                "m_BakedTexture".to_string(),
                self.m_baked_texture.to_value(),
            )),
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
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Mode" => (self.m_mode).inspect_value(path, offset, limit),
                "m_RefreshMode" => (self.m_refresh_mode).inspect_value(path, offset, limit),
                "m_TimeSlicingMode" => {
                    (self.m_time_slicing_mode).inspect_value(path, offset, limit)
                }
                "m_Resolution" => (self.m_resolution).inspect_value(path, offset, limit),
                "m_UpdateFrequency" => (self.m_update_frequency).inspect_value(path, offset, limit),
                "m_Importance" => (self.m_importance).inspect_value(path, offset, limit),
                "m_BoxSize" => (self.m_box_size).inspect_value(path, offset, limit),
                "m_BoxOffset" => (self.m_box_offset).inspect_value(path, offset, limit),
                "m_NearClip" => (self.m_near_clip).inspect_value(path, offset, limit),
                "m_FarClip" => (self.m_far_clip).inspect_value(path, offset, limit),
                "m_ShadowDistance" => (self.m_shadow_distance).inspect_value(path, offset, limit),
                "m_ClearFlags" => (self.m_clear_flags).inspect_value(path, offset, limit),
                "m_BackGroundColor" => {
                    (self.m_back_ground_color).inspect_value(path, offset, limit)
                }
                "m_CullingMask" => (self.m_culling_mask).inspect_value(path, offset, limit),
                "m_IntensityMultiplier" => {
                    (self.m_intensity_multiplier).inspect_value(path, offset, limit)
                }
                "m_BlendDistance" => (self.m_blend_distance).inspect_value(path, offset, limit),
                "m_HDR" => (self.m_hdr).inspect_value(path, offset, limit),
                "m_BoxProjection" => (self.m_box_projection).inspect_value(path, offset, limit),
                "m_RenderDynamicObjects" => {
                    (self.m_render_dynamic_objects).inspect_value(path, offset, limit)
                }
                "m_UseOcclusionCulling" => {
                    (self.m_use_occlusion_culling).inspect_value(path, offset, limit)
                }
                "m_CustomBakedTexture" => {
                    (self.m_custom_baked_texture).inspect_value(path, offset, limit)
                }
                "m_BakedTexture" => (self.m_baked_texture).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_mode).inspect_value(path, offset, limit)).map(|v| ("m_Mode".to_string(), v)),
            ((self.m_refresh_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RefreshMode".to_string(), v)),
            ((self.m_time_slicing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeSlicingMode".to_string(), v)),
            ((self.m_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_Resolution".to_string(), v)),
            ((self.m_update_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateFrequency".to_string(), v)),
            ((self.m_importance).inspect_value(path, offset, limit))
                .map(|v| ("m_Importance".to_string(), v)),
            ((self.m_box_size).inspect_value(path, offset, limit))
                .map(|v| ("m_BoxSize".to_string(), v)),
            ((self.m_box_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_BoxOffset".to_string(), v)),
            ((self.m_near_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_NearClip".to_string(), v)),
            ((self.m_far_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_FarClip".to_string(), v)),
            ((self.m_shadow_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_ShadowDistance".to_string(), v)),
            ((self.m_clear_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_ClearFlags".to_string(), v)),
            ((self.m_back_ground_color).inspect_value(path, offset, limit))
                .map(|v| ("m_BackGroundColor".to_string(), v)),
            ((self.m_culling_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingMask".to_string(), v)),
            ((self.m_intensity_multiplier).inspect_value(path, offset, limit))
                .map(|v| ("m_IntensityMultiplier".to_string(), v)),
            ((self.m_blend_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendDistance".to_string(), v)),
            ((self.m_hdr).inspect_value(path, offset, limit)).map(|v| ("m_HDR".to_string(), v)),
            ((self.m_box_projection).inspect_value(path, offset, limit))
                .map(|v| ("m_BoxProjection".to_string(), v)),
            ((self.m_render_dynamic_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderDynamicObjects".to_string(), v)),
            ((self.m_use_occlusion_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_UseOcclusionCulling".to_string(), v)),
            ((self.m_custom_baked_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomBakedTexture".to_string(), v)),
            ((self.m_baked_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedTexture".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
