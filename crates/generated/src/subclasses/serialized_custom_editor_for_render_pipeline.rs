//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedCustomEditorForRenderPipeline {
    pub custom_editor_name: String,
    pub render_pipeline_type: String,
}

impl crate::Parse for SerializedCustomEditorForRenderPipeline {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let custom_editor_name = r.read_aligned_string()?;
        r.align(4)?;
        let render_pipeline_type = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SerializedCustomEditorForRenderPipeline {
            custom_editor_name,
            render_pipeline_type,
        })
    }
}
