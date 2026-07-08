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
    pub name: Option<String>,
    pub values: Vec<VfxMapping>,
    pub tasks: Vec<VfxEditorTaskDesc>,
}

impl crate::Parse for VfxEditorSystemDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut r#type = None;
        let mut flags = None;
        let mut capacity = None;
        let mut layer = None;
        let mut buffers = None;
        let mut name = None;
        let mut values = None;
        let mut tasks = None;
            r#type = Some(rdr.read_i32()?);
            flags = Some(rdr.read_i32()?);
            capacity = Some(rdr.read_u32()?);
            layer = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }
            buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            values = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            tasks = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEditorTaskDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;

        Ok(VfxEditorSystemDesc {
            r#type: r#type.unwrap(),
            flags: flags.unwrap(),
            capacity: capacity.unwrap(),
            layer: layer.unwrap(),
            buffers: buffers.unwrap(),
            name,
            values: values.unwrap(),
            tasks: tasks.unwrap(),
        })
    }
}
