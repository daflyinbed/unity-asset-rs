//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SecondarySpriteTexture {
    pub texture: PPtr,
    pub name: String,
}

impl crate::Parse for SecondarySpriteTexture {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture = crate::PPtr::parse(rdr, ver)?;
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(SecondarySpriteTexture { texture, name })
    }
}

impl crate::ToValue for SecondarySpriteTexture {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("texture".to_string(), self.texture.to_value())),
            Some(("name".to_string(), self.name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "texture" => (self.texture).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.texture).inspect_value(path, offset, limit)).map(|v| ("texture".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
