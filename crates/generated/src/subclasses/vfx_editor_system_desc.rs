//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_editor_task_desc::VfxEditorTaskDesc;
use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxEditorSystemDesc {
    pub r#type: i32,
    pub flags: i32,
    pub capacity: u32,
    pub layer: u32,
    pub buffers: Vec<VfxMapping>,
    pub values: Vec<VfxMapping>,
    pub tasks: Vec<VfxEditorTaskDesc>,
}

impl crate::Parse for VfxEditorSystemDesc {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = r.read_i32()?;
        let flags = r.read_i32()?;
        let capacity = r.read_u32()?;
        let layer = r.read_u32()?;
        let buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let tasks = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxEditorTaskDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VfxEditorSystemDesc {
            r#type,
            flags,
            capacity,
            layer,
            buffers,
            values,
            tasks,
        })
    }
}
