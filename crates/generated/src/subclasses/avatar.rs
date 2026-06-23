//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::avatar_constant::AvatarConstant;

#[derive(Debug, Clone)]
pub struct Avatar {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_avatar_size: u32,
    pub m_avatar: AvatarConstant,
    pub m_tos: Vec<(u32, String)>,
}

impl crate::Parse for Avatar {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_avatar_size = r.read_u32()?;
        let m_avatar = AvatarConstant::parse(r, ver)?;
        r.align(4)?;
        let m_tos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_u32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(Avatar {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_avatar_size,
            m_avatar,
            m_tos,
        })
    }
}
