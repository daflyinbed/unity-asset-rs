//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderBuiltinSampler {
    pub sampler: u32,
    pub bind_point: i32,
}

impl crate::Parse for RayTracingShaderBuiltinSampler {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sampler = rdr.read_u32()?;
        let bind_point = rdr.read_i32()?;

        Ok(RayTracingShaderBuiltinSampler {
            sampler,
            bind_point,
        })
    }
}

impl crate::ToValue for RayTracingShaderBuiltinSampler {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("sampler".to_string(), self.sampler.to_value())),
            Some(("bindPoint".to_string(), self.bind_point.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "sampler" => (self.sampler).inspect_value(path, offset, limit),
                "bindPoint" => (self.bind_point).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.sampler).inspect_value(path, offset, limit)).map(|v| ("sampler".to_string(), v)),
            ((self.bind_point).inspect_value(path, offset, limit))
                .map(|v| ("bindPoint".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
