//! 自动生成。请勿手动修改。

use crate::subclasses::axes::Axes;
use crate::subclasses::node::Node;

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub m_node: Vec<Node>,
    pub m_id: Vec<u32>,
    pub m_axes_array: Vec<Axes>,
}

impl crate::Parse for Skeleton {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_node = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Node::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_id = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_axes_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Axes::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(Skeleton {
            m_node,
            m_id,
            m_axes_array,
        })
    }
}

impl crate::ToValue for Skeleton {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Node".to_string(), self.m_node.to_value())),
            Some(("m_ID".to_string(), self.m_id.to_value())),
            Some(("m_AxesArray".to_string(), self.m_axes_array.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Node" => (self.m_node).inspect_value(path, offset, limit),
                "m_ID" => (self.m_id).inspect_value(path, offset, limit),
                "m_AxesArray" => (self.m_axes_array).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_node).inspect_value(path, offset, limit)).map(|v| ("m_Node".to_string(), v)),
            ((self.m_id).inspect_value(path, offset, limit)).map(|v| ("m_ID".to_string(), v)),
            ((self.m_axes_array).inspect_value(path, offset, limit))
                .map(|v| ("m_AxesArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
