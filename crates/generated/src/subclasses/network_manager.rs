//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct NetworkManager {
    pub m_object_hide_flags: u32,
    pub m_debug_level: i32,
    pub m_sendrate: f32,
    pub m_asset_to_prefab: Vec<(Guid, PPtr)>,
}

impl crate::Parse for NetworkManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_debug_level = r.read_i32()?;
        let m_sendrate = r.read_f32()?;
        let m_asset_to_prefab = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(NetworkManager {
            m_object_hide_flags,
            m_debug_level,
            m_sendrate,
            m_asset_to_prefab,
        })
    }
}
