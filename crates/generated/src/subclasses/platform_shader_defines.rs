//! 自动生成。请勿手动修改。

use crate::subclasses::fixed_bitset::fixed_bitset;

#[derive(Debug, Clone)]
pub struct PlatformShaderDefines {
    pub shader_platform: i32,
    pub defines_tier1: fixed_bitset,
    pub defines_tier2: fixed_bitset,
    pub defines_tier3: fixed_bitset,
}

impl crate::Parse for PlatformShaderDefines {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_platform = rdr.read_i32()?;
        let defines_tier1 = fixed_bitset::parse(rdr, ver)?;
        let defines_tier2 = fixed_bitset::parse(rdr, ver)?;
        let defines_tier3 = fixed_bitset::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(PlatformShaderDefines {
            shader_platform,
            defines_tier1,
            defines_tier2,
            defines_tier3,
        })
    }
}

impl crate::ToValue for PlatformShaderDefines {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "shaderPlatform".to_string(),
                self.shader_platform.to_value(),
            )),
            Some(("defines_Tier1".to_string(), self.defines_tier1.to_value())),
            Some(("defines_Tier2".to_string(), self.defines_tier2.to_value())),
            Some(("defines_Tier3".to_string(), self.defines_tier3.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "shaderPlatform" => (self.shader_platform).inspect_value(path, offset, limit),
                "defines_Tier1" => (self.defines_tier1).inspect_value(path, offset, limit),
                "defines_Tier2" => (self.defines_tier2).inspect_value(path, offset, limit),
                "defines_Tier3" => (self.defines_tier3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.shader_platform).inspect_value(path, offset, limit))
                .map(|v| ("shaderPlatform".to_string(), v)),
            ((self.defines_tier1).inspect_value(path, offset, limit))
                .map(|v| ("defines_Tier1".to_string(), v)),
            ((self.defines_tier2).inspect_value(path, offset, limit))
                .map(|v| ("defines_Tier2".to_string(), v)),
            ((self.defines_tier3).inspect_value(path, offset, limit))
                .map(|v| ("defines_Tier3".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
