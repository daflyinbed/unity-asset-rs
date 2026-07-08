//! 自动生成。请勿手动修改。

use crate::subclasses::vfxgpu_buffer_desc::VfxgpuBufferDesc;

#[derive(Debug, Clone)]
pub struct VfxTemporaryGpuBufferDesc {
    pub desc: VfxgpuBufferDesc,
    pub frame_count: u32,
}

impl crate::Parse for VfxTemporaryGpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let desc = VfxgpuBufferDesc::parse(rdr, ver)?;
        let frame_count = rdr.read_u32()?;

        Ok(VfxTemporaryGpuBufferDesc {
            desc,
            frame_count,
        })
    }
}
