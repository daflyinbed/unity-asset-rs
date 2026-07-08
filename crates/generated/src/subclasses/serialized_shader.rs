//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_custom_editor_for_render_pipeline::SerializedCustomEditorForRenderPipeline;
use crate::subclasses::serialized_properties::SerializedProperties;
use crate::subclasses::serialized_shader_dependency::SerializedShaderDependency;
use crate::subclasses::serialized_sub_shader::SerializedSubShader;

#[derive(Debug, Clone)]
pub struct SerializedShader {
    pub m_prop_info: SerializedProperties,
    pub m_sub_shaders: Vec<SerializedSubShader>,
    pub m_name: String,
    pub m_keyword_names: Option<Vec<String>>,
    pub m_custom_editor_name: String,
    pub m_keyword_flags: Option<Vec<u8>>,
    pub m_fallback_name: String,
    pub m_dependencies: Vec<SerializedShaderDependency>,
    pub m_disable_no_subshaders_message: bool,
    pub m_custom_editor_for_render_pipelines: Option<Vec<SerializedCustomEditorForRenderPipeline>>,
}

impl crate::Parse for SerializedShader {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_prop_info = None;
        let mut m_sub_shaders = None;
        let mut m_name = None;
        let mut m_keyword_names = None;
        let mut m_custom_editor_name = None;
        let mut m_keyword_flags = None;
        let mut m_fallback_name = None;
        let mut m_dependencies = None;
        let mut m_disable_no_subshaders_message = None;
        let mut m_custom_editor_for_render_pipelines = None;
            m_prop_info = Some(SerializedProperties::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sub_shaders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SerializedSubShader::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_keyword_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_keyword_flags = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_custom_editor_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_fallback_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_dependencies = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SerializedShaderDependency::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_custom_editor_for_render_pipelines = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SerializedCustomEditorForRenderPipeline::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_disable_no_subshaders_message = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(SerializedShader {
            m_prop_info: m_prop_info.unwrap(),
            m_sub_shaders: m_sub_shaders.unwrap(),
            m_name: m_name.unwrap(),
            m_keyword_names,
            m_custom_editor_name: m_custom_editor_name.unwrap(),
            m_keyword_flags,
            m_fallback_name: m_fallback_name.unwrap(),
            m_dependencies: m_dependencies.unwrap(),
            m_disable_no_subshaders_message: m_disable_no_subshaders_message.unwrap(),
            m_custom_editor_for_render_pipelines,
        })
    }
}
