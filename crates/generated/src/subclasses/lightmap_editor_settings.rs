//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct LightmapEditorSettings {
    pub m_resolution: f32,
    pub m_last_used_resolution: f32,
    pub m_texture_width: i32,
    pub m_texture_height: i32,
    pub m_bounce_boost: f32,
    pub m_bounce_intensity: f32,
    pub m_sky_light_color: ColorRgba,
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
        let m_sky_light_color = ColorRgba::parse(rdr, ver)?;
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
