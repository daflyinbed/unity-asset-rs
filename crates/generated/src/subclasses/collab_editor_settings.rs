//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CollabEditorSettings {
    pub in_progress_enabled: bool,
}

impl crate::Parse for CollabEditorSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let in_progress_enabled = r.read_bool()?;
        r.align(4)?;

        Ok(CollabEditorSettings {
            in_progress_enabled,
        })
    }
}
