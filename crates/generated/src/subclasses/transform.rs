//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Transform {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_local_rotation: Quaternionf,
    pub m_local_position: Vector3f,
    pub m_local_scale: Vector3f,
    pub m_children: Vec<PPtr>,
    pub m_father: PPtr,
}

impl crate::Parse for Transform {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_local_rotation = Quaternionf::parse(r, ver)?;
        let m_local_position = Vector3f::parse(r, ver)?;
        let m_local_scale = Vector3f::parse(r, ver)?;
        let m_children = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_father = crate::PPtr::parse(r, ver)?;

        Ok(Transform {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_local_rotation,
            m_local_position,
            m_local_scale,
            m_children,
            m_father,
        })
    }
}
