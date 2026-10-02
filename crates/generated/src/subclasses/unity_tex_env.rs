//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct UnityTexEnv {
    pub m_texture: PPtr,
    pub m_scale: Vector2f,
    pub m_offset: Vector2f,
}

impl crate::Parse for UnityTexEnv {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_texture = crate::PPtr::parse(rdr, ver)?;
        let m_scale = Vector2f::parse(rdr, ver)?;
        let m_offset = Vector2f::parse(rdr, ver)?;

        Ok(UnityTexEnv {
            m_texture,
            m_scale,
            m_offset,
        })
    }
}

impl crate::ToValue for UnityTexEnv {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Texture".to_string(), self.m_texture.to_value())),
            Some(("m_Scale".to_string(), self.m_scale.to_value())),
            Some(("m_Offset".to_string(), self.m_offset.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Texture" => (self.m_texture).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                "m_Offset" => (self.m_offset).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_Texture".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
            ((self.m_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_Offset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
