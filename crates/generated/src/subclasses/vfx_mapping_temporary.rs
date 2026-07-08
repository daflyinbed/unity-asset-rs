//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxMappingTemporary {
    pub mapping: VfxMapping,
    pub past_frame_index: u32,
    pub per_camera_buffer: bool,
}

impl crate::Parse for VfxMappingTemporary {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mapping = VfxMapping::parse(rdr, ver)?;
        rdr.align(4)?;
        let past_frame_index = rdr.read_u32()?;
        let per_camera_buffer = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxMappingTemporary {
            mapping,
            past_frame_index,
            per_camera_buffer,
        })
    }
}
