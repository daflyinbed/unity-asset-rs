//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxExposedMapping {
    pub mapping: VfxMapping,
    pub space: i32,
}

impl crate::Parse for VfxExposedMapping {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mapping = VfxMapping::parse(rdr, ver)?;
        let space = rdr.read_i32()?;

        Ok(VfxExposedMapping { mapping, space })
    }
}

impl crate::ToValue for VfxExposedMapping {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("mapping".to_string(), self.mapping.to_value())),
            Some(("space".to_string(), self.space.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "mapping" => (self.mapping).inspect_value(path, offset, limit),
                "space" => (self.space).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.mapping).inspect_value(path, offset, limit)).map(|v| ("mapping".to_string(), v)),
            ((self.space).inspect_value(path, offset, limit)).map(|v| ("space".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
