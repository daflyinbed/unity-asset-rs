//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub m_parameter_name: String,
    pub m_guid: Guid,
}

impl crate::Parse for Parameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_parameter_name = rdr.read_aligned_string()?;
        let m_guid = Guid::parse(rdr, ver)?;

        Ok(Parameter {
            m_parameter_name,
            m_guid,
        })
    }
}

impl crate::ToValue for Parameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ParameterName".to_string(),
                self.m_parameter_name.to_value(),
            )),
            Some(("m_GUID".to_string(), self.m_guid.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ParameterName" => (self.m_parameter_name).inspect_value(path, offset, limit),
                "m_GUID" => (self.m_guid).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_parameter_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ParameterName".to_string(), v)),
            ((self.m_guid).inspect_value(path, offset, limit)).map(|v| ("m_GUID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
