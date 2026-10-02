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

impl crate::ToValue for LightmapParameters {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("resolution".to_string(), self.resolution.to_value())),
            Some((
                "clusterResolution".to_string(),
                self.cluster_resolution.to_value(),
            )),
            Some((
                "irradianceBudget".to_string(),
                self.irradiance_budget.to_value(),
            )),
            Some((
                "irradianceQuality".to_string(),
                self.irradiance_quality.to_value(),
            )),
            Some((
                "backFaceTolerance".to_string(),
                self.back_face_tolerance.to_value(),
            )),
            Some(("isTransparent".to_string(), self.is_transparent.to_value())),
            Some((
                "modellingTolerance".to_string(),
                self.modelling_tolerance.to_value(),
            )),
            Some(("systemTag".to_string(), self.system_tag.to_value())),
            Some(("edgeStitching".to_string(), self.edge_stitching.to_value())),
            Some(("blurRadius".to_string(), self.blur_radius.to_value())),
            Some((
                "directLightQuality".to_string(),
                self.direct_light_quality.to_value(),
            )),
            Some((
                "antiAliasingSamples".to_string(),
                self.anti_aliasing_samples.to_value(),
            )),
            Some(("AOQuality".to_string(), self.ao_quality.to_value())),
            Some((
                "bakedLightmapTag".to_string(),
                self.baked_lightmap_tag.to_value(),
            )),
            crate::opt_field("pushoff", &self.pushoff),
            Some((
                "AOAntiAliasingSamples".to_string(),
                self.ao_anti_aliasing_samples.to_value(),
            )),
            crate::opt_field("limitLightmapCount", &self.limit_lightmap_count),
            crate::opt_field("maxLightmapCount", &self.max_lightmap_count),
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
                "resolution" => (self.resolution).inspect_value(path, offset, limit),
                "clusterResolution" => (self.cluster_resolution).inspect_value(path, offset, limit),
                "irradianceBudget" => (self.irradiance_budget).inspect_value(path, offset, limit),
                "irradianceQuality" => (self.irradiance_quality).inspect_value(path, offset, limit),
                "backFaceTolerance" => {
                    (self.back_face_tolerance).inspect_value(path, offset, limit)
                }
                "isTransparent" => (self.is_transparent).inspect_value(path, offset, limit),
                "modellingTolerance" => {
                    (self.modelling_tolerance).inspect_value(path, offset, limit)
                }
                "systemTag" => (self.system_tag).inspect_value(path, offset, limit),
                "edgeStitching" => (self.edge_stitching).inspect_value(path, offset, limit),
                "blurRadius" => (self.blur_radius).inspect_value(path, offset, limit),
                "directLightQuality" => {
                    (self.direct_light_quality).inspect_value(path, offset, limit)
                }
                "antiAliasingSamples" => {
                    (self.anti_aliasing_samples).inspect_value(path, offset, limit)
                }
                "AOQuality" => (self.ao_quality).inspect_value(path, offset, limit),
                "bakedLightmapTag" => (self.baked_lightmap_tag).inspect_value(path, offset, limit),
                "pushoff" => (self.pushoff).inspect_value(path, offset, limit),
                "AOAntiAliasingSamples" => {
                    (self.ao_anti_aliasing_samples).inspect_value(path, offset, limit)
                }
                "limitLightmapCount" => {
                    (self.limit_lightmap_count).inspect_value(path, offset, limit)
                }
                "maxLightmapCount" => (self.max_lightmap_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.resolution).inspect_value(path, offset, limit))
                .map(|v| ("resolution".to_string(), v)),
            ((self.cluster_resolution).inspect_value(path, offset, limit))
                .map(|v| ("clusterResolution".to_string(), v)),
            ((self.irradiance_budget).inspect_value(path, offset, limit))
                .map(|v| ("irradianceBudget".to_string(), v)),
            ((self.irradiance_quality).inspect_value(path, offset, limit))
                .map(|v| ("irradianceQuality".to_string(), v)),
            ((self.back_face_tolerance).inspect_value(path, offset, limit))
                .map(|v| ("backFaceTolerance".to_string(), v)),
            ((self.is_transparent).inspect_value(path, offset, limit))
                .map(|v| ("isTransparent".to_string(), v)),
            ((self.modelling_tolerance).inspect_value(path, offset, limit))
                .map(|v| ("modellingTolerance".to_string(), v)),
            ((self.system_tag).inspect_value(path, offset, limit))
                .map(|v| ("systemTag".to_string(), v)),
            ((self.edge_stitching).inspect_value(path, offset, limit))
                .map(|v| ("edgeStitching".to_string(), v)),
            ((self.blur_radius).inspect_value(path, offset, limit))
                .map(|v| ("blurRadius".to_string(), v)),
            ((self.direct_light_quality).inspect_value(path, offset, limit))
                .map(|v| ("directLightQuality".to_string(), v)),
            ((self.anti_aliasing_samples).inspect_value(path, offset, limit))
                .map(|v| ("antiAliasingSamples".to_string(), v)),
            ((self.ao_quality).inspect_value(path, offset, limit))
                .map(|v| ("AOQuality".to_string(), v)),
            ((self.baked_lightmap_tag).inspect_value(path, offset, limit))
                .map(|v| ("bakedLightmapTag".to_string(), v)),
            ((self.pushoff).inspect_value(path, offset, limit)).map(|v| ("pushoff".to_string(), v)),
            ((self.ao_anti_aliasing_samples).inspect_value(path, offset, limit))
                .map(|v| ("AOAntiAliasingSamples".to_string(), v)),
            ((self.limit_lightmap_count).inspect_value(path, offset, limit))
                .map(|v| ("limitLightmapCount".to_string(), v)),
            ((self.max_lightmap_count).inspect_value(path, offset, limit))
                .map(|v| ("maxLightmapCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
