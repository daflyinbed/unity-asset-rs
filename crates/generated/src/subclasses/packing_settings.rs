//! 自动生成。请勿手动修改。

/// 版本枚举：PackingSettingsEnableTightPackingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PackingSettingsEnableTightPackingVariant {
    Default(bool),
    V2021_1(bool),
}

impl crate::Parse for PackingSettingsEnableTightPackingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PackingSettingsEnableTightPackingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PackingSettings {
    pub padding: i32,
    pub block_offset: i32,
    pub allow_alpha_splitting: bool,
    pub enable_rotation: bool,
    pub enable_tight_packing: PackingSettingsEnableTightPackingVariant,
    pub enable_alpha_dilation: Option<bool>,
}

impl crate::Parse for PackingSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let padding = rdr.read_i32()?;
        let block_offset = rdr.read_i32()?;
        let allow_alpha_splitting = rdr.read_bool()?;
        let enable_rotation = rdr.read_bool()?;
        let enable_tight_packing = PackingSettingsEnableTightPackingVariant::parse(rdr, ver)?;
        let enable_alpha_dilation = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PackingSettings {
            padding,
            block_offset,
            allow_alpha_splitting,
            enable_rotation,
            enable_tight_packing,
            enable_alpha_dilation,
        })
    }
}

impl crate::ToValue for PackingSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("padding".to_string(), self.padding.to_value())),
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
            crate::opt_field("enableAlphaDilation", &self.enable_alpha_dilation),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "padding" => (self.padding).inspect_value(path, offset, limit),
                "blockOffset" => (self.block_offset).inspect_value(path, offset, limit),
                "allowAlphaSplitting" => {
                    (self.allow_alpha_splitting).inspect_value(path, offset, limit)
                }
                "enableRotation" => (self.enable_rotation).inspect_value(path, offset, limit),
                "enableTightPacking" => {
                    (self.enable_tight_packing).inspect_value(path, offset, limit)
                }
                "enableAlphaDilation" => {
                    (self.enable_alpha_dilation).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.padding).inspect_value(path, offset, limit)).map(|v| ("padding".to_string(), v)),
            ((self.block_offset).inspect_value(path, offset, limit))
                .map(|v| ("blockOffset".to_string(), v)),
            ((self.allow_alpha_splitting).inspect_value(path, offset, limit))
                .map(|v| ("allowAlphaSplitting".to_string(), v)),
            ((self.enable_rotation).inspect_value(path, offset, limit))
                .map(|v| ("enableRotation".to_string(), v)),
            ((self.enable_tight_packing).inspect_value(path, offset, limit))
                .map(|v| ("enableTightPacking".to_string(), v)),
            ((self.enable_alpha_dilation).inspect_value(path, offset, limit))
                .map(|v| ("enableAlphaDilation".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
