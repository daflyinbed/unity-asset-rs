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
