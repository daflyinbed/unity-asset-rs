//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct UavParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_binding: Option<Binding>,
    pub m_original_index: Option<i32>,
    pub m_original_binding: Option<Binding>,
}

impl crate::Parse for UavParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_binding = None;
        let mut m_original_index = None;
        let mut m_original_binding = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
            m_original_binding = Some(Binding::parse(rdr, ver)?);
        } else {
            m_index = Some(rdr.read_i32()?);
            m_original_index = Some(rdr.read_i32()?);
        }

        Ok(UavParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_binding,
            m_original_index,
            m_original_binding,
        })
    }
}

impl crate::ToValue for UavParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NameIndex".to_string(), self.m_name_index.to_value())),
            crate::opt_field("m_Index", &self.m_index),
            crate::opt_field("m_Binding", &self.m_binding),
            crate::opt_field("m_OriginalIndex", &self.m_original_index),
            crate::opt_field("m_OriginalBinding", &self.m_original_binding),
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
                "m_OriginalIndex" => (self.m_original_index).inspect_value(path, offset, limit),
                "m_OriginalBinding" => (self.m_original_binding).inspect_value(path, offset, limit),
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
            ((self.m_original_index).inspect_value(path, offset, limit))
                .map(|v| ("m_OriginalIndex".to_string(), v)),
            ((self.m_original_binding).inspect_value(path, offset, limit))
                .map(|v| ("m_OriginalBinding".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
