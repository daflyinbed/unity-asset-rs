//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct SortingGroup {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_sorting_layer: i16,
    pub m_sorting_layer_id: Option<i32>,
    pub m_sorting_order: i16,
    pub m_sort_at_root: Option<bool>,
    pub m_sort3_d_as2_d: Option<bool>,
}

impl crate::Parse for SortingGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_sorting_layer = r.read_i16()?;
        let m_sorting_layer_id = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sorting_order = r.read_i16()?;
        r.align(4)?;
        let m_sort_at_root = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sort3_d_as2_d = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SortingGroup {
            parent,
            m_game_object,
            m_enabled,
            m_sorting_layer,
            m_sorting_layer_id,
            m_sorting_order,
            m_sort_at_root,
            m_sort3_d_as2_d,
        })
    }
}
