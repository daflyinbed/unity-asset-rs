//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedCustomEditorForRenderPipeline {
    pub custom_editor_name: String,
    pub render_pipeline_type: String,
}

impl crate::Parse for SerializedCustomEditorForRenderPipeline {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let custom_editor_name = rdr.read_aligned_string()?;
        let render_pipeline_type = rdr.read_aligned_string()?;

        Ok(SerializedCustomEditorForRenderPipeline {
            custom_editor_name,
            render_pipeline_type,
        })
    }
}
