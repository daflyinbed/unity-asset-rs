//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SnapshotConstant {
    pub name_hash: u32,
    pub values: Vec<f32>,
    pub transition_types: Vec<u32>,
    pub transition_indices: Vec<u32>,
}

impl crate::Parse for SnapshotConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name_hash = rdr.read_u32()?;
        let values = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let transition_types = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let transition_indices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SnapshotConstant {
            name_hash,
            values,
            transition_types,
            transition_indices,
        })
    }
}

impl crate::ToValue for SnapshotConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("nameHash".to_string(), self.name_hash.to_value())),
            Some(("values".to_string(), self.values.to_value())),
            Some((
                "transitionTypes".to_string(),
                self.transition_types.to_value(),
            )),
            Some((
                "transitionIndices".to_string(),
                self.transition_indices.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "nameHash" => (self.name_hash).inspect_value(path, offset, limit),
                "values" => (self.values).inspect_value(path, offset, limit),
                "transitionTypes" => (self.transition_types).inspect_value(path, offset, limit),
                "transitionIndices" => (self.transition_indices).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name_hash).inspect_value(path, offset, limit))
                .map(|v| ("nameHash".to_string(), v)),
            ((self.values).inspect_value(path, offset, limit)).map(|v| ("values".to_string(), v)),
            ((self.transition_types).inspect_value(path, offset, limit))
                .map(|v| ("transitionTypes".to_string(), v)),
            ((self.transition_indices).inspect_value(path, offset, limit))
                .map(|v| ("transitionIndices".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
