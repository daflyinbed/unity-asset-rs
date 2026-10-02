//! 自动生成。请勿手动修改。

use crate::subclasses::source_location::SourceLocation;

#[derive(Debug, Clone)]
pub struct Message {
    pub m_severity: i32,
    pub m_code: i32,
    pub m_location: SourceLocation,
    pub m_text: String,
}

impl crate::Parse for Message {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_severity = rdr.read_i32()?;
        let m_code = rdr.read_i32()?;
        let m_location = SourceLocation::parse(rdr, ver)?;
        let m_text = rdr.read_aligned_string()?;

        Ok(Message {
            m_severity,
            m_code,
            m_location,
            m_text,
        })
    }
}

impl crate::ToValue for Message {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Severity".to_string(), self.m_severity.to_value())),
            Some(("m_Code".to_string(), self.m_code.to_value())),
            Some(("m_Location".to_string(), self.m_location.to_value())),
            Some(("m_Text".to_string(), self.m_text.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Severity" => (self.m_severity).inspect_value(path, offset, limit),
                "m_Code" => (self.m_code).inspect_value(path, offset, limit),
                "m_Location" => (self.m_location).inspect_value(path, offset, limit),
                "m_Text" => (self.m_text).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_severity).inspect_value(path, offset, limit))
                .map(|v| ("m_Severity".to_string(), v)),
            ((self.m_code).inspect_value(path, offset, limit)).map(|v| ("m_Code".to_string(), v)),
            ((self.m_location).inspect_value(path, offset, limit))
                .map(|v| ("m_Location".to_string(), v)),
            ((self.m_text).inspect_value(path, offset, limit)).map(|v| ("m_Text".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
