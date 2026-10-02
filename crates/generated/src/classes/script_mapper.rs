//! 自动生成。请勿手动修改。

use crate::subclasses::name_to_object_map_shader::NameToObjectMap_Shader;

#[derive(Debug, Clone)]
pub struct ScriptMapper {
    pub m_shaders: NameToObjectMap_Shader,
    pub m_preload_shaders: Option<bool>,
}

impl crate::Parse for ScriptMapper {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_shaders = NameToObjectMap_Shader::parse(rdr, ver)?;
        let m_preload_shaders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(ScriptMapper {
            m_shaders,
            m_preload_shaders,
        })
    }
}

impl crate::ToValue for ScriptMapper {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Shaders".to_string(), self.m_shaders.to_value())),
            crate::opt_field("m_PreloadShaders", &self.m_preload_shaders),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Shaders" => (self.m_shaders).inspect_value(path, offset, limit),
                "m_PreloadShaders" => (self.m_preload_shaders).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_Shaders".to_string(), v)),
            ((self.m_preload_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadShaders".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
