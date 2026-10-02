//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AddedGameObject {
    pub target_corresponding_source_object: PPtr,
    pub insert_index: i32,
    pub added_object: PPtr,
}

impl crate::Parse for AddedGameObject {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_corresponding_source_object = crate::PPtr::parse(rdr, ver)?;
        let insert_index = rdr.read_i32()?;
        let added_object = crate::PPtr::parse(rdr, ver)?;

        Ok(AddedGameObject {
            target_corresponding_source_object,
            insert_index,
            added_object,
        })
    }
}

impl crate::ToValue for AddedGameObject {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "targetCorrespondingSourceObject".to_string(),
                self.target_corresponding_source_object.to_value(),
            )),
            Some(("insertIndex".to_string(), self.insert_index.to_value())),
            Some(("addedObject".to_string(), self.added_object.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "targetCorrespondingSourceObject" => {
                    (self.target_corresponding_source_object).inspect_value(path, offset, limit)
                }
                "insertIndex" => (self.insert_index).inspect_value(path, offset, limit),
                "addedObject" => (self.added_object).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.target_corresponding_source_object).inspect_value(path, offset, limit))
                .map(|v| ("targetCorrespondingSourceObject".to_string(), v)),
            ((self.insert_index).inspect_value(path, offset, limit))
                .map(|v| ("insertIndex".to_string(), v)),
            ((self.added_object).inspect_value(path, offset, limit))
                .map(|v| ("addedObject".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
