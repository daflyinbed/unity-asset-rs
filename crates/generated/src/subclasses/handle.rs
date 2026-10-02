//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct Handle {
    pub m_x: xform,
    pub m_parent_human_index: u32,
    pub m_id: u32,
}

impl crate::Parse for Handle {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(rdr, ver)?;
        let m_parent_human_index = rdr.read_u32()?;
        let m_id = rdr.read_u32()?;

        Ok(Handle {
            m_x,
            m_parent_human_index,
            m_id,
        })
    }
}

impl crate::ToValue for Handle {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_X".to_string(), self.m_x.to_value())),
            Some((
                "m_ParentHumanIndex".to_string(),
                self.m_parent_human_index.to_value(),
            )),
            Some(("m_ID".to_string(), self.m_id.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_X" => (self.m_x).inspect_value(path, offset, limit),
                "m_ParentHumanIndex" => {
                    (self.m_parent_human_index).inspect_value(path, offset, limit)
                }
                "m_ID" => (self.m_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_x).inspect_value(path, offset, limit)).map(|v| ("m_X".to_string(), v)),
            ((self.m_parent_human_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ParentHumanIndex".to_string(), v)),
            ((self.m_id).inspect_value(path, offset, limit)).map(|v| ("m_ID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
