//! 自动生成。请勿手动修改。

use crate::subclasses::component_pair::ComponentPair;


/// 版本枚举：GameObjectMComponentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GameObjectMComponentsVariant {
    Default(Vec<ComponentPair>),
    V2017_1(Vec<ComponentPair>),
}

impl crate::Parse for GameObjectMComponentsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComponentPair::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComponentPair::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct GameObject {
    pub m_components: GameObjectMComponentsVariant,
    pub m_layer: u32,
    pub m_name: String,
    pub m_tag: u16,
    pub m_is_active: GameObjectMIsActiveVariant,
}

impl crate::Parse for GameObject {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_components = GameObjectMComponentsVariant::parse(rdr, ver)?;
        let m_layer = rdr.read_u32()?;
        let m_name = rdr.read_aligned_string()?;
        let m_tag = rdr.read_u16()?;
        let m_is_active = GameObjectMIsActiveVariant::parse(rdr, ver)?;

        Ok(GameObject {
            m_components,
            m_layer,
            m_name,
            m_tag,
            m_is_active,
        })
    }
}
