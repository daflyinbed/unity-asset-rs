//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Hand {
    pub m_hand_bone_index: Vec<i32>,
}

impl crate::Parse for Hand {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hand_bone_index = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(Hand { m_hand_bone_index })
    }
}

impl crate::ToValue for Hand {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_HandBoneIndex".to_string(),
            self.m_hand_bone_index.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_HandBoneIndex" => (self.m_hand_bone_index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_hand_bone_index).inspect_value(path, offset, limit))
                .map(|v| ("m_HandBoneIndex".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
