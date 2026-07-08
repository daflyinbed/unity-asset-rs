//! 自动生成。请勿手动修改。

use crate::subclasses::name_to_object_map::NameToObjectMap;

#[derive(Debug, Clone)]
pub struct ScriptMapper {
    pub m_shaders: NameToObjectMap,
    pub m_preload_shaders: Option<bool>,
}

impl crate::Parse for ScriptMapper {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_shaders = NameToObjectMap::parse(rdr, ver)?;
        rdr.align(4)?;
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
