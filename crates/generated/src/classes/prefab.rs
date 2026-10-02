//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Prefab {
    pub m_root_game_object: PPtr,
    pub m_hide_flags_behaviour: Option<i32>,
    pub m_contains_missing_serialize_reference_types: Option<bool>,
}

impl crate::Parse for Prefab {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_root_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_hide_flags_behaviour = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_contains_missing_serialize_reference_types =
            if *ver >= crate::UnityVersion::new(2021, 2, 0) {
                Some(rdr.read_bool()?)
            } else {
                None
            };

        Ok(Prefab {
            m_root_game_object,
            m_hide_flags_behaviour,
            m_contains_missing_serialize_reference_types,
        })
    }
}

impl crate::ToValue for Prefab {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_RootGameObject".to_string(),
                self.m_root_game_object.to_value(),
            )),
            crate::opt_field("m_HideFlagsBehaviour", &self.m_hide_flags_behaviour),
            crate::opt_field(
                "m_ContainsMissingSerializeReferenceTypes",
                &self.m_contains_missing_serialize_reference_types,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_RootGameObject" => (self.m_root_game_object).inspect_value(path, offset, limit),
                "m_HideFlagsBehaviour" => {
                    (self.m_hide_flags_behaviour).inspect_value(path, offset, limit)
                }
                "m_ContainsMissingSerializeReferenceTypes" => (self
                    .m_contains_missing_serialize_reference_types)
                    .inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_root_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_RootGameObject".to_string(), v)),
            ((self.m_hide_flags_behaviour).inspect_value(path, offset, limit))
                .map(|v| ("m_HideFlagsBehaviour".to_string(), v)),
            ((self.m_contains_missing_serialize_reference_types)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_ContainsMissingSerializeReferenceTypes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
