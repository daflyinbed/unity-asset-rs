//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_pass::SerializedPass;
use crate::subclasses::serialized_tag_map::SerializedTagMap;

#[derive(Debug, Clone)]
pub struct SerializedSubShader {
    pub m_passes: Vec<SerializedPass>,
    pub m_tags: SerializedTagMap,
    pub m_lod: i32,
}

impl crate::Parse for SerializedSubShader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_passes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SerializedPass::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_tags = SerializedTagMap::parse(r, ver)?;
        r.align(4)?;
        let m_lod = r.read_i32()?;

        Ok(SerializedSubShader {
            m_passes,
            m_tags,
            m_lod,
        })
    }
}
