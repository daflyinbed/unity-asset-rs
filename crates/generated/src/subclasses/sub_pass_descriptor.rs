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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let inputs = AttachmentIndexArray::parse(r, ver)?;
        r.align(4)?;
        let color_outputs = AttachmentIndexArray::parse(r, ver)?;
        r.align(4)?;
        let flags = r.read_i32()?;

        Ok(SubPassDescriptor {
            inputs,
            color_outputs,
            flags,
        })
    }
}
