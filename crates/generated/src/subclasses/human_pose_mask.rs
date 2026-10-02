//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HumanPoseMask {
    pub word0: u32,
    pub word1: u32,
    pub word2: Option<u32>,
}

impl crate::Parse for HumanPoseMask {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let word0 = rdr.read_u32()?;
        let word1 = rdr.read_u32()?;
        let word2 = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };

        Ok(HumanPoseMask {
            word0,
            word1,
            word2,
        })
    }
}

impl crate::ToValue for HumanPoseMask {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("word0".to_string(), self.word0.to_value())),
            Some(("word1".to_string(), self.word1.to_value())),
            crate::opt_field("word2", &self.word2),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "word0" => (self.word0).inspect_value(path, offset, limit),
                "word1" => (self.word1).inspect_value(path, offset, limit),
                "word2" => (self.word2).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.word0).inspect_value(path, offset, limit)).map(|v| ("word0".to_string(), v)),
            ((self.word1).inspect_value(path, offset, limit)).map(|v| ("word1".to_string(), v)),
            ((self.word2).inspect_value(path, offset, limit)).map(|v| ("word2".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
