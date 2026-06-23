//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct QualitySetting {
    pub pixel_light_count: i32,
    pub shadows: i32,
    pub shadow_resolution: i32,
    pub shadow_projection: i32,
    pub shadow_cascades: i32,
    pub shadow_distance: f32,
    pub blend_weights: i32,
    pub texture_quality: i32,
    pub anisotropic_textures: i32,
    pub anti_aliasing: i32,
    pub soft_particles: bool,
    pub soft_vegetation: bool,
    pub v_sync_count: i32,
}

impl crate::Parse for QualitySetting {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let pixel_light_count = r.read_i32()?;
        let shadows = r.read_i32()?;
        let shadow_resolution = r.read_i32()?;
        let shadow_projection = r.read_i32()?;
        let shadow_cascades = r.read_i32()?;
        let shadow_distance = r.read_f32()?;
        let blend_weights = r.read_i32()?;
        let texture_quality = r.read_i32()?;
        let anisotropic_textures = r.read_i32()?;
        let anti_aliasing = r.read_i32()?;
        let soft_particles = r.read_bool()?;
        let soft_vegetation = r.read_bool()?;
        r.align(4)?;
        let v_sync_count = r.read_i32()?;
        r.align(4)?;

        Ok(QualitySetting {
            pixel_light_count,
            shadows,
            shadow_resolution,
            shadow_projection,
            shadow_cascades,
            shadow_distance,
            blend_weights,
            texture_quality,
            anisotropic_textures,
            anti_aliasing,
            soft_particles,
            soft_vegetation,
            v_sync_count,
        })
    }
}
