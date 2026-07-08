//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::component_pair::ComponentPair;


/// 版本枚举：GameObjectMComponentVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GameObjectMComponentVariant {
    Default(Vec<(i32, PPtr)>),
    V5_5(Vec<ComponentPair>),
    V2017_1(Vec<ComponentPair>),
}

impl crate::Parse for GameObjectMComponentVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComponentPair::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComponentPair::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_5(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GameObjectMIsActiveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GameObjectMIsActiveVariant {
    Default(u8),
    V4_0(bool),
}

impl crate::Parse for GameObjectMIsActiveVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = r.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct GameObject {
    pub m_component: GameObjectMComponentVariant,
    pub m_layer: u32,
    pub m_name: String,
    pub m_tag: u16,
    pub m_is_active: GameObjectMIsActiveVariant,
}

impl crate::Parse for GameObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_component = GameObjectMComponentVariant::parse(r, ver)?;
        let m_layer = r.read_u32()?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_tag = r.read_u16()?;
        let m_is_active = GameObjectMIsActiveVariant::parse(r, ver)?;

        Ok(GameObject {
            m_component,
            m_layer,
            m_name,
            m_tag,
            m_is_active,
        })
    }
}
