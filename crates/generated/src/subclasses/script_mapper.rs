//! 自动生成。请勿手动修改。

use crate::subclasses::name_to_object_map::NameToObjectMap;

#[derive(Debug, Clone)]
pub struct ScriptMapper {
    pub m_object_hide_flags: u32,
    pub m_shaders: NameToObjectMap,
    pub m_shader_includes: NameToObjectMap,
}

impl crate::Parse for ScriptMapper {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_shaders = NameToObjectMap::parse(r, ver)?;
        r.align(4)?;
        let m_shader_includes = NameToObjectMap::parse(r, ver)?;
        r.align(4)?;

        Ok(ScriptMapper {
            m_object_hide_flags,
            m_shaders,
            m_shader_includes,
        })
    }
}
