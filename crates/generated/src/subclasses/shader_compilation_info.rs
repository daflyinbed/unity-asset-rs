//! 自动生成。请勿手动修改。

use crate::subclasses::shader_snippet::ShaderSnippet;

#[derive(Debug, Clone)]
pub struct ShaderCompilationInfo {
    pub m_snippets: Vec<(i32, ShaderSnippet)>,
    pub m_mesh_components_from_snippets: u32,
    pub m_has_surface_shaders: bool,
}

impl crate::Parse for ShaderCompilationInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_snippets = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_i32()?;
                        let b = ShaderSnippet::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_mesh_components_from_snippets = rdr.read_u32()?;
        let m_has_surface_shaders = rdr.read_bool()?;

        Ok(ShaderCompilationInfo {
            m_snippets,
            m_mesh_components_from_snippets,
            m_has_surface_shaders,
        })
    }
}
