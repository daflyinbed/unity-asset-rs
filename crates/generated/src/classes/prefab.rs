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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_root_game_object = crate::PPtr::parse(r, ver)?;
        let m_hide_flags_behaviour = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_contains_missing_serialize_reference_types = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_bool()?)
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
