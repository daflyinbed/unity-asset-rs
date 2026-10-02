//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PropertyModification {
    pub target: PPtr,
    pub property_path: String,
    pub value: String,
    pub object_reference: PPtr,
}

impl crate::Parse for PropertyModification {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target = crate::PPtr::parse(rdr, ver)?;
        let property_path = rdr.read_aligned_string()?;
        let value = rdr.read_aligned_string()?;
        let object_reference = crate::PPtr::parse(rdr, ver)?;

        Ok(PropertyModification {
            target,
            property_path,
            value,
            object_reference,
        })
    }
}

impl crate::ToValue for PropertyModification {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("target".to_string(), self.target.to_value())),
            Some(("propertyPath".to_string(), self.property_path.to_value())),
            Some(("value".to_string(), self.value.to_value())),
            Some((
                "objectReference".to_string(),
                self.object_reference.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "target" => (self.target).inspect_value(path, offset, limit),
                "propertyPath" => (self.property_path).inspect_value(path, offset, limit),
                "value" => (self.value).inspect_value(path, offset, limit),
                "objectReference" => (self.object_reference).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.target).inspect_value(path, offset, limit)).map(|v| ("target".to_string(), v)),
            ((self.property_path).inspect_value(path, offset, limit))
                .map(|v| ("propertyPath".to_string(), v)),
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
            ((self.object_reference).inspect_value(path, offset, limit))
                .map(|v| ("objectReference".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
