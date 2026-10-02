//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MotionNeighborList {
    pub m_neighbor_array: Vec<u32>,
}

impl crate::Parse for MotionNeighborList {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_neighbor_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(MotionNeighborList { m_neighbor_array })
    }
}

impl crate::ToValue for MotionNeighborList {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_NeighborArray".to_string(),
            self.m_neighbor_array.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NeighborArray" => (self.m_neighbor_array).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_neighbor_array).inspect_value(path, offset, limit))
                .map(|v| ("m_NeighborArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
