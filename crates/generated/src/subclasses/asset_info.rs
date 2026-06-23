//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub preload_index: i32,
    pub preload_size: i32,
    pub asset: PPtr,
}

impl crate::Parse for AssetInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let preload_index = r.read_i32()?;
        let preload_size = r.read_i32()?;
        let asset = crate::PPtr::parse(r, ver)?;

        Ok(AssetInfo {
            preload_index,
            preload_size,
            asset,
        })
    }
}
