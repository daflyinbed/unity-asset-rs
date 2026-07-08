//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AttachmentIndexArray {
    pub attachments: Vec<i32>,
    pub active_attachments: i32,
}

impl crate::Parse for AttachmentIndexArray {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let attachments = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let active_attachments = rdr.read_i32()?;

        Ok(AttachmentIndexArray {
            attachments,
            active_attachments,
        })
    }
}
