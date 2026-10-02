//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct NameToObjectMap_Shader {
    pub m_object_to_name: Vec<(PPtr, String)>,
}

impl crate::Parse for NameToObjectMap_Shader {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_to_name = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = crate::PPtr::parse(rdr, ver)?;
                        let b = rdr.read_aligned_string()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(NameToObjectMap_Shader { m_object_to_name })
    }
}

impl crate::ToValue for NameToObjectMap_Shader {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_ObjectToName".to_string(),
            self.m_object_to_name.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ObjectToName" => (self.m_object_to_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_object_to_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ObjectToName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
