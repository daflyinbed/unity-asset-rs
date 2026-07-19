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
