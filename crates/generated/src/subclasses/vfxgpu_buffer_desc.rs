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
        let mut r#type = None;
        let mut target = None;
        let mut size = None;
        let mut layout = None;
        let mut capacity = None;
        let mut stride = None;
        let mut mode = None;
        let mut debug_name = None;
        if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            target = Some(rdr.read_i32()?);
        } else {
            r#type = Some(rdr.read_i32()?);
        }
            size = Some(rdr.read_u32()?);
            layout = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxLayoutElementDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            capacity = Some(rdr.read_u32()?);
            stride = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 4) {
            mode = Some(rdr.read_i32()?);
            debug_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 14) {
            mode = Some(rdr.read_i32()?);
            debug_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            mode = Some(rdr.read_i32()?);
        } else {
        }

        Ok(VfxgpuBufferDesc {
            r#type,
            target,
            size: size.unwrap(),
            layout: layout.unwrap(),
            capacity: capacity.unwrap(),
            stride: stride.unwrap(),
            mode,
            debug_name,
        })
    }
}
