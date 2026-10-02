//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Guid {
    pub data_0: u32,
    pub data_1: u32,
    pub data_2: u32,
    pub data_3: u32,
}

impl crate::Parse for Guid {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data_0 = rdr.read_u32()?;
        let data_1 = rdr.read_u32()?;
        let data_2 = rdr.read_u32()?;
        let data_3 = rdr.read_u32()?;

        Ok(Guid {
            data_0,
            data_1,
            data_2,
            data_3,
        })
    }
}

impl crate::ToValue for Guid {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("data[0]".to_string(), self.data_0.to_value())),
            Some(("data[1]".to_string(), self.data_1.to_value())),
            Some(("data[2]".to_string(), self.data_2.to_value())),
            Some(("data[3]".to_string(), self.data_3.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "data[0]" => (self.data_0).inspect_value(path, offset, limit),
                "data[1]" => (self.data_1).inspect_value(path, offset, limit),
                "data[2]" => (self.data_2).inspect_value(path, offset, limit),
                "data[3]" => (self.data_3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.data_0).inspect_value(path, offset, limit)).map(|v| ("data[0]".to_string(), v)),
            ((self.data_1).inspect_value(path, offset, limit)).map(|v| ("data[1]".to_string(), v)),
            ((self.data_2).inspect_value(path, offset, limit)).map(|v| ("data[2]".to_string(), v)),
            ((self.data_3).inspect_value(path, offset, limit)).map(|v| ("data[3]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
