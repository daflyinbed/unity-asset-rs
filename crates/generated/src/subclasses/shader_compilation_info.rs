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

impl crate::ToValue for ShaderCompilationInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Snippets".to_string(), self.m_snippets.to_value())),
            Some((
                "m_MeshComponentsFromSnippets".to_string(),
                self.m_mesh_components_from_snippets.to_value(),
            )),
            Some((
                "m_HasSurfaceShaders".to_string(),
                self.m_has_surface_shaders.to_value(),
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
                "m_Snippets" => (self.m_snippets).inspect_value(path, offset, limit),
                "m_MeshComponentsFromSnippets" => {
                    (self.m_mesh_components_from_snippets).inspect_value(path, offset, limit)
                }
                "m_HasSurfaceShaders" => {
                    (self.m_has_surface_shaders).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_snippets).inspect_value(path, offset, limit))
                .map(|v| ("m_Snippets".to_string(), v)),
            ((self.m_mesh_components_from_snippets).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshComponentsFromSnippets".to_string(), v)),
            ((self.m_has_surface_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_HasSurfaceShaders".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
