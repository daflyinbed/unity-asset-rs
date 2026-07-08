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
            let v = ( { Ok(Box::new(UnitySceneHandle::parse(rdr, ver)?)) } )?;
            Ok(Self::V6000_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
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

        Ok(SceneIdentifier {
            guid,
            handle,
        })
    }
}
