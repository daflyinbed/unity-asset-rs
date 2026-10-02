//! 自动生成。请勿手动修改。

/// 版本枚举：ComputeShaderBuiltinSamplerSamplerVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderBuiltinSamplerSamplerVariant {
    Default(i32),
    V2017_1(u32),
}

impl crate::Parse for ComputeShaderBuiltinSamplerSamplerVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderBuiltinSamplerSamplerVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderBuiltinSampler {
    pub sampler: ComputeShaderBuiltinSamplerSamplerVariant,
    pub bind_point: i32,
}

impl crate::Parse for ComputeShaderBuiltinSampler {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sampler = ComputeShaderBuiltinSamplerSamplerVariant::parse(rdr, ver)?;
        let bind_point = rdr.read_i32()?;

        Ok(ComputeShaderBuiltinSampler {
            sampler,
            bind_point,
        })
    }
}

impl crate::ToValue for ComputeShaderBuiltinSampler {
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
