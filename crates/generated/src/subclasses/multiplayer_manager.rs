//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MultiplayerManager {
    pub m_object_hide_flags: u32,
    pub m_enable_multiplayer_roles: bool,
    pub m_active_multiplayer_role: i32,
}

impl crate::Parse for MultiplayerManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_enable_multiplayer_roles = r.read_bool()?;
        r.align(4)?;
        let m_active_multiplayer_role = r.read_i32()?;

        Ok(MultiplayerManager {
            m_object_hide_flags,
            m_enable_multiplayer_roles,
            m_active_multiplayer_role,
        })
    }
}
