//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::lod::Lod;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct LodGroup {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_local_reference_point: Vector3f,
    pub m_size: f32,
    pub m_screen_relative_transition_height: f32,
    pub m_lo_ds: Vec<Lod>,
    pub m_enabled: bool,
}

impl crate::Parse for LodGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_local_reference_point = Vector3f::parse(r, ver)?;
        let m_size = r.read_f32()?;
        let m_screen_relative_transition_height = r.read_f32()?;
        let m_lo_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Lod::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_enabled = r.read_bool()?;

        Ok(LodGroup {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_local_reference_point,
            m_size,
            m_screen_relative_transition_height,
            m_lo_ds,
            m_enabled,
        })
    }
}
