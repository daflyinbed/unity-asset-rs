//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::director_generic_binding::DirectorGenericBinding;
use crate::subclasses::exposed_reference_table::ExposedReferenceTable;

#[derive(Debug, Clone)]
pub struct PlayableDirector {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_playable_asset: PPtr,
    pub m_initial_state: i32,
    pub m_wrap_mode: i32,
    pub m_director_update_mode: i32,
    pub m_initial_time: f64,
    pub m_scene_bindings: Vec<DirectorGenericBinding>,
    pub m_exposed_references: ExposedReferenceTable,
}

impl crate::Parse for PlayableDirector {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_playable_asset = crate::PPtr::parse(r, ver)?;
        let m_initial_state = r.read_i32()?;
        let m_wrap_mode = r.read_i32()?;
        let m_director_update_mode = r.read_i32()?;
        let m_initial_time = r.read_f64()?;
        let m_scene_bindings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DirectorGenericBinding::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_exposed_references = ExposedReferenceTable::parse(r, ver)?;
        r.align(4)?;

        Ok(PlayableDirector {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_playable_asset,
            m_initial_state,
            m_wrap_mode,
            m_director_update_mode,
            m_initial_time,
            m_scene_bindings,
            m_exposed_references,
        })
    }
}
