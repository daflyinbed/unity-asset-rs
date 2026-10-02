//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_mapping::VfxMapping;
use crate::subclasses::vfx_mapping_temporary::VfxMappingTemporary;

#[derive(Debug, Clone)]
pub struct VfxTaskDesc {
    pub r#type: i32,
    pub buffers: Vec<VfxMapping>,
    pub values: Vec<VfxMapping>,
    pub temporary_buffers: Option<Vec<VfxMappingTemporary>>,
    pub params: Vec<VfxMapping>,
    pub processor: PPtr,
    pub instance_split_index: Option<u32>,
}

impl crate::Parse for VfxTaskDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut r#type = None;
        let mut buffers = None;
        let mut values = None;
        let mut temporary_buffers = None;
        let mut params = None;
        let mut processor = None;
        let mut instance_split_index = None;
        r#type = Some(rdr.read_i32()?);
        buffers = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            temporary_buffers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| VfxMappingTemporary::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        values = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        params = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        processor = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 17) {
            instance_split_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            instance_split_index = Some(rdr.read_u32()?);
        } else {
        }

        Ok(VfxTaskDesc {
            r#type: r#type.unwrap(),
            buffers: buffers.unwrap(),
            values: values.unwrap(),
            temporary_buffers,
            params: params.unwrap(),
            processor: processor.unwrap(),
            instance_split_index,
        })
    }
}

impl crate::ToValue for VfxTaskDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("buffers".to_string(), self.buffers.to_value())),
            Some(("values".to_string(), self.values.to_value())),
            crate::opt_field("temporaryBuffers", &self.temporary_buffers),
            Some(("params".to_string(), self.params.to_value())),
            Some(("processor".to_string(), self.processor.to_value())),
            crate::opt_field("instanceSplitIndex", &self.instance_split_index),
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
                "buffers" => (self.buffers).inspect_value(path, offset, limit),
                "values" => (self.values).inspect_value(path, offset, limit),
                "temporaryBuffers" => (self.temporary_buffers).inspect_value(path, offset, limit),
                "params" => (self.params).inspect_value(path, offset, limit),
                "processor" => (self.processor).inspect_value(path, offset, limit),
                "instanceSplitIndex" => {
                    (self.instance_split_index).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.buffers).inspect_value(path, offset, limit)).map(|v| ("buffers".to_string(), v)),
            ((self.values).inspect_value(path, offset, limit)).map(|v| ("values".to_string(), v)),
            ((self.temporary_buffers).inspect_value(path, offset, limit))
                .map(|v| ("temporaryBuffers".to_string(), v)),
            ((self.params).inspect_value(path, offset, limit)).map(|v| ("params".to_string(), v)),
            ((self.processor).inspect_value(path, offset, limit))
                .map(|v| ("processor".to_string(), v)),
            ((self.instance_split_index).inspect_value(path, offset, limit))
                .map(|v| ("instanceSplitIndex".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
