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
        let mut capacity = None;
        let mut stride = None;
        let mut layout = None;
        let mut initial_data = None;
        let mut debug_name = None;
            capacity = Some(rdr.read_u32()?);
            stride = Some(rdr.read_u32()?);
            layout = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxLayoutElementDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            initial_data = Some(VfxcpuBufferData::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
        } else if *ver >= crate::UnityVersion::new(6000, 4, 4) {
            debug_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
        } else if *ver >= crate::UnityVersion::new(6000, 3, 14) {
            debug_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }

        Ok(VfxcpuBufferDesc {
            capacity: capacity.unwrap(),
            stride: stride.unwrap(),
            layout: layout.unwrap(),
            initial_data: initial_data.unwrap(),
            debug_name,
        })
    }
}
