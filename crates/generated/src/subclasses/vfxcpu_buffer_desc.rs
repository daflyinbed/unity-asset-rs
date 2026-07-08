//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_element_desc::VfxLayoutElementDesc;
use crate::subclasses::vfxcpu_buffer_data::VfxcpuBufferData;

#[derive(Debug, Clone)]
pub struct VfxcpuBufferDesc {
    pub capacity: u32,
    pub stride: u32,
    pub layout: Vec<VfxLayoutElementDesc>,
    pub initial_data: VfxcpuBufferData,
    pub debug_name: Option<String>,
}

impl crate::Parse for VfxcpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let capacity = rdr.read_u32()?;
        let stride = rdr.read_u32()?;
        let layout = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxLayoutElementDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let initial_data = VfxcpuBufferData::parse(rdr, ver)?;
        rdr.align(4)?;
        let debug_name = if *ver >= crate::UnityVersion::new(6000, 3, 14) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VfxcpuBufferDesc {
            capacity,
            stride,
            layout,
            initial_data,
            debug_name,
        })
    }
}
