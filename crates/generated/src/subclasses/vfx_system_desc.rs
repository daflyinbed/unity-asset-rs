//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_instance_split_desc::VfxInstanceSplitDesc;
use crate::subclasses::vfx_mapping::VfxMapping;
use crate::subclasses::vfx_task_desc::VfxTaskDesc;

#[derive(Debug, Clone)]
pub struct VfxSystemDesc {
    pub r#type: i32,
    pub flags: i32,
    pub capacity: u32,
    pub layer: u32,
    pub buffers: Vec<VfxMapping>,
    pub name: Option<String>,
    pub values: Vec<VfxMapping>,
    pub tasks: Vec<VfxTaskDesc>,
    pub instance_split_descs: Option<Vec<VfxInstanceSplitDesc>>,
}

impl crate::Parse for VfxSystemDesc {
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
        let mut instance_split_descs = None;
            r#type = Some(rdr.read_i32()?);
            flags = Some(rdr.read_i32()?);
            capacity = Some(rdr.read_u32()?);
            layer = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            name = Some(rdr.read_aligned_string()?);
        } else {
        }
            buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            values = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            tasks = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxTaskDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 1, 17) {
            instance_split_descs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxInstanceSplitDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            instance_split_descs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxInstanceSplitDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }

        Ok(VfxSystemDesc {
            r#type: r#type.unwrap(),
            flags: flags.unwrap(),
            capacity: capacity.unwrap(),
            layer: layer.unwrap(),
            buffers: buffers.unwrap(),
            name,
            values: values.unwrap(),
            tasks: tasks.unwrap(),
            instance_split_descs,
        })
    }
}
