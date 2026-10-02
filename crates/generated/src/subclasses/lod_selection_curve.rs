//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LodSelectionCurve {
    pub m_lod_slope: f32,
    pub m_lod_bias: f32,
}

impl crate::Parse for LodSelectionCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lod_slope = rdr.read_f32()?;
        let m_lod_bias = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(LodSelectionCurve {
            m_lod_slope,
            m_lod_bias,
        })
    }
}

impl crate::ToValue for LodSelectionCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_LodSlope".to_string(), self.m_lod_slope.to_value())),
            Some(("m_LodBias".to_string(), self.m_lod_bias.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_LodSlope" => (self.m_lod_slope).inspect_value(path, offset, limit),
                "m_LodBias" => (self.m_lod_bias).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_lod_slope).inspect_value(path, offset, limit))
                .map(|v| ("m_LodSlope".to_string(), v)),
            ((self.m_lod_bias).inspect_value(path, offset, limit))
                .map(|v| ("m_LodBias".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
