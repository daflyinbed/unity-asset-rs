//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AvatarSkeletonMaskElement {
    pub path: String,
    pub weight: f32,
}

impl crate::Parse for AvatarSkeletonMaskElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = rdr.read_aligned_string()?;
        let weight = rdr.read_f32()?;

        Ok(AvatarSkeletonMaskElement { path, weight })
    }
}

impl crate::ToValue for AvatarSkeletonMaskElement {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("path".to_string(), self.path.to_value())),
            Some(("weight".to_string(), self.weight.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "path" => (self.path).inspect_value(path, offset, limit),
                "weight" => (self.weight).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.path).inspect_value(path, offset, limit)).map(|v| ("path".to_string(), v)),
            ((self.weight).inspect_value(path, offset, limit)).map(|v| ("weight".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
