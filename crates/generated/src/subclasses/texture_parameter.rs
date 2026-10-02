//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct TextureParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_binding: Option<Binding>,
    pub m_sampler_index: Option<i32>,
    pub m_sampler_binding: Option<Binding>,
    pub m_dim: i8,
    pub m_multi_sampled: Option<bool>,
}

impl crate::Parse for TextureParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_binding = None;
        let mut m_sampler_index = None;
        let mut m_sampler_binding = None;
        let mut m_dim = None;
        let mut m_multi_sampled = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
            m_sampler_binding = Some(Binding::parse(rdr, ver)?);
            m_multi_sampled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_index = Some(rdr.read_i32()?);
            m_sampler_index = Some(rdr.read_i32()?);
            m_multi_sampled = Some(rdr.read_bool()?);
        } else {
            m_index = Some(rdr.read_i32()?);
            m_sampler_index = Some(rdr.read_i32()?);
        }
        m_dim = Some(rdr.read_i8()?);
        rdr.align(4)?;

        Ok(TextureParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_binding,
            m_sampler_index,
            m_sampler_binding,
            m_dim: m_dim.unwrap(),
            m_multi_sampled,
        })
    }
}

impl crate::ToValue for TextureParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NameIndex".to_string(), self.m_name_index.to_value())),
            crate::opt_field("m_Index", &self.m_index),
            crate::opt_field("m_Binding", &self.m_binding),
            crate::opt_field("m_SamplerIndex", &self.m_sampler_index),
            crate::opt_field("m_SamplerBinding", &self.m_sampler_binding),
            Some(("m_Dim".to_string(), self.m_dim.to_value())),
            crate::opt_field("m_MultiSampled", &self.m_multi_sampled),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NameIndex" => (self.m_name_index).inspect_value(path, offset, limit),
                "m_Index" => (self.m_index).inspect_value(path, offset, limit),
                "m_Binding" => (self.m_binding).inspect_value(path, offset, limit),
                "m_SamplerIndex" => (self.m_sampler_index).inspect_value(path, offset, limit),
                "m_SamplerBinding" => (self.m_sampler_binding).inspect_value(path, offset, limit),
                "m_Dim" => (self.m_dim).inspect_value(path, offset, limit),
                "m_MultiSampled" => (self.m_multi_sampled).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name_index).inspect_value(path, offset, limit))
                .map(|v| ("m_NameIndex".to_string(), v)),
            ((self.m_index).inspect_value(path, offset, limit)).map(|v| ("m_Index".to_string(), v)),
            ((self.m_binding).inspect_value(path, offset, limit))
                .map(|v| ("m_Binding".to_string(), v)),
            ((self.m_sampler_index).inspect_value(path, offset, limit))
                .map(|v| ("m_SamplerIndex".to_string(), v)),
            ((self.m_sampler_binding).inspect_value(path, offset, limit))
                .map(|v| ("m_SamplerBinding".to_string(), v)),
            ((self.m_dim).inspect_value(path, offset, limit)).map(|v| ("m_Dim".to_string(), v)),
            ((self.m_multi_sampled).inspect_value(path, offset, limit))
                .map(|v| ("m_MultiSampled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
