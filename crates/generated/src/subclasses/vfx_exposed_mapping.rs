//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxExposedMapping {
    pub mapping: VfxMapping,
    pub space: i32,
}

impl crate::Parse for VfxExposedMapping {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mapping = VfxMapping::parse(rdr, ver)?;
        rdr.align(4)?;
        let space = rdr.read_i32()?;

        Ok(VfxExposedMapping {
            mapping,
            space,
        })
    }
}
