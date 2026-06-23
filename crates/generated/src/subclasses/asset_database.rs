//! 自动生成。请勿手动修改。

use crate::subclasses::asset::Asset;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AssetDatabase {
    pub m_object_hide_flags: u32,
    pub m_assets: Vec<(Guid, Asset)>,
}

impl crate::Parse for AssetDatabase {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_assets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = Asset::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AssetDatabase {
            m_object_hide_flags,
            m_assets,
        })
    }
}
