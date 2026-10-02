//! 自动生成。请勿手动修改。

/// 版本枚举：TierGraphicsSettingsUseCascadedShadowMapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TierGraphicsSettingsUseCascadedShadowMapsVariant {
    Default(bool),
    V5_6(bool),
}

impl crate::Parse for TierGraphicsSettingsUseCascadedShadowMapsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TierGraphicsSettingsUseCascadedShadowMapsVariant {
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
#[derive(Debug, Clone)]
pub struct TierGraphicsSettings {
    pub rendering_path: i32,
    pub use_cascaded_shadow_maps: TierGraphicsSettingsUseCascadedShadowMapsVariant,
    pub hdr_mode: Option<i32>,
    pub realtime_gicpu_usage: Option<i32>,
    pub use_hdr: Option<bool>,
    pub enable_lppv: Option<bool>,
    pub prefer32_bit_shadow_maps: Option<bool>,
}

impl crate::Parse for TierGraphicsSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut rendering_path = None;
        let mut use_cascaded_shadow_maps = None;
        let mut hdr_mode = None;
        let mut realtime_gicpu_usage = None;
        let mut use_hdr = None;
        let mut enable_lppv = None;
        let mut prefer32_bit_shadow_maps = None;
        rendering_path = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            hdr_mode = Some(rdr.read_i32()?);
            realtime_gicpu_usage = Some(rdr.read_i32()?);
        } else {
        }
        use_cascaded_shadow_maps = Some(TierGraphicsSettingsUseCascadedShadowMapsVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            prefer32_bit_shadow_maps = Some(rdr.read_bool()?);
            enable_lppv = Some(rdr.read_bool()?);
            use_hdr = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 3) {
            enable_lppv = Some(rdr.read_bool()?);
            use_hdr = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            use_hdr = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(TierGraphicsSettings {
            rendering_path: rendering_path.unwrap(),
            use_cascaded_shadow_maps: use_cascaded_shadow_maps.unwrap(),
            hdr_mode,
            realtime_gicpu_usage,
            use_hdr,
            enable_lppv,
            prefer32_bit_shadow_maps,
        })
    }
}

impl crate::ToValue for TierGraphicsSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("renderingPath".to_string(), self.rendering_path.to_value())),
            Some((
                "useCascadedShadowMaps".to_string(),
                self.use_cascaded_shadow_maps.to_value(),
            )),
            crate::opt_field("hdrMode", &self.hdr_mode),
            crate::opt_field("realtimeGICPUUsage", &self.realtime_gicpu_usage),
            crate::opt_field("useHDR", &self.use_hdr),
            crate::opt_field("enableLPPV", &self.enable_lppv),
            crate::opt_field("prefer32BitShadowMaps", &self.prefer32_bit_shadow_maps),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "renderingPath" => (self.rendering_path).inspect_value(path, offset, limit),
                "useCascadedShadowMaps" => {
                    (self.use_cascaded_shadow_maps).inspect_value(path, offset, limit)
                }
                "hdrMode" => (self.hdr_mode).inspect_value(path, offset, limit),
                "realtimeGICPUUsage" => {
                    (self.realtime_gicpu_usage).inspect_value(path, offset, limit)
                }
                "useHDR" => (self.use_hdr).inspect_value(path, offset, limit),
                "enableLPPV" => (self.enable_lppv).inspect_value(path, offset, limit),
                "prefer32BitShadowMaps" => {
                    (self.prefer32_bit_shadow_maps).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.rendering_path).inspect_value(path, offset, limit))
                .map(|v| ("renderingPath".to_string(), v)),
            ((self.use_cascaded_shadow_maps).inspect_value(path, offset, limit))
                .map(|v| ("useCascadedShadowMaps".to_string(), v)),
            ((self.hdr_mode).inspect_value(path, offset, limit))
                .map(|v| ("hdrMode".to_string(), v)),
            ((self.realtime_gicpu_usage).inspect_value(path, offset, limit))
                .map(|v| ("realtimeGICPUUsage".to_string(), v)),
            ((self.use_hdr).inspect_value(path, offset, limit)).map(|v| ("useHDR".to_string(), v)),
            ((self.enable_lppv).inspect_value(path, offset, limit))
                .map(|v| ("enableLPPV".to_string(), v)),
            ((self.prefer32_bit_shadow_maps).inspect_value(path, offset, limit))
                .map(|v| ("prefer32BitShadowMaps".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
