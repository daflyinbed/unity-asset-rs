//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxExposedMapping {
    pub mapping: VfxMapping,
    pub space: i32,
}

impl crate::Parse for VfxExposedMapping {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mapping = VfxMapping::parse(r, ver)?;
        r.align(4)?;
        let space = r.read_i32()?;

        Ok(VfxExposedMapping {
            mapping,
            space,
        })
    }
}
