//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use super::level_game_manager::LevelGameManager;

#[derive(Debug, Clone)]
pub struct Scene {
    pub parent: LevelGameManager,
    pub m_pvs_data: Option<Vec<u8>>,
    pub m_scene_guid: Option<Guid>,
    pub m_pvs_objects_array: Option<Vec<PPtr>>,
    pub m_query_mode: Option<u32>,
    pub m_occlusion_culling_data: Option<PPtr>,
    pub m_pvs_portals_array: Option<Vec<PPtr>>,
    pub m_static_renderers: Option<Vec<PPtr>>,
    pub m_portals: Option<Vec<PPtr>>,
}

impl crate::Parse for Scene {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = LevelGameManager::parse(r, ver)?;
        let m_pvs_data = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_scene_guid = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let m_pvs_objects_array = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_query_mode = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_occlusion_culling_data = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_pvs_portals_array = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_static_renderers = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_portals = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(Scene {
            parent,
            m_pvs_data,
            m_scene_guid,
            m_pvs_objects_array,
            m_query_mode,
            m_occlusion_culling_data,
            m_pvs_portals_array,
            m_static_renderers,
            m_portals,
        })
    }
}
