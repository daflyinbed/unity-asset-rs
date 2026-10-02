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

impl crate::ToValue for MultiplayerManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_ActiveMultiplayerRole", &self.m_active_multiplayer_role),
            crate::opt_field("m_ActiveMultiplayerRoles", &self.m_active_multiplayer_roles),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ActiveMultiplayerRole" => {
                    (self.m_active_multiplayer_role).inspect_value(path, offset, limit)
                }
                "m_ActiveMultiplayerRoles" => {
                    (self.m_active_multiplayer_roles).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_active_multiplayer_role).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveMultiplayerRole".to_string(), v)),
            ((self.m_active_multiplayer_roles).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveMultiplayerRoles".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
