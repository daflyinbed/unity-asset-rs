//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct BlendTreeConstant {
    pub m_node_array: Vec<OffsetPtr>,
    pub m_blend_event_array_constant: Option<OffsetPtr>,
}

impl crate::Parse for BlendTreeConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_node_array = None;
        let mut m_blend_event_array_constant = None;
        m_node_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OffsetPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
        } else {
            m_blend_event_array_constant = Some(OffsetPtr::parse(rdr, ver)?);
        }

        Ok(BlendTreeConstant {
            m_node_array: m_node_array.unwrap(),
            m_blend_event_array_constant,
        })
    }
}

impl crate::ToValue for BlendTreeConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NodeArray".to_string(), self.m_node_array.to_value())),
            crate::opt_field(
                "m_BlendEventArrayConstant",
                &self.m_blend_event_array_constant,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NodeArray" => (self.m_node_array).inspect_value(path, offset, limit),
                "m_BlendEventArrayConstant" => {
                    (self.m_blend_event_array_constant).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_node_array).inspect_value(path, offset, limit))
                .map(|v| ("m_NodeArray".to_string(), v)),
            ((self.m_blend_event_array_constant).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendEventArrayConstant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
