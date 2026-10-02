//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct BrokenPrefabAsset {
    pub m_name: String,
    pub m_message: String,
    pub m_is_warning: bool,
    pub m_is_variant: bool,
    pub m_is_prefab_file_valid: bool,
    pub m_broken_parent_prefab: PPtr,
}

impl crate::Parse for BrokenPrefabAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_message = rdr.read_aligned_string()?;
        let m_is_warning = rdr.read_bool()?;
        let m_is_variant = rdr.read_bool()?;
        let m_is_prefab_file_valid = rdr.read_bool()?;
        rdr.align(4)?;
        let m_broken_parent_prefab = crate::PPtr::parse(rdr, ver)?;

        Ok(BrokenPrefabAsset {
            m_name,
            m_message,
            m_is_warning,
            m_is_variant,
            m_is_prefab_file_valid,
            m_broken_parent_prefab,
        })
    }
}

impl crate::ToValue for BrokenPrefabAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Message".to_string(), self.m_message.to_value())),
            Some(("m_IsWarning".to_string(), self.m_is_warning.to_value())),
            Some(("m_IsVariant".to_string(), self.m_is_variant.to_value())),
            Some((
                "m_IsPrefabFileValid".to_string(),
                self.m_is_prefab_file_valid.to_value(),
            )),
            Some((
                "m_BrokenParentPrefab".to_string(),
                self.m_broken_parent_prefab.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Message" => (self.m_message).inspect_value(path, offset, limit),
                "m_IsWarning" => (self.m_is_warning).inspect_value(path, offset, limit),
                "m_IsVariant" => (self.m_is_variant).inspect_value(path, offset, limit),
                "m_IsPrefabFileValid" => {
                    (self.m_is_prefab_file_valid).inspect_value(path, offset, limit)
                }
                "m_BrokenParentPrefab" => {
                    (self.m_broken_parent_prefab).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_message).inspect_value(path, offset, limit))
                .map(|v| ("m_Message".to_string(), v)),
            ((self.m_is_warning).inspect_value(path, offset, limit))
                .map(|v| ("m_IsWarning".to_string(), v)),
            ((self.m_is_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_IsVariant".to_string(), v)),
            ((self.m_is_prefab_file_valid).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPrefabFileValid".to_string(), v)),
            ((self.m_broken_parent_prefab).inspect_value(path, offset, limit))
                .map(|v| ("m_BrokenParentPrefab".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
