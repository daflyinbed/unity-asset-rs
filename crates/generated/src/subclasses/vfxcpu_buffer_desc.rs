//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_element_desc::VfxLayoutElementDesc;
use crate::subclasses::vfxcpu_buffer_data::VfxcpuBufferData;

#[derive(Debug, Clone)]
pub struct VfxcpuBufferDesc {
    pub capacity: u32,
    pub stride: u32,
    pub layout: Vec<VfxLayoutElementDesc>,
    pub initial_data: VfxcpuBufferData,
}

impl crate::Parse for VfxcpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let capacity = r.read_u32()?;
        let stride = r.read_u32()?;
        let layout = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxLayoutElementDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let initial_data = VfxcpuBufferData::parse(r, ver)?;
        r.align(4)?;

        Ok(VfxcpuBufferDesc {
            capacity,
            stride,
            layout,
            initial_data,
        })
    }
}
