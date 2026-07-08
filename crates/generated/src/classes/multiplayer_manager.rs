//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MultiplayerManager {
    pub m_active_multiplayer_role: Option<i32>,
    pub m_active_multiplayer_roles: Option<i32>,
}

impl crate::Parse for MultiplayerManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_active_multiplayer_role = None;
        let mut m_active_multiplayer_roles = None;
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_active_multiplayer_roles = Some(rdr.read_i32()?);
        } else {
            m_active_multiplayer_role = Some(rdr.read_i32()?);
        }

        Ok(MultiplayerManager {
            m_active_multiplayer_role,
            m_active_multiplayer_roles,
        })
    }
}
