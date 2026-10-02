//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：CanvasMOverridePixelPerfectVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CanvasMOverridePixelPerfectVariant {
    Default(bool),
    V5_3_4(bool),
}

impl crate::Parse for CanvasMOverridePixelPerfectVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 4) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_3_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CanvasMOverridePixelPerfectVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CanvasMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CanvasMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for CanvasMSortingLayerIdVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CanvasMSortingLayerIdVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CanvasMSortingBucketNormalizedSizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CanvasMSortingBucketNormalizedSizeVariant {
    Default(f32),
    V5_6(f32),
}

impl crate::Parse for CanvasMSortingBucketNormalizedSizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CanvasMSortingBucketNormalizedSizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CanvasMAdditionalShaderChannelsFlagVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CanvasMAdditionalShaderChannelsFlagVariant {
    Default(i32),
    V2022_1(i32),
}

impl crate::Parse for CanvasMAdditionalShaderChannelsFlagVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CanvasMAdditionalShaderChannelsFlagVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CanvasMVertexColorAlwaysGammaSpaceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CanvasMVertexColorAlwaysGammaSpaceVariant {
    Default(bool),
    V6000_4(bool),
}

impl crate::Parse for CanvasMVertexColorAlwaysGammaSpaceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CanvasMVertexColorAlwaysGammaSpaceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Canvas {
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
    pub m_override_pixel_perfect: Option<CanvasMOverridePixelPerfectVariant>,
    pub m_sorting_layer_id: Option<CanvasMSortingLayerIdVariant>,
    pub m_sorting_bucket_normalized_size: Option<CanvasMSortingBucketNormalizedSizeVariant>,
    pub m_sorting_order: Option<i16>,
    pub m_additional_shader_channels_flag: Option<CanvasMAdditionalShaderChannelsFlagVariant>,
    pub m_vertex_color_always_gamma_space: Option<CanvasMVertexColorAlwaysGammaSpaceVariant>,
    pub m_target_display: Option<i8>,
    pub m_update_rect_transform_for_standalone: Option<i32>,
    pub m_use_reflection_probes: Option<bool>,
}

impl crate::Parse for Canvas {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_alpha = None;
        let mut m_render_mode = None;
        let mut m_camera = None;
        let mut m_plane_distance = None;
        let mut m_normals = None;
        let mut m_pixel_perfect = None;
        let mut m_position_u_vs = None;
        let mut m_receives_events = None;
        let mut m_override_sorting = None;
        let mut m_override_pixel_perfect = None;
        let mut m_sorting_layer_id = None;
        let mut m_sorting_bucket_normalized_size = None;
        let mut m_sorting_order = None;
        let mut m_additional_shader_channels_flag = None;
        let mut m_vertex_color_always_gamma_space = None;
        let mut m_target_display = None;
        let mut m_update_rect_transform_for_standalone = None;
        let mut m_use_reflection_probes = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(4, 6, 0) {
        } else {
            m_alpha = Some(rdr.read_f32()?);
        }
        m_render_mode = Some(rdr.read_i32()?);
        m_camera = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 0) {
            m_plane_distance = Some(rdr.read_f32()?);
        } else {
            m_normals = Some(rdr.read_bool()?);
            m_position_u_vs = Some(rdr.read_bool()?);
        }
        m_pixel_perfect = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_bucket_normalized_size =
                Some(CanvasMSortingBucketNormalizedSizeVariant::parse(rdr, ver)?);
            m_vertex_color_always_gamma_space =
                Some(CanvasMVertexColorAlwaysGammaSpaceVariant::parse(rdr, ver)?);
            m_use_reflection_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_shader_channels_flag =
                Some(CanvasMAdditionalShaderChannelsFlagVariant::parse(rdr, ver)?);
            m_update_rect_transform_for_standalone = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 15) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_bucket_normalized_size =
                Some(CanvasMSortingBucketNormalizedSizeVariant::parse(rdr, ver)?);
            m_vertex_color_always_gamma_space =
                Some(CanvasMVertexColorAlwaysGammaSpaceVariant::parse(rdr, ver)?);
            m_additional_shader_channels_flag =
                Some(CanvasMAdditionalShaderChannelsFlagVariant::parse(rdr, ver)?);
            m_update_rect_transform_for_standalone = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_bucket_normalized_size =
                Some(CanvasMSortingBucketNormalizedSizeVariant::parse(rdr, ver)?);
            m_additional_shader_channels_flag =
                Some(CanvasMAdditionalShaderChannelsFlagVariant::parse(rdr, ver)?);
            m_update_rect_transform_for_standalone = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 34) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_bucket_normalized_size =
                Some(CanvasMSortingBucketNormalizedSizeVariant::parse(rdr, ver)?);
            m_vertex_color_always_gamma_space =
                Some(CanvasMVertexColorAlwaysGammaSpaceVariant::parse(rdr, ver)?);
            m_additional_shader_channels_flag =
                Some(CanvasMAdditionalShaderChannelsFlagVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_bucket_normalized_size =
                Some(CanvasMSortingBucketNormalizedSizeVariant::parse(rdr, ver)?);
            m_additional_shader_channels_flag =
                Some(CanvasMAdditionalShaderChannelsFlagVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_bucket_normalized_size =
                Some(CanvasMSortingBucketNormalizedSizeVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
            m_target_display = Some(rdr.read_i8()?);
        } else if *ver >= crate::UnityVersion::new(4, 6, 0) {
            m_receives_events = Some(rdr.read_bool()?);
            m_override_sorting = Some(rdr.read_bool()?);
            m_override_pixel_perfect = Some(CanvasMOverridePixelPerfectVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(CanvasMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(rdr.read_i16()?);
        } else {
        }

        Ok(Canvas {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_alpha,
            m_render_mode: m_render_mode.unwrap(),
            m_camera: m_camera.unwrap(),
            m_plane_distance,
            m_normals,
            m_pixel_perfect: m_pixel_perfect.unwrap(),
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

impl crate::ToValue for Canvas {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_Alpha", &self.m_alpha),
            Some(("m_RenderMode".to_string(), self.m_render_mode.to_value())),
            Some(("m_Camera".to_string(), self.m_camera.to_value())),
            crate::opt_field("m_PlaneDistance", &self.m_plane_distance),
            crate::opt_field("m_Normals", &self.m_normals),
            Some((
                "m_PixelPerfect".to_string(),
                self.m_pixel_perfect.to_value(),
            )),
            crate::opt_field("m_PositionUVs", &self.m_position_u_vs),
            crate::opt_field("m_ReceivesEvents", &self.m_receives_events),
            crate::opt_field("m_OverrideSorting", &self.m_override_sorting),
            crate::opt_field("m_OverridePixelPerfect", &self.m_override_pixel_perfect),
            crate::opt_field("m_SortingLayerID", &self.m_sorting_layer_id),
            crate::opt_field(
                "m_SortingBucketNormalizedSize",
                &self.m_sorting_bucket_normalized_size,
            ),
            crate::opt_field("m_SortingOrder", &self.m_sorting_order),
            crate::opt_field(
                "m_AdditionalShaderChannelsFlag",
                &self.m_additional_shader_channels_flag,
            ),
            crate::opt_field(
                "m_VertexColorAlwaysGammaSpace",
                &self.m_vertex_color_always_gamma_space,
            ),
            crate::opt_field("m_TargetDisplay", &self.m_target_display),
            crate::opt_field(
                "m_UpdateRectTransformForStandalone",
                &self.m_update_rect_transform_for_standalone,
            ),
            crate::opt_field("m_UseReflectionProbes", &self.m_use_reflection_probes),
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
                "m_Alpha" => (self.m_alpha).inspect_value(path, offset, limit),
                "m_RenderMode" => (self.m_render_mode).inspect_value(path, offset, limit),
                "m_Camera" => (self.m_camera).inspect_value(path, offset, limit),
                "m_PlaneDistance" => (self.m_plane_distance).inspect_value(path, offset, limit),
                "m_Normals" => (self.m_normals).inspect_value(path, offset, limit),
                "m_PixelPerfect" => (self.m_pixel_perfect).inspect_value(path, offset, limit),
                "m_PositionUVs" => (self.m_position_u_vs).inspect_value(path, offset, limit),
                "m_ReceivesEvents" => (self.m_receives_events).inspect_value(path, offset, limit),
                "m_OverrideSorting" => (self.m_override_sorting).inspect_value(path, offset, limit),
                "m_OverridePixelPerfect" => {
                    (self.m_override_pixel_perfect).inspect_value(path, offset, limit)
                }
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_SortingBucketNormalizedSize" => {
                    (self.m_sorting_bucket_normalized_size).inspect_value(path, offset, limit)
                }
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                "m_AdditionalShaderChannelsFlag" => {
                    (self.m_additional_shader_channels_flag).inspect_value(path, offset, limit)
                }
                "m_VertexColorAlwaysGammaSpace" => {
                    (self.m_vertex_color_always_gamma_space).inspect_value(path, offset, limit)
                }
                "m_TargetDisplay" => (self.m_target_display).inspect_value(path, offset, limit),
                "m_UpdateRectTransformForStandalone" => {
                    (self.m_update_rect_transform_for_standalone).inspect_value(path, offset, limit)
                }
                "m_UseReflectionProbes" => {
                    (self.m_use_reflection_probes).inspect_value(path, offset, limit)
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
            ((self.m_alpha).inspect_value(path, offset, limit)).map(|v| ("m_Alpha".to_string(), v)),
            ((self.m_render_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderMode".to_string(), v)),
            ((self.m_camera).inspect_value(path, offset, limit))
                .map(|v| ("m_Camera".to_string(), v)),
            ((self.m_plane_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_PlaneDistance".to_string(), v)),
            ((self.m_normals).inspect_value(path, offset, limit))
                .map(|v| ("m_Normals".to_string(), v)),
            ((self.m_pixel_perfect).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelPerfect".to_string(), v)),
            ((self.m_position_u_vs).inspect_value(path, offset, limit))
                .map(|v| ("m_PositionUVs".to_string(), v)),
            ((self.m_receives_events).inspect_value(path, offset, limit))
                .map(|v| ("m_ReceivesEvents".to_string(), v)),
            ((self.m_override_sorting).inspect_value(path, offset, limit))
                .map(|v| ("m_OverrideSorting".to_string(), v)),
            ((self.m_override_pixel_perfect).inspect_value(path, offset, limit))
                .map(|v| ("m_OverridePixelPerfect".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_sorting_bucket_normalized_size).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingBucketNormalizedSize".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
            ((self.m_additional_shader_channels_flag).inspect_value(path, offset, limit))
                .map(|v| ("m_AdditionalShaderChannelsFlag".to_string(), v)),
            ((self.m_vertex_color_always_gamma_space).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexColorAlwaysGammaSpace".to_string(), v)),
            ((self.m_target_display).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetDisplay".to_string(), v)),
            ((self.m_update_rect_transform_for_standalone).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateRectTransformForStandalone".to_string(), v)),
            ((self.m_use_reflection_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_UseReflectionProbes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
