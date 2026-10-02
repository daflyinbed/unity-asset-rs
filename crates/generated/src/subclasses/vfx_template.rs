//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxTemplate {
    pub name: String,
    pub category: String,
    pub description: String,
    pub icon: PPtr,
    pub thumbnail: PPtr,
    pub order: Option<i32>,
}

impl crate::Parse for VfxTemplate {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let category = rdr.read_aligned_string()?;
        let description = rdr.read_aligned_string()?;
        let icon = crate::PPtr::parse(rdr, ver)?;
        let thumbnail = crate::PPtr::parse(rdr, ver)?;
        let order = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(VfxTemplate {
            name,
            category,
            description,
            icon,
            thumbnail,
            order,
        })
    }
}

impl crate::ToValue for VfxTemplate {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("category".to_string(), self.category.to_value())),
            Some(("description".to_string(), self.description.to_value())),
            Some(("icon".to_string(), self.icon.to_value())),
            Some(("thumbnail".to_string(), self.thumbnail.to_value())),
            crate::opt_field("order", &self.order),
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
                "category" => (self.category).inspect_value(path, offset, limit),
                "description" => (self.description).inspect_value(path, offset, limit),
                "icon" => (self.icon).inspect_value(path, offset, limit),
                "thumbnail" => (self.thumbnail).inspect_value(path, offset, limit),
                "order" => (self.order).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.category).inspect_value(path, offset, limit))
                .map(|v| ("category".to_string(), v)),
            ((self.description).inspect_value(path, offset, limit))
                .map(|v| ("description".to_string(), v)),
            ((self.icon).inspect_value(path, offset, limit)).map(|v| ("icon".to_string(), v)),
            ((self.thumbnail).inspect_value(path, offset, limit))
                .map(|v| ("thumbnail".to_string(), v)),
            ((self.order).inspect_value(path, offset, limit)).map(|v| ("order".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
