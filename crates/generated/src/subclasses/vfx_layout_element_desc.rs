//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_offset::VfxLayoutOffset;

#[derive(Debug, Clone)]
pub struct VfxLayoutElementDesc {
    pub name: String,
    pub r#type: i32,
    pub offset: VfxLayoutOffset,
}

impl crate::Parse for VfxLayoutElementDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let r#type = rdr.read_i32()?;
        let offset = VfxLayoutOffset::parse(rdr, ver)?;

        Ok(VfxLayoutElementDesc {
            name,
            r#type,
            offset,
        })
    }
}

impl crate::ToValue for VfxLayoutElementDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("offset".to_string(), self.offset.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
