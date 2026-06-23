//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::director_generic_binding::DirectorGenericBinding;
use crate::subclasses::exposed_reference_table::ExposedReferenceTable;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct DirectorPlayer {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: Option<u8>,
    pub m_playable_asset: Option<PPtr>,
    pub m_initial_state: Option<i32>,
    pub m_wrap_mode: Option<i32>,
    pub m_director_update_mode: Option<i32>,
    pub m_initial_time: Option<f64>,
    pub m_scene_bindings: Option<Vec<DirectorGenericBinding>>,
    pub m_exposed_references: Option<ExposedReferenceTable>,
}

impl crate::Parse for DirectorPlayer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_playable_asset = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_initial_state = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_wrap_mode = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_director_update_mode = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_initial_time = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_f64()?)
        } else {
            None
        };
        let m_scene_bindings = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DirectorGenericBinding::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_exposed_references = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ExposedReferenceTable::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(DirectorPlayer {
            parent,
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
