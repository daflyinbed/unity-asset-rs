//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_properties::SerializedProperties;
use crate::subclasses::serialized_shader_dependency::SerializedShaderDependency;
use crate::subclasses::serialized_sub_shader::SerializedSubShader;

#[derive(Debug, Clone)]
pub struct SerializedShader {
    pub m_prop_info: SerializedProperties,
    pub m_sub_shaders: Vec<SerializedSubShader>,
    pub m_name: String,
    pub m_custom_editor_name: String,
    pub m_fallback_name: String,
    pub m_dependencies: Vec<SerializedShaderDependency>,
    pub m_disable_no_subshaders_message: bool,
}

impl crate::Parse for SerializedShader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_prop_info = SerializedProperties::parse(r, ver)?;
        r.align(4)?;
        let m_sub_shaders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SerializedSubShader::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_custom_editor_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_fallback_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_dependencies = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SerializedShaderDependency::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_disable_no_subshaders_message = r.read_bool()?;
        r.align(4)?;

        Ok(SerializedShader {
            m_prop_info,
            m_sub_shaders,
            m_name,
            m_custom_editor_name,
            m_fallback_name,
            m_dependencies,
            m_disable_no_subshaders_message,
        })
    }
}
