//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct SceneIdentifier {
    pub guid: Guid,
    pub handle: i32,
}

impl crate::Parse for SceneIdentifier {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(r, ver)?;
        let handle = r.read_i32()?;

        Ok(SceneIdentifier {
            guid,
            handle,
        })
    }
}
