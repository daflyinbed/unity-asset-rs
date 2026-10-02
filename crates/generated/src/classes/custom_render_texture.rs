//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::update_zone_info::UpdateZoneInfo;

/// 版本枚举：CustomRenderTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for CustomRenderTextureMDownscaleFallbackVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CustomRenderTextureMDownscaleFallbackVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CustomRenderTextureMSrgbVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMSrgbVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for CustomRenderTextureMSrgbVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CustomRenderTextureMSrgbVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CustomRenderTextureMBindMsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMBindMsVariant {
    Default(bool),
    V2019_1(bool),
}

impl crate::Parse for CustomRenderTextureMBindMsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CustomRenderTextureMBindMsVariant {
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

/// 版本枚举：CustomRenderTextureMEnableCompatibleFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMEnableCompatibleFormatVariant {
    Default(bool),
    V2021_3_29(bool),
    V2022_1(bool),
    V2022_3_6(bool),
    V2023_1(bool),
    V2023_1_5(bool),
}

impl crate::Parse for CustomRenderTextureMEnableCompatibleFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 5) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_1_5(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_3_29(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CustomRenderTextureMEnableCompatibleFormatVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_3_29(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
            Self::V2022_3_6(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
            Self::V2023_1_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_29(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_6(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CustomRenderTextureMUpdateZonesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CustomRenderTextureMUpdateZonesVariant {
    Default(Vec<UpdateZoneInfo>),
    V2017_1(Vec<UpdateZoneInfo>),
}

impl crate::Parse for CustomRenderTextureMUpdateZonesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| UpdateZoneInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| UpdateZoneInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CustomRenderTextureMUpdateZonesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomRenderTexture {
    pub m_name: String,
    pub m_width: i32,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_height: i32,
    pub m_downscale_fallback: Option<CustomRenderTextureMDownscaleFallbackVariant>,
    pub m_anti_aliasing: i32,
    pub m_depth_format: Option<i32>,
    pub m_color_format: i32,
    pub m_mip_count: Option<i32>,
    pub m_mip_map: bool,
    pub m_depth_stencil_format: Option<i32>,
    pub m_generate_mips: bool,
    pub m_srgb: CustomRenderTextureMSrgbVariant,
    pub m_texture_settings: GlTextureSettings,
    pub m_dimension: i32,
    pub m_volume_depth: i32,
    pub m_use_dynamic_scale: Option<bool>,
    pub m_material: PPtr,
    pub m_bind_ms: Option<CustomRenderTextureMBindMsVariant>,
    pub m_use_dynamic_scale_explicit: Option<bool>,
    pub m_init_source: i32,
    pub m_enable_compatible_format: Option<CustomRenderTextureMEnableCompatibleFormatVariant>,
    pub m_init_material: PPtr,
    pub m_init_color: ColorRgbAf,
    pub m_enable_random_write: Option<bool>,
    pub m_init_texture: PPtr,
    pub m_update_mode: i32,
    pub m_initialization_mode: i32,
    pub m_update_zone_space: i32,
    pub m_shadow_sampling_mode: Option<i32>,
    pub m_current_update_zone_space: i32,
    pub m_update_zones: CustomRenderTextureMUpdateZonesVariant,
    pub m_update_period: f32,
    pub m_shader_pass: u32,
    pub m_cubemap_face_mask: u32,
    pub m_double_buffered: bool,
    pub m_wrap_update_zones: bool,
}

impl crate::Parse for CustomRenderTexture {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_width = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_height = None;
        let mut m_downscale_fallback = None;
        let mut m_anti_aliasing = None;
        let mut m_depth_format = None;
        let mut m_color_format = None;
        let mut m_mip_count = None;
        let mut m_mip_map = None;
        let mut m_depth_stencil_format = None;
        let mut m_generate_mips = None;
        let mut m_srgb = None;
        let mut m_texture_settings = None;
        let mut m_dimension = None;
        let mut m_volume_depth = None;
        let mut m_use_dynamic_scale = None;
        let mut m_material = None;
        let mut m_bind_ms = None;
        let mut m_use_dynamic_scale_explicit = None;
        let mut m_init_source = None;
        let mut m_enable_compatible_format = None;
        let mut m_init_material = None;
        let mut m_init_color = None;
        let mut m_enable_random_write = None;
        let mut m_init_texture = None;
        let mut m_update_mode = None;
        let mut m_initialization_mode = None;
        let mut m_update_zone_space = None;
        let mut m_shadow_sampling_mode = None;
        let mut m_current_update_zone_space = None;
        let mut m_update_zones = None;
        let mut m_update_period = None;
        let mut m_shader_pass = None;
        let mut m_cubemap_face_mask = None;
        let mut m_double_buffered = None;
        let mut m_wrap_update_zones = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(CustomRenderTextureMDownscaleFallbackVariant::parse(
                rdr, ver,
            )?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(CustomRenderTextureMDownscaleFallbackVariant::parse(
                rdr, ver,
            )?);
        } else {
        }
        m_width = Some(rdr.read_i32()?);
        m_height = Some(rdr.read_i32()?);
        m_anti_aliasing = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_mip_count = Some(rdr.read_i32()?);
            m_depth_stencil_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_mip_count = Some(rdr.read_i32()?);
            m_depth_format = Some(rdr.read_i32()?);
        } else {
            m_depth_format = Some(rdr.read_i32()?);
        }
        m_color_format = Some(rdr.read_i32()?);
        m_mip_map = Some(rdr.read_bool()?);
        m_generate_mips = Some(rdr.read_bool()?);
        m_srgb = Some(CustomRenderTextureMSrgbVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_use_dynamic_scale_explicit = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 5) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
            m_enable_random_write = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
            m_enable_compatible_format = Some(
                CustomRenderTextureMEnableCompatibleFormatVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_use_dynamic_scale = Some(rdr.read_bool()?);
            m_bind_ms = Some(CustomRenderTextureMBindMsVariant::parse(rdr, ver)?);
        } else {
        }
        m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        m_dimension = Some(rdr.read_i32()?);
        m_volume_depth = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_shadow_sampling_mode = Some(rdr.read_i32()?);
        } else {
        }
        m_material = Some(crate::PPtr::parse(rdr, ver)?);
        m_init_source = Some(rdr.read_i32()?);
        m_init_material = Some(crate::PPtr::parse(rdr, ver)?);
        m_init_color = Some(ColorRgbAf::parse(rdr, ver)?);
        m_init_texture = Some(crate::PPtr::parse(rdr, ver)?);
        m_update_mode = Some(rdr.read_i32()?);
        m_initialization_mode = Some(rdr.read_i32()?);
        m_update_zone_space = Some(rdr.read_i32()?);
        m_current_update_zone_space = Some(rdr.read_i32()?);
        m_update_zones = Some(CustomRenderTextureMUpdateZonesVariant::parse(rdr, ver)?);
        m_update_period = Some(rdr.read_f32()?);
        rdr.align(4)?;
        m_shader_pass = Some(rdr.read_u32()?);
        m_cubemap_face_mask = Some(rdr.read_u32()?);
        m_double_buffered = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_wrap_update_zones = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(CustomRenderTexture {
            m_name: m_name.unwrap(),
            m_width: m_width.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_height: m_height.unwrap(),
            m_downscale_fallback,
            m_anti_aliasing: m_anti_aliasing.unwrap(),
            m_depth_format,
            m_color_format: m_color_format.unwrap(),
            m_mip_count,
            m_mip_map: m_mip_map.unwrap(),
            m_depth_stencil_format,
            m_generate_mips: m_generate_mips.unwrap(),
            m_srgb: m_srgb.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
            m_dimension: m_dimension.unwrap(),
            m_volume_depth: m_volume_depth.unwrap(),
            m_use_dynamic_scale,
            m_material: m_material.unwrap(),
            m_bind_ms,
            m_use_dynamic_scale_explicit,
            m_init_source: m_init_source.unwrap(),
            m_enable_compatible_format,
            m_init_material: m_init_material.unwrap(),
            m_init_color: m_init_color.unwrap(),
            m_enable_random_write,
            m_init_texture: m_init_texture.unwrap(),
            m_update_mode: m_update_mode.unwrap(),
            m_initialization_mode: m_initialization_mode.unwrap(),
            m_update_zone_space: m_update_zone_space.unwrap(),
            m_shadow_sampling_mode,
            m_current_update_zone_space: m_current_update_zone_space.unwrap(),
            m_update_zones: m_update_zones.unwrap(),
            m_update_period: m_update_period.unwrap(),
            m_shader_pass: m_shader_pass.unwrap(),
            m_cubemap_face_mask: m_cubemap_face_mask.unwrap(),
            m_double_buffered: m_double_buffered.unwrap(),
            m_wrap_update_zones: m_wrap_update_zones.unwrap(),
        })
    }
}

impl crate::ToValue for CustomRenderTexture {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Width".to_string(), self.m_width.to_value())),
            crate::opt_field("m_ForcedFallbackFormat", &self.m_forced_fallback_format),
            crate::opt_field(
                "m_IsAlphaChannelOptional",
                &self.m_is_alpha_channel_optional,
            ),
            Some(("m_Height".to_string(), self.m_height.to_value())),
            crate::opt_field("m_DownscaleFallback", &self.m_downscale_fallback),
            Some((
                "m_AntiAliasing".to_string(),
                self.m_anti_aliasing.to_value(),
            )),
            crate::opt_field("m_DepthFormat", &self.m_depth_format),
            Some(("m_ColorFormat".to_string(), self.m_color_format.to_value())),
            crate::opt_field("m_MipCount", &self.m_mip_count),
            Some(("m_MipMap".to_string(), self.m_mip_map.to_value())),
            crate::opt_field("m_DepthStencilFormat", &self.m_depth_stencil_format),
            Some((
                "m_GenerateMips".to_string(),
                self.m_generate_mips.to_value(),
            )),
            Some(("m_SRGB".to_string(), self.m_srgb.to_value())),
            Some((
                "m_TextureSettings".to_string(),
                self.m_texture_settings.to_value(),
            )),
            Some(("m_Dimension".to_string(), self.m_dimension.to_value())),
            Some(("m_VolumeDepth".to_string(), self.m_volume_depth.to_value())),
            crate::opt_field("m_UseDynamicScale", &self.m_use_dynamic_scale),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            crate::opt_field("m_BindMS", &self.m_bind_ms),
            crate::opt_field(
                "m_UseDynamicScaleExplicit",
                &self.m_use_dynamic_scale_explicit,
            ),
            Some(("m_InitSource".to_string(), self.m_init_source.to_value())),
            crate::opt_field("m_EnableCompatibleFormat", &self.m_enable_compatible_format),
            Some((
                "m_InitMaterial".to_string(),
                self.m_init_material.to_value(),
            )),
            Some(("m_InitColor".to_string(), self.m_init_color.to_value())),
            crate::opt_field("m_EnableRandomWrite", &self.m_enable_random_write),
            Some(("m_InitTexture".to_string(), self.m_init_texture.to_value())),
            Some(("m_UpdateMode".to_string(), self.m_update_mode.to_value())),
            Some((
                "m_InitializationMode".to_string(),
                self.m_initialization_mode.to_value(),
            )),
            Some((
                "m_UpdateZoneSpace".to_string(),
                self.m_update_zone_space.to_value(),
            )),
            crate::opt_field("m_ShadowSamplingMode", &self.m_shadow_sampling_mode),
            Some((
                "m_CurrentUpdateZoneSpace".to_string(),
                self.m_current_update_zone_space.to_value(),
            )),
            Some(("m_UpdateZones".to_string(), self.m_update_zones.to_value())),
            Some((
                "m_UpdatePeriod".to_string(),
                self.m_update_period.to_value(),
            )),
            Some(("m_ShaderPass".to_string(), self.m_shader_pass.to_value())),
            Some((
                "m_CubemapFaceMask".to_string(),
                self.m_cubemap_face_mask.to_value(),
            )),
            Some((
                "m_DoubleBuffered".to_string(),
                self.m_double_buffered.to_value(),
            )),
            Some((
                "m_WrapUpdateZones".to_string(),
                self.m_wrap_update_zones.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Width" => (self.m_width).inspect_value(path, offset, limit),
                "m_ForcedFallbackFormat" => {
                    (self.m_forced_fallback_format).inspect_value(path, offset, limit)
                }
                "m_IsAlphaChannelOptional" => {
                    (self.m_is_alpha_channel_optional).inspect_value(path, offset, limit)
                }
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_DownscaleFallback" => {
                    (self.m_downscale_fallback).inspect_value(path, offset, limit)
                }
                "m_AntiAliasing" => (self.m_anti_aliasing).inspect_value(path, offset, limit),
                "m_DepthFormat" => (self.m_depth_format).inspect_value(path, offset, limit),
                "m_ColorFormat" => (self.m_color_format).inspect_value(path, offset, limit),
                "m_MipCount" => (self.m_mip_count).inspect_value(path, offset, limit),
                "m_MipMap" => (self.m_mip_map).inspect_value(path, offset, limit),
                "m_DepthStencilFormat" => {
                    (self.m_depth_stencil_format).inspect_value(path, offset, limit)
                }
                "m_GenerateMips" => (self.m_generate_mips).inspect_value(path, offset, limit),
                "m_SRGB" => (self.m_srgb).inspect_value(path, offset, limit),
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "m_Dimension" => (self.m_dimension).inspect_value(path, offset, limit),
                "m_VolumeDepth" => (self.m_volume_depth).inspect_value(path, offset, limit),
                "m_UseDynamicScale" => {
                    (self.m_use_dynamic_scale).inspect_value(path, offset, limit)
                }
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_BindMS" => (self.m_bind_ms).inspect_value(path, offset, limit),
                "m_UseDynamicScaleExplicit" => {
                    (self.m_use_dynamic_scale_explicit).inspect_value(path, offset, limit)
                }
                "m_InitSource" => (self.m_init_source).inspect_value(path, offset, limit),
                "m_EnableCompatibleFormat" => {
                    (self.m_enable_compatible_format).inspect_value(path, offset, limit)
                }
                "m_InitMaterial" => (self.m_init_material).inspect_value(path, offset, limit),
                "m_InitColor" => (self.m_init_color).inspect_value(path, offset, limit),
                "m_EnableRandomWrite" => {
                    (self.m_enable_random_write).inspect_value(path, offset, limit)
                }
                "m_InitTexture" => (self.m_init_texture).inspect_value(path, offset, limit),
                "m_UpdateMode" => (self.m_update_mode).inspect_value(path, offset, limit),
                "m_InitializationMode" => {
                    (self.m_initialization_mode).inspect_value(path, offset, limit)
                }
                "m_UpdateZoneSpace" => {
                    (self.m_update_zone_space).inspect_value(path, offset, limit)
                }
                "m_ShadowSamplingMode" => {
                    (self.m_shadow_sampling_mode).inspect_value(path, offset, limit)
                }
                "m_CurrentUpdateZoneSpace" => {
                    (self.m_current_update_zone_space).inspect_value(path, offset, limit)
                }
                "m_UpdateZones" => (self.m_update_zones).inspect_value(path, offset, limit),
                "m_UpdatePeriod" => (self.m_update_period).inspect_value(path, offset, limit),
                "m_ShaderPass" => (self.m_shader_pass).inspect_value(path, offset, limit),
                "m_CubemapFaceMask" => {
                    (self.m_cubemap_face_mask).inspect_value(path, offset, limit)
                }
                "m_DoubleBuffered" => (self.m_double_buffered).inspect_value(path, offset, limit),
                "m_WrapUpdateZones" => {
                    (self.m_wrap_update_zones).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_width).inspect_value(path, offset, limit)).map(|v| ("m_Width".to_string(), v)),
            ((self.m_forced_fallback_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ForcedFallbackFormat".to_string(), v)),
            ((self.m_is_alpha_channel_optional).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAlphaChannelOptional".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
            ((self.m_anti_aliasing).inspect_value(path, offset, limit))
                .map(|v| ("m_AntiAliasing".to_string(), v)),
            ((self.m_depth_format).inspect_value(path, offset, limit))
                .map(|v| ("m_DepthFormat".to_string(), v)),
            ((self.m_color_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorFormat".to_string(), v)),
            ((self.m_mip_count).inspect_value(path, offset, limit))
                .map(|v| ("m_MipCount".to_string(), v)),
            ((self.m_mip_map).inspect_value(path, offset, limit))
                .map(|v| ("m_MipMap".to_string(), v)),
            ((self.m_depth_stencil_format).inspect_value(path, offset, limit))
                .map(|v| ("m_DepthStencilFormat".to_string(), v)),
            ((self.m_generate_mips).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateMips".to_string(), v)),
            ((self.m_srgb).inspect_value(path, offset, limit)).map(|v| ("m_SRGB".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.m_dimension).inspect_value(path, offset, limit))
                .map(|v| ("m_Dimension".to_string(), v)),
            ((self.m_volume_depth).inspect_value(path, offset, limit))
                .map(|v| ("m_VolumeDepth".to_string(), v)),
            ((self.m_use_dynamic_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDynamicScale".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_bind_ms).inspect_value(path, offset, limit))
                .map(|v| ("m_BindMS".to_string(), v)),
            ((self.m_use_dynamic_scale_explicit).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDynamicScaleExplicit".to_string(), v)),
            ((self.m_init_source).inspect_value(path, offset, limit))
                .map(|v| ("m_InitSource".to_string(), v)),
            ((self.m_enable_compatible_format).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCompatibleFormat".to_string(), v)),
            ((self.m_init_material).inspect_value(path, offset, limit))
                .map(|v| ("m_InitMaterial".to_string(), v)),
            ((self.m_init_color).inspect_value(path, offset, limit))
                .map(|v| ("m_InitColor".to_string(), v)),
            ((self.m_enable_random_write).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableRandomWrite".to_string(), v)),
            ((self.m_init_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_InitTexture".to_string(), v)),
            ((self.m_update_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateMode".to_string(), v)),
            ((self.m_initialization_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_InitializationMode".to_string(), v)),
            ((self.m_update_zone_space).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateZoneSpace".to_string(), v)),
            ((self.m_shadow_sampling_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ShadowSamplingMode".to_string(), v)),
            ((self.m_current_update_zone_space).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentUpdateZoneSpace".to_string(), v)),
            ((self.m_update_zones).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateZones".to_string(), v)),
            ((self.m_update_period).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdatePeriod".to_string(), v)),
            ((self.m_shader_pass).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderPass".to_string(), v)),
            ((self.m_cubemap_face_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_CubemapFaceMask".to_string(), v)),
            ((self.m_double_buffered).inspect_value(path, offset, limit))
                .map(|v| ("m_DoubleBuffered".to_string(), v)),
            ((self.m_wrap_update_zones).inspect_value(path, offset, limit))
                .map(|v| ("m_WrapUpdateZones".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
