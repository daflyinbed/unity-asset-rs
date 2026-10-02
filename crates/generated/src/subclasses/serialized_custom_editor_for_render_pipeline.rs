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

impl crate::ToValue for SerializedCustomEditorForRenderPipeline {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "customEditorName".to_string(),
                self.custom_editor_name.to_value(),
            )),
            Some((
                "renderPipelineType".to_string(),
                self.render_pipeline_type.to_value(),
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
                "customEditorName" => (self.custom_editor_name).inspect_value(path, offset, limit),
                "renderPipelineType" => {
                    (self.render_pipeline_type).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.custom_editor_name).inspect_value(path, offset, limit))
                .map(|v| ("customEditorName".to_string(), v)),
            ((self.render_pipeline_type).inspect_value(path, offset, limit))
                .map(|v| ("renderPipelineType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
