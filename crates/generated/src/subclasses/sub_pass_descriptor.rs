//! 自动生成。请勿手动修改。

use crate::subclasses::attachment_index_array::AttachmentIndexArray;

#[derive(Debug, Clone)]
pub struct SubPassDescriptor {
    pub inputs: AttachmentIndexArray,
    pub color_outputs: AttachmentIndexArray,
    pub flags: i32,
}

impl crate::Parse for SubPassDescriptor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let inputs = AttachmentIndexArray::parse(rdr, ver)?;
        let color_outputs = AttachmentIndexArray::parse(rdr, ver)?;
        let flags = rdr.read_i32()?;

        Ok(SubPassDescriptor {
            inputs,
            color_outputs,
            flags,
        })
    }
}

impl crate::ToValue for SubPassDescriptor {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("inputs".to_string(), self.inputs.to_value())),
            Some(("colorOutputs".to_string(), self.color_outputs.to_value())),
            Some(("flags".to_string(), self.flags.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "inputs" => (self.inputs).inspect_value(path, offset, limit),
                "colorOutputs" => (self.color_outputs).inspect_value(path, offset, limit),
                "flags" => (self.flags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.inputs).inspect_value(path, offset, limit)).map(|v| ("inputs".to_string(), v)),
            ((self.color_outputs).inspect_value(path, offset, limit))
                .map(|v| ("colorOutputs".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
