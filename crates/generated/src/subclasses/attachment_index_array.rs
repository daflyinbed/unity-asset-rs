//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AttachmentIndexArray {
    pub attachments: Vec<i32>,
    pub active_attachments: i32,
}

impl crate::Parse for AttachmentIndexArray {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let attachments = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let active_attachments = r.read_i32()?;

        Ok(AttachmentIndexArray {
            attachments,
            active_attachments,
        })
    }
}
