//! 自动生成。请勿手动修改。

use super::editor_extension::EditorExtension;

#[derive(Debug, Clone)]
pub struct NamedObject {
    pub parent: EditorExtension,
}

impl crate::Parse for NamedObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = EditorExtension::parse(r, ver)?;

        Ok(NamedObject {
            parent,
        })
    }
}
