//! 自动生成。请勿手动修改。

use crate::subclasses::hint::Hint;

#[derive(Debug, Clone)]
pub struct ReflectedParameter {
    pub m_direction: i32,
    pub m_type_name: String,
    pub m_name: String,
    pub m_hints: Vec<Hint>,
}

impl crate::Parse for ReflectedParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_direction = rdr.read_i32()?;
        let m_type_name = rdr.read_aligned_string()?;
        let m_name = rdr.read_aligned_string()?;
        let m_hints = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Hint::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(ReflectedParameter {
            m_direction,
            m_type_name,
            m_name,
            m_hints,
        })
    }
}

impl crate::ToValue for ReflectedParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Direction".to_string(), self.m_direction.to_value())),
            Some(("m_TypeName".to_string(), self.m_type_name.to_value())),
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Hints".to_string(), self.m_hints.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Direction" => (self.m_direction).inspect_value(path, offset, limit),
                "m_TypeName" => (self.m_type_name).inspect_value(path, offset, limit),
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Hints" => (self.m_hints).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_direction).inspect_value(path, offset, limit))
                .map(|v| ("m_Direction".to_string(), v)),
            ((self.m_type_name).inspect_value(path, offset, limit))
                .map(|v| ("m_TypeName".to_string(), v)),
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_hints).inspect_value(path, offset, limit)).map(|v| ("m_Hints".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
