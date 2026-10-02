//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubstanceValue {
    pub scalar_0: f32,
    pub scalar_1: f32,
    pub scalar_2: f32,
    pub scalar_3: f32,
    pub texture: PPtr,
    pub stringvalue: Option<String>,
}

impl crate::Parse for SubstanceValue {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let scalar_0 = rdr.read_f32()?;
        let scalar_1 = rdr.read_f32()?;
        let scalar_2 = rdr.read_f32()?;
        let scalar_3 = rdr.read_f32()?;
        let texture = crate::PPtr::parse(rdr, ver)?;
        let stringvalue = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(rdr.read_aligned_string()?)
        } else {
            None
        };

        Ok(SubstanceValue {
            scalar_0,
            scalar_1,
            scalar_2,
            scalar_3,
            texture,
            stringvalue,
        })
    }
}

impl crate::ToValue for SubstanceValue {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("scalar[0]".to_string(), self.scalar_0.to_value())),
            Some(("scalar[1]".to_string(), self.scalar_1.to_value())),
            Some(("scalar[2]".to_string(), self.scalar_2.to_value())),
            Some(("scalar[3]".to_string(), self.scalar_3.to_value())),
            Some(("texture".to_string(), self.texture.to_value())),
            crate::opt_field("stringvalue", &self.stringvalue),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "scalar[0]" => (self.scalar_0).inspect_value(path, offset, limit),
                "scalar[1]" => (self.scalar_1).inspect_value(path, offset, limit),
                "scalar[2]" => (self.scalar_2).inspect_value(path, offset, limit),
                "scalar[3]" => (self.scalar_3).inspect_value(path, offset, limit),
                "texture" => (self.texture).inspect_value(path, offset, limit),
                "stringvalue" => (self.stringvalue).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.scalar_0).inspect_value(path, offset, limit))
                .map(|v| ("scalar[0]".to_string(), v)),
            ((self.scalar_1).inspect_value(path, offset, limit))
                .map(|v| ("scalar[1]".to_string(), v)),
            ((self.scalar_2).inspect_value(path, offset, limit))
                .map(|v| ("scalar[2]".to_string(), v)),
            ((self.scalar_3).inspect_value(path, offset, limit))
                .map(|v| ("scalar[3]".to_string(), v)),
            ((self.texture).inspect_value(path, offset, limit)).map(|v| ("texture".to_string(), v)),
            ((self.stringvalue).inspect_value(path, offset, limit))
                .map(|v| ("stringvalue".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
