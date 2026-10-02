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
        let attachments = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let active_attachments = rdr.read_i32()?;

        Ok(AttachmentIndexArray {
            attachments,
            active_attachments,
        })
    }
}

impl crate::ToValue for AttachmentIndexArray {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("attachments".to_string(), self.attachments.to_value())),
            Some((
                "activeAttachments".to_string(),
                self.active_attachments.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "attachments" => (self.attachments).inspect_value(path, offset, limit),
                "activeAttachments" => (self.active_attachments).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.attachments).inspect_value(path, offset, limit))
                .map(|v| ("attachments".to_string(), v)),
            ((self.active_attachments).inspect_value(path, offset, limit))
                .map(|v| ("activeAttachments".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
