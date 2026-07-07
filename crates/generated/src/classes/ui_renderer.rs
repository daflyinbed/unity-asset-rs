//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct UiRenderer {
    pub m_game_object: PPtr,
    pub m_cull_transparent_mesh: Option<bool>,
}

impl crate::Parse for UiRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_cull_transparent_mesh = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(UiRenderer {
            m_game_object,
            m_cull_transparent_mesh,
        })
    }
}
