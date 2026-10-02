//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct LightmapEditorSettings {
    pub m_resolution: f32,
    pub m_last_used_resolution: f32,
    pub m_texture_width: i32,
    pub m_texture_height: i32,
    pub m_bounce_boost: f32,
    pub m_bounce_intensity: f32,
    pub m_sky_light_color: ColorRgbAf,
    pub m_sky_light_intensity: f32,
    pub m_quality: i32,
    pub m_bounces: i32,
    pub m_final_gather_rays: i32,
    pub m_final_gather_contrast_threshold: f32,
    pub m_final_gather_gradient_threshold: f32,
    pub m_final_gather_interpolation_points: i32,
    pub m_ao_amount: f32,
    pub m_ao_max_distance: f32,
    pub m_ao_contrast: f32,
    pub m_texture_compression: bool,
    pub m_lock_atlas: bool,
}

impl crate::Parse for LightmapEditorSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_resolution = rdr.read_f32()?;
        let m_last_used_resolution = rdr.read_f32()?;
        let m_texture_width = rdr.read_i32()?;
        let m_texture_height = rdr.read_i32()?;
        let m_bounce_boost = rdr.read_f32()?;
        let m_bounce_intensity = rdr.read_f32()?;
        let m_sky_light_color = ColorRgbAf::parse(rdr, ver)?;
        let m_sky_light_intensity = rdr.read_f32()?;
        let m_quality = rdr.read_i32()?;
        let m_bounces = rdr.read_i32()?;
        let m_final_gather_rays = rdr.read_i32()?;
        let m_final_gather_contrast_threshold = rdr.read_f32()?;
        let m_final_gather_gradient_threshold = rdr.read_f32()?;
        let m_final_gather_interpolation_points = rdr.read_i32()?;
        let m_ao_amount = rdr.read_f32()?;
        let m_ao_max_distance = rdr.read_f32()?;
        let m_ao_contrast = rdr.read_f32()?;
        let m_texture_compression = rdr.read_bool()?;
        let m_lock_atlas = rdr.read_bool()?;

        Ok(LightmapEditorSettings {
            m_resolution,
            m_last_used_resolution,
            m_texture_width,
            m_texture_height,
            m_bounce_boost,
            m_bounce_intensity,
            m_sky_light_color,
            m_sky_light_intensity,
            m_quality,
            m_bounces,
            m_final_gather_rays,
            m_final_gather_contrast_threshold,
            m_final_gather_gradient_threshold,
            m_final_gather_interpolation_points,
            m_ao_amount,
            m_ao_max_distance,
            m_ao_contrast,
            m_texture_compression,
            m_lock_atlas,
        })
    }
}

impl crate::ToValue for LightmapEditorSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Resolution".to_string(), self.m_resolution.to_value())),
            Some((
                "m_LastUsedResolution".to_string(),
                self.m_last_used_resolution.to_value(),
            )),
            Some((
                "m_TextureWidth".to_string(),
                self.m_texture_width.to_value(),
            )),
            Some((
                "m_TextureHeight".to_string(),
                self.m_texture_height.to_value(),
            )),
            Some(("m_BounceBoost".to_string(), self.m_bounce_boost.to_value())),
            Some((
                "m_BounceIntensity".to_string(),
                self.m_bounce_intensity.to_value(),
            )),
            Some((
                "m_SkyLightColor".to_string(),
                self.m_sky_light_color.to_value(),
            )),
            Some((
                "m_SkyLightIntensity".to_string(),
                self.m_sky_light_intensity.to_value(),
            )),
            Some(("m_Quality".to_string(), self.m_quality.to_value())),
            Some(("m_Bounces".to_string(), self.m_bounces.to_value())),
            Some((
                "m_FinalGatherRays".to_string(),
                self.m_final_gather_rays.to_value(),
            )),
            Some((
                "m_FinalGatherContrastThreshold".to_string(),
                self.m_final_gather_contrast_threshold.to_value(),
            )),
            Some((
                "m_FinalGatherGradientThreshold".to_string(),
                self.m_final_gather_gradient_threshold.to_value(),
            )),
            Some((
                "m_FinalGatherInterpolationPoints".to_string(),
                self.m_final_gather_interpolation_points.to_value(),
            )),
            Some(("m_AOAmount".to_string(), self.m_ao_amount.to_value())),
            Some((
                "m_AOMaxDistance".to_string(),
                self.m_ao_max_distance.to_value(),
            )),
            Some(("m_AOContrast".to_string(), self.m_ao_contrast.to_value())),
            Some((
                "m_TextureCompression".to_string(),
                self.m_texture_compression.to_value(),
            )),
            Some(("m_LockAtlas".to_string(), self.m_lock_atlas.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Resolution" => (self.m_resolution).inspect_value(path, offset, limit),
                "m_LastUsedResolution" => {
                    (self.m_last_used_resolution).inspect_value(path, offset, limit)
                }
                "m_TextureWidth" => (self.m_texture_width).inspect_value(path, offset, limit),
                "m_TextureHeight" => (self.m_texture_height).inspect_value(path, offset, limit),
                "m_BounceBoost" => (self.m_bounce_boost).inspect_value(path, offset, limit),
                "m_BounceIntensity" => (self.m_bounce_intensity).inspect_value(path, offset, limit),
                "m_SkyLightColor" => (self.m_sky_light_color).inspect_value(path, offset, limit),
                "m_SkyLightIntensity" => {
                    (self.m_sky_light_intensity).inspect_value(path, offset, limit)
                }
                "m_Quality" => (self.m_quality).inspect_value(path, offset, limit),
                "m_Bounces" => (self.m_bounces).inspect_value(path, offset, limit),
                "m_FinalGatherRays" => {
                    (self.m_final_gather_rays).inspect_value(path, offset, limit)
                }
                "m_FinalGatherContrastThreshold" => {
                    (self.m_final_gather_contrast_threshold).inspect_value(path, offset, limit)
                }
                "m_FinalGatherGradientThreshold" => {
                    (self.m_final_gather_gradient_threshold).inspect_value(path, offset, limit)
                }
                "m_FinalGatherInterpolationPoints" => {
                    (self.m_final_gather_interpolation_points).inspect_value(path, offset, limit)
                }
                "m_AOAmount" => (self.m_ao_amount).inspect_value(path, offset, limit),
                "m_AOMaxDistance" => (self.m_ao_max_distance).inspect_value(path, offset, limit),
                "m_AOContrast" => (self.m_ao_contrast).inspect_value(path, offset, limit),
                "m_TextureCompression" => {
                    (self.m_texture_compression).inspect_value(path, offset, limit)
                }
                "m_LockAtlas" => (self.m_lock_atlas).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_Resolution".to_string(), v)),
            ((self.m_last_used_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_LastUsedResolution".to_string(), v)),
            ((self.m_texture_width).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureWidth".to_string(), v)),
            ((self.m_texture_height).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureHeight".to_string(), v)),
            ((self.m_bounce_boost).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceBoost".to_string(), v)),
            ((self.m_bounce_intensity).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceIntensity".to_string(), v)),
            ((self.m_sky_light_color).inspect_value(path, offset, limit))
                .map(|v| ("m_SkyLightColor".to_string(), v)),
            ((self.m_sky_light_intensity).inspect_value(path, offset, limit))
                .map(|v| ("m_SkyLightIntensity".to_string(), v)),
            ((self.m_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_Quality".to_string(), v)),
            ((self.m_bounces).inspect_value(path, offset, limit))
                .map(|v| ("m_Bounces".to_string(), v)),
            ((self.m_final_gather_rays).inspect_value(path, offset, limit))
                .map(|v| ("m_FinalGatherRays".to_string(), v)),
            ((self.m_final_gather_contrast_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_FinalGatherContrastThreshold".to_string(), v)),
            ((self.m_final_gather_gradient_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_FinalGatherGradientThreshold".to_string(), v)),
            ((self.m_final_gather_interpolation_points).inspect_value(path, offset, limit))
                .map(|v| ("m_FinalGatherInterpolationPoints".to_string(), v)),
            ((self.m_ao_amount).inspect_value(path, offset, limit))
                .map(|v| ("m_AOAmount".to_string(), v)),
            ((self.m_ao_max_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_AOMaxDistance".to_string(), v)),
            ((self.m_ao_contrast).inspect_value(path, offset, limit))
                .map(|v| ("m_AOContrast".to_string(), v)),
            ((self.m_texture_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureCompression".to_string(), v)),
            ((self.m_lock_atlas).inspect_value(path, offset, limit))
                .map(|v| ("m_LockAtlas".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
