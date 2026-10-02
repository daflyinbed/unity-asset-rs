//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::unity_scene_handle::UnitySceneHandle;

/// 版本枚举：SceneIdentifierHandleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SceneIdentifierHandleVariant {
    Default(i32),
    V6000_3(Box<UnitySceneHandle>),
}

impl crate::Parse for SceneIdentifierHandleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = ({ Ok(Box::new(UnitySceneHandle::parse(rdr, ver)?)) })?;
            Ok(Self::V6000_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SceneIdentifierHandleVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SceneIdentifier {
    pub guid: Guid,
    pub handle: SceneIdentifierHandleVariant,
}

impl crate::Parse for SceneIdentifier {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(rdr, ver)?;
        let handle = SceneIdentifierHandleVariant::parse(rdr, ver)?;

        Ok(SceneIdentifier { guid, handle })
    }
}

impl crate::ToValue for SceneIdentifier {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("handle".to_string(), self.handle.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "handle" => (self.handle).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.handle).inspect_value(path, offset, limit)).map(|v| ("handle".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
