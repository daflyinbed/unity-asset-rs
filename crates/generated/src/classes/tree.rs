//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Tree {
    pub m_game_object: PPtr,
    pub m_speed_tree_wind_asset: Option<PPtr>,
}

impl crate::Parse for Tree {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_speed_tree_wind_asset = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };

        Ok(Tree {
            m_game_object,
            m_speed_tree_wind_asset,
        })
    }
}
