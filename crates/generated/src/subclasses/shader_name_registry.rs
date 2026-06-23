//! 自动生成。请勿手动修改。

use crate::subclasses::name_to_object_map::NameToObjectMap;

#[derive(Debug, Clone)]
pub struct ShaderNameRegistry {
    pub m_object_hide_flags: u32,
    pub m_shaders: NameToObjectMap,
    pub m_preload_shaders: bool,
}

impl crate::Parse for ShaderNameRegistry {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_shaders = NameToObjectMap::parse(r, ver)?;
        r.align(4)?;
        let m_preload_shaders = r.read_bool()?;

        Ok(ShaderNameRegistry {
            m_object_hide_flags,
            m_shaders,
            m_preload_shaders,
        })
    }
}
