//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct FalloffTable {
    pub m_table_0: f32,
    pub m_table_1: f32,
    pub m_table_2: f32,
    pub m_table_3: f32,
    pub m_table_4: f32,
    pub m_table_5: f32,
    pub m_table_6: f32,
    pub m_table_7: f32,
    pub m_table_8: f32,
    pub m_table_9: f32,
    pub m_table_10: f32,
    pub m_table_11: f32,
    pub m_table_12: f32,
}

impl crate::Parse for FalloffTable {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_table_0 = rdr.read_f32()?;
        let m_table_1 = rdr.read_f32()?;
        let m_table_2 = rdr.read_f32()?;
        let m_table_3 = rdr.read_f32()?;
        let m_table_4 = rdr.read_f32()?;
        let m_table_5 = rdr.read_f32()?;
        let m_table_6 = rdr.read_f32()?;
        let m_table_7 = rdr.read_f32()?;
        let m_table_8 = rdr.read_f32()?;
        let m_table_9 = rdr.read_f32()?;
        let m_table_10 = rdr.read_f32()?;
        let m_table_11 = rdr.read_f32()?;
        let m_table_12 = rdr.read_f32()?;

        Ok(FalloffTable {
            m_table_0,
            m_table_1,
            m_table_2,
            m_table_3,
            m_table_4,
            m_table_5,
            m_table_6,
            m_table_7,
            m_table_8,
            m_table_9,
            m_table_10,
            m_table_11,
            m_table_12,
        })
    }
}

impl crate::ToValue for FalloffTable {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Table[0]".to_string(), self.m_table_0.to_value())),
            Some(("m_Table[1]".to_string(), self.m_table_1.to_value())),
            Some(("m_Table[2]".to_string(), self.m_table_2.to_value())),
            Some(("m_Table[3]".to_string(), self.m_table_3.to_value())),
            Some(("m_Table[4]".to_string(), self.m_table_4.to_value())),
            Some(("m_Table[5]".to_string(), self.m_table_5.to_value())),
            Some(("m_Table[6]".to_string(), self.m_table_6.to_value())),
            Some(("m_Table[7]".to_string(), self.m_table_7.to_value())),
            Some(("m_Table[8]".to_string(), self.m_table_8.to_value())),
            Some(("m_Table[9]".to_string(), self.m_table_9.to_value())),
            Some(("m_Table[10]".to_string(), self.m_table_10.to_value())),
            Some(("m_Table[11]".to_string(), self.m_table_11.to_value())),
            Some(("m_Table[12]".to_string(), self.m_table_12.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Table[0]" => (self.m_table_0).inspect_value(path, offset, limit),
                "m_Table[1]" => (self.m_table_1).inspect_value(path, offset, limit),
                "m_Table[2]" => (self.m_table_2).inspect_value(path, offset, limit),
                "m_Table[3]" => (self.m_table_3).inspect_value(path, offset, limit),
                "m_Table[4]" => (self.m_table_4).inspect_value(path, offset, limit),
                "m_Table[5]" => (self.m_table_5).inspect_value(path, offset, limit),
                "m_Table[6]" => (self.m_table_6).inspect_value(path, offset, limit),
                "m_Table[7]" => (self.m_table_7).inspect_value(path, offset, limit),
                "m_Table[8]" => (self.m_table_8).inspect_value(path, offset, limit),
                "m_Table[9]" => (self.m_table_9).inspect_value(path, offset, limit),
                "m_Table[10]" => (self.m_table_10).inspect_value(path, offset, limit),
                "m_Table[11]" => (self.m_table_11).inspect_value(path, offset, limit),
                "m_Table[12]" => (self.m_table_12).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_table_0).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[0]".to_string(), v)),
            ((self.m_table_1).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[1]".to_string(), v)),
            ((self.m_table_2).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[2]".to_string(), v)),
            ((self.m_table_3).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[3]".to_string(), v)),
            ((self.m_table_4).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[4]".to_string(), v)),
            ((self.m_table_5).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[5]".to_string(), v)),
            ((self.m_table_6).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[6]".to_string(), v)),
            ((self.m_table_7).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[7]".to_string(), v)),
            ((self.m_table_8).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[8]".to_string(), v)),
            ((self.m_table_9).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[9]".to_string(), v)),
            ((self.m_table_10).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[10]".to_string(), v)),
            ((self.m_table_11).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[11]".to_string(), v)),
            ((self.m_table_12).inspect_value(path, offset, limit))
                .map(|v| ("m_Table[12]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
