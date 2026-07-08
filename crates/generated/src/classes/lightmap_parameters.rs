//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightmapParameters {
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
    pub baked_lightmap_tag: i32,
    pub pushoff: Option<f32>,
    pub ao_anti_aliasing_samples: i32,
    pub limit_lightmap_count: Option<bool>,
    pub max_lightmap_count: Option<i32>,
}

impl crate::Parse for LightmapParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut resolution = None;
        let mut cluster_resolution = None;
        let mut irradiance_budget = None;
        let mut irradiance_quality = None;
        let mut back_face_tolerance = None;
        let mut is_transparent = None;
        let mut modelling_tolerance = None;
        let mut system_tag = None;
        let mut edge_stitching = None;
        let mut blur_radius = None;
        let mut direct_light_quality = None;
        let mut anti_aliasing_samples = None;
        let mut ao_quality = None;
        let mut baked_lightmap_tag = None;
        let mut pushoff = None;
        let mut ao_anti_aliasing_samples = None;
        let mut limit_lightmap_count = None;
        let mut max_lightmap_count = None;
            m_name = Some(rdr.read_aligned_string()?);
            resolution = Some(rdr.read_f32()?);
            cluster_resolution = Some(rdr.read_f32()?);
            irradiance_budget = Some(rdr.read_i32()?);
            irradiance_quality = Some(rdr.read_i32()?);
            back_face_tolerance = Some(rdr.read_f32()?);
            is_transparent = Some(rdr.read_i32()?);
            modelling_tolerance = Some(rdr.read_f32()?);
            system_tag = Some(rdr.read_i32()?);
            edge_stitching = Some(rdr.read_i32()?);
            blur_radius = Some(rdr.read_i32()?);
            direct_light_quality = Some(rdr.read_i32()?);
            anti_aliasing_samples = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            pushoff = Some(rdr.read_f32()?);
            baked_lightmap_tag = Some(rdr.read_i32()?);
            limit_lightmap_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            max_lightmap_count = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            baked_lightmap_tag = Some(rdr.read_i32()?);
            pushoff = Some(rdr.read_f32()?);
        } else {
        }
            ao_quality = Some(rdr.read_i32()?);
            ao_anti_aliasing_samples = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
        } else {
            baked_lightmap_tag = Some(rdr.read_i32()?);
        }

        Ok(LightmapParameters {
            m_name: m_name.unwrap(),
            resolution: resolution.unwrap(),
            cluster_resolution: cluster_resolution.unwrap(),
            irradiance_budget: irradiance_budget.unwrap(),
            irradiance_quality: irradiance_quality.unwrap(),
            back_face_tolerance: back_face_tolerance.unwrap(),
            is_transparent: is_transparent.unwrap(),
            modelling_tolerance: modelling_tolerance.unwrap(),
            system_tag: system_tag.unwrap(),
            edge_stitching: edge_stitching.unwrap(),
            blur_radius: blur_radius.unwrap(),
            direct_light_quality: direct_light_quality.unwrap(),
            anti_aliasing_samples: anti_aliasing_samples.unwrap(),
            ao_quality: ao_quality.unwrap(),
            baked_lightmap_tag: baked_lightmap_tag.unwrap(),
            pushoff,
            ao_anti_aliasing_samples: ao_anti_aliasing_samples.unwrap(),
            limit_lightmap_count,
            max_lightmap_count,
        })
    }
}
