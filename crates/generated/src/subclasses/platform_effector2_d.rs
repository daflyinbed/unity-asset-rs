//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct PlatformEffector2D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_one_way: bool,
    pub m_side_friction: bool,
    pub m_side_bounce: bool,
    pub m_side_angle_variance: f32,
}

impl crate::Parse for PlatformEffector2D {
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
        let m_collider_mask = BitField::parse(r, ver)?;
        let m_one_way = r.read_bool()?;
        let m_side_friction = r.read_bool()?;
        let m_side_bounce = r.read_bool()?;
        r.align(4)?;
        let m_side_angle_variance = r.read_f32()?;

        Ok(PlatformEffector2D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_collider_mask,
            m_one_way,
            m_side_friction,
            m_side_bounce,
            m_side_angle_variance,
        })
    }
}
