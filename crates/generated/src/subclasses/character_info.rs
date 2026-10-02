//! 自动生成。请勿手动修改。

use crate::subclasses::rectf::Rectf;

#[derive(Debug, Clone)]
pub struct CharacterInfo {
    pub index: u32,
    pub uv: Rectf,
    pub vert: Rectf,
    pub width: Option<f32>,
    pub advance: Option<f32>,
    pub flipped: Option<bool>,
}

impl crate::Parse for CharacterInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut index = None;
        let mut uv = None;
        let mut vert = None;
        let mut width = None;
        let mut advance = None;
        let mut flipped = None;
        index = Some(rdr.read_u32()?);
        uv = Some(Rectf::parse(rdr, ver)?);
        vert = Some(Rectf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            advance = Some(rdr.read_f32()?);
            flipped = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            width = Some(rdr.read_f32()?);
            flipped = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            width = Some(rdr.read_f32()?);
        }

        Ok(CharacterInfo {
            index: index.unwrap(),
            uv: uv.unwrap(),
            vert: vert.unwrap(),
            width,
            advance,
            flipped,
        })
    }
}

impl crate::ToValue for CharacterInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("index".to_string(), self.index.to_value())),
            Some(("uv".to_string(), self.uv.to_value())),
            Some(("vert".to_string(), self.vert.to_value())),
            crate::opt_field("width", &self.width),
            crate::opt_field("advance", &self.advance),
            crate::opt_field("flipped", &self.flipped),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "index" => (self.index).inspect_value(path, offset, limit),
                "uv" => (self.uv).inspect_value(path, offset, limit),
                "vert" => (self.vert).inspect_value(path, offset, limit),
                "width" => (self.width).inspect_value(path, offset, limit),
                "advance" => (self.advance).inspect_value(path, offset, limit),
                "flipped" => (self.flipped).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.index).inspect_value(path, offset, limit)).map(|v| ("index".to_string(), v)),
            ((self.uv).inspect_value(path, offset, limit)).map(|v| ("uv".to_string(), v)),
            ((self.vert).inspect_value(path, offset, limit)).map(|v| ("vert".to_string(), v)),
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("width".to_string(), v)),
            ((self.advance).inspect_value(path, offset, limit)).map(|v| ("advance".to_string(), v)),
            ((self.flipped).inspect_value(path, offset, limit)).map(|v| ("flipped".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
