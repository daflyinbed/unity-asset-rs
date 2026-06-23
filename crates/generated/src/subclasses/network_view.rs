//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::network_view_id::NetworkViewId;

#[derive(Debug, Clone)]
pub struct NetworkView {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_state_synchronization: i32,
    pub m_observed: PPtr,
    pub m_view_id: NetworkViewId,
}

impl crate::Parse for NetworkView {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_state_synchronization = r.read_i32()?;
        let m_observed = crate::PPtr::parse(r, ver)?;
        let m_view_id = NetworkViewId::parse(r, ver)?;

        Ok(NetworkView {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_state_synchronization,
            m_observed,
            m_view_id,
        })
    }
}
