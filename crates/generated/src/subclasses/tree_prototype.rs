//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TreePrototype {
    pub prefab: PPtr,
    pub bend_factor: f32,
}

impl crate::Parse for TreePrototype {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let prefab = crate::PPtr::parse(r, ver)?;
        let bend_factor = r.read_f32()?;

        Ok(TreePrototype {
            prefab,
            bend_factor,
        })
    }
}
