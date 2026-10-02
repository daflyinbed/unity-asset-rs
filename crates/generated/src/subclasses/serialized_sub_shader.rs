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

impl crate::ToValue for SerializedSubShaderMPassesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SerializedSubShader {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Passes".to_string(), self.m_passes.to_value())),
            Some(("m_Tags".to_string(), self.m_tags.to_value())),
            Some(("m_LOD".to_string(), self.m_lod.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Passes" => (self.m_passes).inspect_value(path, offset, limit),
                "m_Tags" => (self.m_tags).inspect_value(path, offset, limit),
                "m_LOD" => (self.m_lod).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_passes).inspect_value(path, offset, limit))
                .map(|v| ("m_Passes".to_string(), v)),
            ((self.m_tags).inspect_value(path, offset, limit)).map(|v| ("m_Tags".to_string(), v)),
            ((self.m_lod).inspect_value(path, offset, limit)).map(|v| ("m_LOD".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
