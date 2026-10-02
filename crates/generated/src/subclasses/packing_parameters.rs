//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackingParameters {
    pub padding_power: u32,
    pub block_offset: u32,
    pub allow_alpha_splitting: i32,
    pub enable_rotation: i32,
    pub enable_tight_packing: i32,
}

impl crate::Parse for PackingParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let padding_power = rdr.read_u32()?;
        let block_offset = rdr.read_u32()?;
        let allow_alpha_splitting = rdr.read_i32()?;
        let enable_rotation = rdr.read_i32()?;
        let enable_tight_packing = rdr.read_i32()?;

        Ok(PackingParameters {
            padding_power,
            block_offset,
            allow_alpha_splitting,
            enable_rotation,
            enable_tight_packing,
        })
    }
}

impl crate::ToValue for PackingParameters {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("paddingPower".to_string(), self.padding_power.to_value())),
            Some(("blockOffset".to_string(), self.block_offset.to_value())),
            Some((
                "allowAlphaSplitting".to_string(),
                self.allow_alpha_splitting.to_value(),
            )),
            Some((
                "enableRotation".to_string(),
                self.enable_rotation.to_value(),
            )),
            Some((
                "enableTightPacking".to_string(),
                self.enable_tight_packing.to_value(),
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
                "paddingPower" => (self.padding_power).inspect_value(path, offset, limit),
                "blockOffset" => (self.block_offset).inspect_value(path, offset, limit),
                "allowAlphaSplitting" => {
                    (self.allow_alpha_splitting).inspect_value(path, offset, limit)
                }
                "enableRotation" => (self.enable_rotation).inspect_value(path, offset, limit),
                "enableTightPacking" => {
                    (self.enable_tight_packing).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.padding_power).inspect_value(path, offset, limit))
                .map(|v| ("paddingPower".to_string(), v)),
            ((self.block_offset).inspect_value(path, offset, limit))
                .map(|v| ("blockOffset".to_string(), v)),
            ((self.allow_alpha_splitting).inspect_value(path, offset, limit))
                .map(|v| ("allowAlphaSplitting".to_string(), v)),
            ((self.enable_rotation).inspect_value(path, offset, limit))
                .map(|v| ("enableRotation".to_string(), v)),
            ((self.enable_tight_packing).inspect_value(path, offset, limit))
                .map(|v| ("enableTightPacking".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
