//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightmapParameters {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub resolution: f32,
    pub cluster_resolution: f32,
    pub irradiance_budget: i32,
    pub irradiance_quality: i32,
    pub back_face_tolerance: f32,
    pub is_transparent: i32,
    pub modelling_tolerance: f32,
    pub system_tag: i32,
    pub edge_stitching: i32,
    pub blur_radius: i32,
    pub direct_light_quality: i32,
    pub anti_aliasing_samples: i32,
    pub ao_quality: i32,
    pub ao_anti_aliasing_samples: i32,
    pub baked_lightmap_tag: i32,
}

impl crate::Parse for LightmapParameters {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let resolution = r.read_f32()?;
        let cluster_resolution = r.read_f32()?;
        let irradiance_budget = r.read_i32()?;
        let irradiance_quality = r.read_i32()?;
        let back_face_tolerance = r.read_f32()?;
        let is_transparent = r.read_i32()?;
        let modelling_tolerance = r.read_f32()?;
        let system_tag = r.read_i32()?;
        let edge_stitching = r.read_i32()?;
        let blur_radius = r.read_i32()?;
        let direct_light_quality = r.read_i32()?;
        let anti_aliasing_samples = r.read_i32()?;
        let ao_quality = r.read_i32()?;
        let ao_anti_aliasing_samples = r.read_i32()?;
        let baked_lightmap_tag = r.read_i32()?;

        Ok(LightmapParameters {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            resolution,
            cluster_resolution,
            irradiance_budget,
            irradiance_quality,
            back_face_tolerance,
            is_transparent,
            modelling_tolerance,
            system_tag,
            edge_stitching,
            blur_radius,
            direct_light_quality,
            anti_aliasing_samples,
            ao_quality,
            ao_anti_aliasing_samples,
            baked_lightmap_tag,
        })
    }
}
