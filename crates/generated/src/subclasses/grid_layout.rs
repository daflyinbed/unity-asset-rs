//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct GridLayout {
    pub m_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
}

impl crate::Parse for GridLayout {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hide_flags = rdr.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(rdr, ver)?;
        let m_prefab_internal = crate::PPtr::parse(rdr, ver)?;
        let m_game_object = crate::PPtr::parse(rdr, ver)?;

        Ok(GridLayout {
            m_hide_flags,
            m_corresponding_source_object,
            m_prefab_internal,
            m_game_object,
        })
    }
}

impl crate::ToValue for GridLayout {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_HideFlags".to_string(), self.m_hide_flags.to_value())),
            Some((
                "m_CorrespondingSourceObject".to_string(),
                self.m_corresponding_source_object.to_value(),
            )),
            Some((
                "m_PrefabInternal".to_string(),
                self.m_prefab_internal.to_value(),
            )),
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_HideFlags" => (self.m_hide_flags).inspect_value(path, offset, limit),
                "m_CorrespondingSourceObject" => {
                    (self.m_corresponding_source_object).inspect_value(path, offset, limit)
                }
                "m_PrefabInternal" => (self.m_prefab_internal).inspect_value(path, offset, limit),
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_hide_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_HideFlags".to_string(), v)),
            ((self.m_corresponding_source_object).inspect_value(path, offset, limit))
                .map(|v| ("m_CorrespondingSourceObject".to_string(), v)),
            ((self.m_prefab_internal).inspect_value(path, offset, limit))
                .map(|v| ("m_PrefabInternal".to_string(), v)),
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
