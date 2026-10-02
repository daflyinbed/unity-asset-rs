//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GroupConnection {
    pub source_group_index: u32,
    pub target_group_index: u32,
    pub send_effect_index: u32,
}

impl crate::Parse for GroupConnection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source_group_index = rdr.read_u32()?;
        let target_group_index = rdr.read_u32()?;
        let send_effect_index = rdr.read_u32()?;

        Ok(GroupConnection {
            source_group_index,
            target_group_index,
            send_effect_index,
        })
    }
}

impl crate::ToValue for GroupConnection {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "sourceGroupIndex".to_string(),
                self.source_group_index.to_value(),
            )),
            Some((
                "targetGroupIndex".to_string(),
                self.target_group_index.to_value(),
            )),
            Some((
                "sendEffectIndex".to_string(),
                self.send_effect_index.to_value(),
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
                "sourceGroupIndex" => (self.source_group_index).inspect_value(path, offset, limit),
                "targetGroupIndex" => (self.target_group_index).inspect_value(path, offset, limit),
                "sendEffectIndex" => (self.send_effect_index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.source_group_index).inspect_value(path, offset, limit))
                .map(|v| ("sourceGroupIndex".to_string(), v)),
            ((self.target_group_index).inspect_value(path, offset, limit))
                .map(|v| ("targetGroupIndex".to_string(), v)),
            ((self.send_effect_index).inspect_value(path, offset, limit))
                .map(|v| ("sendEffectIndex".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
