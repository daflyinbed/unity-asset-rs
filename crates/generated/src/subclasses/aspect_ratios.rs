//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AspectRatios {
    pub f_4_3: bool,
    pub f_5_4: bool,
    pub f_16_10: bool,
    pub f_16_9: bool,
    pub others: bool,
}

impl crate::Parse for AspectRatios {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let f_4_3 = rdr.read_bool()?;
        let f_5_4 = rdr.read_bool()?;
        let f_16_10 = rdr.read_bool()?;
        let f_16_9 = rdr.read_bool()?;
        let others = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(AspectRatios {
            f_4_3,
            f_5_4,
            f_16_10,
            f_16_9,
            others,
        })
    }
}

impl crate::ToValue for AspectRatios {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("4:3".to_string(), self.f_4_3.to_value())),
            Some(("5:4".to_string(), self.f_5_4.to_value())),
            Some(("16:10".to_string(), self.f_16_10.to_value())),
            Some(("16:9".to_string(), self.f_16_9.to_value())),
            Some(("Others".to_string(), self.others.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "4:3" => (self.f_4_3).inspect_value(path, offset, limit),
                "5:4" => (self.f_5_4).inspect_value(path, offset, limit),
                "16:10" => (self.f_16_10).inspect_value(path, offset, limit),
                "16:9" => (self.f_16_9).inspect_value(path, offset, limit),
                "Others" => (self.others).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.f_4_3).inspect_value(path, offset, limit)).map(|v| ("4:3".to_string(), v)),
            ((self.f_5_4).inspect_value(path, offset, limit)).map(|v| ("5:4".to_string(), v)),
            ((self.f_16_10).inspect_value(path, offset, limit)).map(|v| ("16:10".to_string(), v)),
            ((self.f_16_9).inspect_value(path, offset, limit)).map(|v| ("16:9".to_string(), v)),
            ((self.others).inspect_value(path, offset, limit)).map(|v| ("Others".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
