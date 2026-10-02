//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct SamplerParameter {
    pub sampler: u32,
    pub bind_point: Option<i32>,
    pub m_binding: Option<Binding>,
}

impl crate::Parse for SamplerParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut sampler = None;
        let mut bind_point = None;
        let mut m_binding = None;
        sampler = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
        } else {
            bind_point = Some(rdr.read_i32()?);
        }

        Ok(SamplerParameter {
            sampler: sampler.unwrap(),
            bind_point,
            m_binding,
        })
    }
}

impl crate::ToValue for SamplerParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("sampler".to_string(), self.sampler.to_value())),
            crate::opt_field("bindPoint", &self.bind_point),
            crate::opt_field("m_Binding", &self.m_binding),
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
                "m_Binding" => (self.m_binding).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.sampler).inspect_value(path, offset, limit)).map(|v| ("sampler".to_string(), v)),
            ((self.bind_point).inspect_value(path, offset, limit))
                .map(|v| ("bindPoint".to_string(), v)),
            ((self.m_binding).inspect_value(path, offset, limit))
                .map(|v| ("m_Binding".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
