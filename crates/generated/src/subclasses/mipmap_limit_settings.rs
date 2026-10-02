//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MipmapLimitSettings {
    pub limit_bias_mode: i32,
    pub limit_bias: i32,
}

impl crate::Parse for MipmapLimitSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let limit_bias_mode = rdr.read_i32()?;
        let limit_bias = rdr.read_i32()?;

        Ok(MipmapLimitSettings {
            limit_bias_mode,
            limit_bias,
        })
    }
}

impl crate::ToValue for MipmapLimitSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("limitBiasMode".to_string(), self.limit_bias_mode.to_value())),
            Some(("limitBias".to_string(), self.limit_bias.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "limitBiasMode" => (self.limit_bias_mode).inspect_value(path, offset, limit),
                "limitBias" => (self.limit_bias).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.limit_bias_mode).inspect_value(path, offset, limit))
                .map(|v| ("limitBiasMode".to_string(), v)),
            ((self.limit_bias).inspect_value(path, offset, limit))
                .map(|v| ("limitBias".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
