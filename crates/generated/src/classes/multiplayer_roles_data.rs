//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::object_role_pair::ObjectRolePair;

#[derive(Debug, Clone)]
pub struct MultiplayerRolesData {
    pub m_game_object: PPtr,
    pub m_game_object_roles_mask: i32,
    pub m_components_roles_masks: Vec<ObjectRolePair>,
}

impl crate::Parse for MultiplayerRolesData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_game_object_roles_mask = rdr.read_i32()?;
        let m_components_roles_masks = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ObjectRolePair::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(MultiplayerRolesData {
            m_game_object,
            m_game_object_roles_mask,
            m_components_roles_masks,
        })
    }
}

impl crate::ToValue for MultiplayerRolesData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "m_GameObjectRolesMask".to_string(),
                self.m_game_object_roles_mask.to_value(),
            )),
            Some((
                "m_ComponentsRolesMasks".to_string(),
                self.m_components_roles_masks.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_GameObjectRolesMask" => {
                    (self.m_game_object_roles_mask).inspect_value(path, offset, limit)
                }
                "m_ComponentsRolesMasks" => {
                    (self.m_components_roles_masks).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_game_object_roles_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObjectRolesMask".to_string(), v)),
            ((self.m_components_roles_masks).inspect_value(path, offset, limit))
                .map(|v| ("m_ComponentsRolesMasks".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
