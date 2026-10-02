//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TierGraphicsSettingsEditor {
    pub use_screen_space_shadows: Option<bool>,
    pub use_cascaded_shadow_maps: Option<bool>,
}

impl crate::Parse for TierGraphicsSettingsEditor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut use_screen_space_shadows = None;
        let mut use_cascaded_shadow_maps = None;
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            use_cascaded_shadow_maps = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            use_screen_space_shadows = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(TierGraphicsSettingsEditor {
            use_screen_space_shadows,
            use_cascaded_shadow_maps,
        })
    }
}

impl crate::ToValue for TierGraphicsSettingsEditor {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("useScreenSpaceShadows", &self.use_screen_space_shadows),
            crate::opt_field("useCascadedShadowMaps", &self.use_cascaded_shadow_maps),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "useScreenSpaceShadows" => {
                    (self.use_screen_space_shadows).inspect_value(path, offset, limit)
                }
                "useCascadedShadowMaps" => {
                    (self.use_cascaded_shadow_maps).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.use_screen_space_shadows).inspect_value(path, offset, limit))
                .map(|v| ("useScreenSpaceShadows".to_string(), v)),
            ((self.use_cascaded_shadow_maps).inspect_value(path, offset, limit))
                .map(|v| ("useCascadedShadowMaps".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
