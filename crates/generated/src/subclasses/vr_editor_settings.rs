//! 自动生成。请勿手动修改。

use crate::subclasses::none::None;

#[derive(Debug, Clone)]
pub struct VrEditorSettings {
    pub none: None,
}

impl crate::Parse for VrEditorSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let none = None::parse(r, ver)?;
        r.align(4)?;

        Ok(VrEditorSettings {
            none,
        })
    }
}
