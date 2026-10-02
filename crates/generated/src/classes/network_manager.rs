//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct NetworkManager {
    pub m_debug_level: i32,
    pub m_sendrate: f32,
    pub m_asset_to_prefab: Vec<(Guid, PPtr)>,
}

impl crate::Parse for NetworkManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_debug_level = rdr.read_i32()?;
        let m_sendrate = rdr.read_f32()?;
        let m_asset_to_prefab = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Guid::parse(rdr, ver)?;
                        let b = crate::PPtr::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(NetworkManager {
            m_debug_level,
            m_sendrate,
            m_asset_to_prefab,
        })
    }
}

impl crate::ToValue for NetworkManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_DebugLevel".to_string(), self.m_debug_level.to_value())),
            Some(("m_Sendrate".to_string(), self.m_sendrate.to_value())),
            Some((
                "m_AssetToPrefab".to_string(),
                self.m_asset_to_prefab.to_value(),
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
                "m_DebugLevel" => (self.m_debug_level).inspect_value(path, offset, limit),
                "m_Sendrate" => (self.m_sendrate).inspect_value(path, offset, limit),
                "m_AssetToPrefab" => (self.m_asset_to_prefab).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_debug_level).inspect_value(path, offset, limit))
                .map(|v| ("m_DebugLevel".to_string(), v)),
            ((self.m_sendrate).inspect_value(path, offset, limit))
                .map(|v| ("m_Sendrate".to_string(), v)),
            ((self.m_asset_to_prefab).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetToPrefab".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
