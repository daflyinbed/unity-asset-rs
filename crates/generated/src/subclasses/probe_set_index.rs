//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct ProbeSetIndex {
    pub m_hash: Hash128,
    pub m_offset: i32,
    pub m_size: i32,
}

impl crate::Parse for ProbeSetIndex {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hash = Hash128::parse(rdr, ver)?;
        let m_offset = rdr.read_i32()?;
        let m_size = rdr.read_i32()?;

        Ok(ProbeSetIndex {
            m_hash,
            m_offset,
            m_size,
        })
    }
}

impl crate::ToValue for ProbeSetIndex {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Hash".to_string(), self.m_hash.to_value())),
            Some(("m_Offset".to_string(), self.m_offset.to_value())),
            Some(("m_Size".to_string(), self.m_size.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Hash" => (self.m_hash).inspect_value(path, offset, limit),
                "m_Offset" => (self.m_offset).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_hash).inspect_value(path, offset, limit)).map(|v| ("m_Hash".to_string(), v)),
            ((self.m_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_Offset".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
