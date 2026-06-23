//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_element_desc::VfxLayoutElementDesc;

#[derive(Debug, Clone)]
pub struct VfxgpuBufferDesc {
    pub r#type: i32,
    pub size: u32,
    pub layout: Vec<VfxLayoutElementDesc>,
    pub capacity: u32,
    pub stride: u32,
}

impl crate::Parse for VfxgpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = r.read_i32()?;
        let size = r.read_u32()?;
        let layout = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxLayoutElementDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let capacity = r.read_u32()?;
        let stride = r.read_u32()?;

        Ok(VfxgpuBufferDesc {
            r#type,
            size,
            layout,
            capacity,
            stride,
        })
    }
}
