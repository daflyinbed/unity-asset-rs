//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Matrix3x4f {
    pub e00: f32,
    pub e01: f32,
    pub e02: f32,
    pub e03: f32,
    pub e10: f32,
    pub e11: f32,
    pub e12: f32,
    pub e13: f32,
    pub e20: f32,
    pub e21: f32,
    pub e22: f32,
    pub e23: f32,
}

impl crate::Parse for Matrix3x4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let e00 = rdr.read_f32()?;
        let e01 = rdr.read_f32()?;
        let e02 = rdr.read_f32()?;
        let e03 = rdr.read_f32()?;
        let e10 = rdr.read_f32()?;
        let e11 = rdr.read_f32()?;
        let e12 = rdr.read_f32()?;
        let e13 = rdr.read_f32()?;
        let e20 = rdr.read_f32()?;
        let e21 = rdr.read_f32()?;
        let e22 = rdr.read_f32()?;
        let e23 = rdr.read_f32()?;

        Ok(Matrix3x4f {
            e00,
            e01,
            e02,
            e03,
            e10,
            e11,
            e12,
            e13,
            e20,
            e21,
            e22,
            e23,
        })
    }
}

impl crate::ToValue for Matrix3x4f {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("e00".to_string(), self.e00.to_value())),
            Some(("e01".to_string(), self.e01.to_value())),
            Some(("e02".to_string(), self.e02.to_value())),
            Some(("e03".to_string(), self.e03.to_value())),
            Some(("e10".to_string(), self.e10.to_value())),
            Some(("e11".to_string(), self.e11.to_value())),
            Some(("e12".to_string(), self.e12.to_value())),
            Some(("e13".to_string(), self.e13.to_value())),
            Some(("e20".to_string(), self.e20.to_value())),
            Some(("e21".to_string(), self.e21.to_value())),
            Some(("e22".to_string(), self.e22.to_value())),
            Some(("e23".to_string(), self.e23.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "e00" => (self.e00).inspect_value(path, offset, limit),
                "e01" => (self.e01).inspect_value(path, offset, limit),
                "e02" => (self.e02).inspect_value(path, offset, limit),
                "e03" => (self.e03).inspect_value(path, offset, limit),
                "e10" => (self.e10).inspect_value(path, offset, limit),
                "e11" => (self.e11).inspect_value(path, offset, limit),
                "e12" => (self.e12).inspect_value(path, offset, limit),
                "e13" => (self.e13).inspect_value(path, offset, limit),
                "e20" => (self.e20).inspect_value(path, offset, limit),
                "e21" => (self.e21).inspect_value(path, offset, limit),
                "e22" => (self.e22).inspect_value(path, offset, limit),
                "e23" => (self.e23).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.e00).inspect_value(path, offset, limit)).map(|v| ("e00".to_string(), v)),
            ((self.e01).inspect_value(path, offset, limit)).map(|v| ("e01".to_string(), v)),
            ((self.e02).inspect_value(path, offset, limit)).map(|v| ("e02".to_string(), v)),
            ((self.e03).inspect_value(path, offset, limit)).map(|v| ("e03".to_string(), v)),
            ((self.e10).inspect_value(path, offset, limit)).map(|v| ("e10".to_string(), v)),
            ((self.e11).inspect_value(path, offset, limit)).map(|v| ("e11".to_string(), v)),
            ((self.e12).inspect_value(path, offset, limit)).map(|v| ("e12".to_string(), v)),
            ((self.e13).inspect_value(path, offset, limit)).map(|v| ("e13".to_string(), v)),
            ((self.e20).inspect_value(path, offset, limit)).map(|v| ("e20".to_string(), v)),
            ((self.e21).inspect_value(path, offset, limit)).map(|v| ("e21".to_string(), v)),
            ((self.e22).inspect_value(path, offset, limit)).map(|v| ("e22".to_string(), v)),
            ((self.e23).inspect_value(path, offset, limit)).map(|v| ("e23".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
