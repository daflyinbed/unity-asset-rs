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
        buffers = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        values = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        tasks = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxTaskDesc::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 1, 17) {
            instance_split_descs = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| VfxInstanceSplitDesc::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            instance_split_descs = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| VfxInstanceSplitDesc::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for VfxSystemDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("flags".to_string(), self.flags.to_value())),
            Some(("capacity".to_string(), self.capacity.to_value())),
            Some(("layer".to_string(), self.layer.to_value())),
            Some(("buffers".to_string(), self.buffers.to_value())),
            crate::opt_field("name", &self.name),
            Some(("values".to_string(), self.values.to_value())),
            Some(("tasks".to_string(), self.tasks.to_value())),
            crate::opt_field("instanceSplitDescs", &self.instance_split_descs),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "flags" => (self.flags).inspect_value(path, offset, limit),
                "capacity" => (self.capacity).inspect_value(path, offset, limit),
                "layer" => (self.layer).inspect_value(path, offset, limit),
                "buffers" => (self.buffers).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                "values" => (self.values).inspect_value(path, offset, limit),
                "tasks" => (self.tasks).inspect_value(path, offset, limit),
                "instanceSplitDescs" => {
                    (self.instance_split_descs).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
            ((self.capacity).inspect_value(path, offset, limit))
                .map(|v| ("capacity".to_string(), v)),
            ((self.layer).inspect_value(path, offset, limit)).map(|v| ("layer".to_string(), v)),
            ((self.buffers).inspect_value(path, offset, limit)).map(|v| ("buffers".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.values).inspect_value(path, offset, limit)).map(|v| ("values".to_string(), v)),
            ((self.tasks).inspect_value(path, offset, limit)).map(|v| ("tasks".to_string(), v)),
            ((self.instance_split_descs).inspect_value(path, offset, limit))
                .map(|v| ("instanceSplitDescs".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
