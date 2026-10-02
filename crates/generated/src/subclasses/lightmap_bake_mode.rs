//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightmapBakeMode {
    pub lightmap_bake_type: i32,
    pub mixed_lighting_mode: i32,
}

impl crate::Parse for LightmapBakeMode {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let lightmap_bake_type = rdr.read_i32()?;
        let mixed_lighting_mode = rdr.read_i32()?;

        Ok(LightmapBakeMode {
            lightmap_bake_type,
            mixed_lighting_mode,
        })
    }
}

impl crate::ToValue for LightmapBakeMode {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "lightmapBakeType".to_string(),
                self.lightmap_bake_type.to_value(),
            )),
            Some((
                "mixedLightingMode".to_string(),
                self.mixed_lighting_mode.to_value(),
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
                "lightmapBakeType" => (self.lightmap_bake_type).inspect_value(path, offset, limit),
                "mixedLightingMode" => {
                    (self.mixed_lighting_mode).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.lightmap_bake_type).inspect_value(path, offset, limit))
                .map(|v| ("lightmapBakeType".to_string(), v)),
            ((self.mixed_lighting_mode).inspect_value(path, offset, limit))
                .map(|v| ("mixedLightingMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
