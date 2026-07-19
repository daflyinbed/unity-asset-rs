//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_pass::SerializedPass;
use crate::subclasses::serialized_tag_map::SerializedTagMap;

/// 版本枚举：SerializedSubShaderMPassesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubShaderMPassesVariant {
    Default(Vec<SerializedPass>),
    V2017_1(Vec<SerializedPass>),
}

impl crate::Parse for SerializedSubShaderMPassesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SerializedPass::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SerializedPass::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedSubShader {
    pub m_passes: SerializedSubShaderMPassesVariant,
    pub m_tags: SerializedTagMap,
    pub m_lod: i32,
}

impl crate::Parse for SerializedSubShader {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_passes = SerializedSubShaderMPassesVariant::parse(rdr, ver)?;
        let m_tags = SerializedTagMap::parse(rdr, ver)?;
        let m_lod = rdr.read_i32()?;

        Ok(SerializedSubShader {
            m_passes,
            m_tags,
            m_lod,
        })
    }
}
