//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::editor_extension::EditorExtension;

#[derive(Debug, Clone)]
pub struct Component {
    pub parent: EditorExtension,
    pub m_game_object: PPtr,
}

impl crate::Parse for Component {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = EditorExtension::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;

        Ok(Component {
            parent,
            m_game_object,
        })
    }
}
