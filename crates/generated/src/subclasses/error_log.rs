//! 自动生成。请勿手动修改。

use crate::subclasses::message::Message;

#[derive(Debug, Clone)]
pub struct ErrorLog {
    pub m_messages: Vec<Message>,
    pub m_has_errors: bool,
}

impl crate::Parse for ErrorLog {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_messages = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Message::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_has_errors = rdr.read_bool()?;

        Ok(ErrorLog {
            m_messages,
            m_has_errors,
        })
    }
}

impl crate::ToValue for ErrorLog {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Messages".to_string(), self.m_messages.to_value())),
            Some(("m_HasErrors".to_string(), self.m_has_errors.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Messages" => (self.m_messages).inspect_value(path, offset, limit),
                "m_HasErrors" => (self.m_has_errors).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_messages).inspect_value(path, offset, limit))
                .map(|v| ("m_Messages".to_string(), v)),
            ((self.m_has_errors).inspect_value(path, offset, limit))
                .map(|v| ("m_HasErrors".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
