//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Hash128 {
    pub bytes_0: u8,
    pub bytes_1: u8,
    pub bytes_2: u8,
    pub bytes_3: u8,
    pub bytes_4: u8,
    pub bytes_5: u8,
    pub bytes_6: u8,
    pub bytes_7: u8,
    pub bytes_8: u8,
    pub bytes_9: u8,
    pub bytes_10: u8,
    pub bytes_11: u8,
    pub bytes_12: u8,
    pub bytes_13: u8,
    pub bytes_14: u8,
    pub bytes_15: u8,
}

impl crate::Parse for Hash128 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bytes_0 = rdr.read_u8()?;
        let bytes_1 = rdr.read_u8()?;
        let bytes_2 = rdr.read_u8()?;
        let bytes_3 = rdr.read_u8()?;
        let bytes_4 = rdr.read_u8()?;
        let bytes_5 = rdr.read_u8()?;
        let bytes_6 = rdr.read_u8()?;
        let bytes_7 = rdr.read_u8()?;
        let bytes_8 = rdr.read_u8()?;
        let bytes_9 = rdr.read_u8()?;
        let bytes_10 = rdr.read_u8()?;
        let bytes_11 = rdr.read_u8()?;
        let bytes_12 = rdr.read_u8()?;
        let bytes_13 = rdr.read_u8()?;
        let bytes_14 = rdr.read_u8()?;
        let bytes_15 = rdr.read_u8()?;

        Ok(Hash128 {
            bytes_0,
            bytes_1,
            bytes_2,
            bytes_3,
            bytes_4,
            bytes_5,
            bytes_6,
            bytes_7,
            bytes_8,
            bytes_9,
            bytes_10,
            bytes_11,
            bytes_12,
            bytes_13,
            bytes_14,
            bytes_15,
        })
    }
}

impl crate::ToValue for Hash128 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("bytes[0]".to_string(), self.bytes_0.to_value())),
            Some(("bytes[1]".to_string(), self.bytes_1.to_value())),
            Some(("bytes[2]".to_string(), self.bytes_2.to_value())),
            Some(("bytes[3]".to_string(), self.bytes_3.to_value())),
            Some(("bytes[4]".to_string(), self.bytes_4.to_value())),
            Some(("bytes[5]".to_string(), self.bytes_5.to_value())),
            Some(("bytes[6]".to_string(), self.bytes_6.to_value())),
            Some(("bytes[7]".to_string(), self.bytes_7.to_value())),
            Some(("bytes[8]".to_string(), self.bytes_8.to_value())),
            Some(("bytes[9]".to_string(), self.bytes_9.to_value())),
            Some(("bytes[10]".to_string(), self.bytes_10.to_value())),
            Some(("bytes[11]".to_string(), self.bytes_11.to_value())),
            Some(("bytes[12]".to_string(), self.bytes_12.to_value())),
            Some(("bytes[13]".to_string(), self.bytes_13.to_value())),
            Some(("bytes[14]".to_string(), self.bytes_14.to_value())),
            Some(("bytes[15]".to_string(), self.bytes_15.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "bytes[0]" => (self.bytes_0).inspect_value(path, offset, limit),
                "bytes[1]" => (self.bytes_1).inspect_value(path, offset, limit),
                "bytes[2]" => (self.bytes_2).inspect_value(path, offset, limit),
                "bytes[3]" => (self.bytes_3).inspect_value(path, offset, limit),
                "bytes[4]" => (self.bytes_4).inspect_value(path, offset, limit),
                "bytes[5]" => (self.bytes_5).inspect_value(path, offset, limit),
                "bytes[6]" => (self.bytes_6).inspect_value(path, offset, limit),
                "bytes[7]" => (self.bytes_7).inspect_value(path, offset, limit),
                "bytes[8]" => (self.bytes_8).inspect_value(path, offset, limit),
                "bytes[9]" => (self.bytes_9).inspect_value(path, offset, limit),
                "bytes[10]" => (self.bytes_10).inspect_value(path, offset, limit),
                "bytes[11]" => (self.bytes_11).inspect_value(path, offset, limit),
                "bytes[12]" => (self.bytes_12).inspect_value(path, offset, limit),
                "bytes[13]" => (self.bytes_13).inspect_value(path, offset, limit),
                "bytes[14]" => (self.bytes_14).inspect_value(path, offset, limit),
                "bytes[15]" => (self.bytes_15).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bytes_0).inspect_value(path, offset, limit))
                .map(|v| ("bytes[0]".to_string(), v)),
            ((self.bytes_1).inspect_value(path, offset, limit))
                .map(|v| ("bytes[1]".to_string(), v)),
            ((self.bytes_2).inspect_value(path, offset, limit))
                .map(|v| ("bytes[2]".to_string(), v)),
            ((self.bytes_3).inspect_value(path, offset, limit))
                .map(|v| ("bytes[3]".to_string(), v)),
            ((self.bytes_4).inspect_value(path, offset, limit))
                .map(|v| ("bytes[4]".to_string(), v)),
            ((self.bytes_5).inspect_value(path, offset, limit))
                .map(|v| ("bytes[5]".to_string(), v)),
            ((self.bytes_6).inspect_value(path, offset, limit))
                .map(|v| ("bytes[6]".to_string(), v)),
            ((self.bytes_7).inspect_value(path, offset, limit))
                .map(|v| ("bytes[7]".to_string(), v)),
            ((self.bytes_8).inspect_value(path, offset, limit))
                .map(|v| ("bytes[8]".to_string(), v)),
            ((self.bytes_9).inspect_value(path, offset, limit))
                .map(|v| ("bytes[9]".to_string(), v)),
            ((self.bytes_10).inspect_value(path, offset, limit))
                .map(|v| ("bytes[10]".to_string(), v)),
            ((self.bytes_11).inspect_value(path, offset, limit))
                .map(|v| ("bytes[11]".to_string(), v)),
            ((self.bytes_12).inspect_value(path, offset, limit))
                .map(|v| ("bytes[12]".to_string(), v)),
            ((self.bytes_13).inspect_value(path, offset, limit))
                .map(|v| ("bytes[13]".to_string(), v)),
            ((self.bytes_14).inspect_value(path, offset, limit))
                .map(|v| ("bytes[14]".to_string(), v)),
            ((self.bytes_15).inspect_value(path, offset, limit))
                .map(|v| ("bytes[15]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
