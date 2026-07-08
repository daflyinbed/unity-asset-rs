//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_element_desc::VfxLayoutElementDesc;

#[derive(Debug, Clone)]
pub struct VfxgpuBufferDesc {
    pub r#type: Option<i32>,
    pub target: Option<i32>,
    pub size: u32,
    pub layout: Vec<VfxLayoutElementDesc>,
    pub capacity: u32,
    pub stride: u32,
    pub mode: Option<i32>,
    pub debug_name: Option<String>,
}

impl crate::Parse for VfxgpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let target = if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let size = rdr.read_u32()?;
        let layout = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxLayoutElementDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let capacity = rdr.read_u32()?;
        let stride = rdr.read_u32()?;
        let mode = if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let debug_name = if *ver >= crate::UnityVersion::new(6000, 3, 14) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VfxgpuBufferDesc {
            r#type,
            target,
            size,
            layout,
            capacity,
            stride,
            mode,
            debug_name,
        })
    }
}
