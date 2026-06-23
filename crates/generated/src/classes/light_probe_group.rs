//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::behaviour::Behaviour;
use super::component::Component;

#[derive(Debug, Clone)]
pub enum LightProbeGroupParent {
    Component(Component),
    Behaviour(Behaviour),
}

#[derive(Debug, Clone)]
pub struct LightProbeGroup {
    pub parent: LightProbeGroupParent,
    pub m_game_object: PPtr,
    pub m_enabled: Option<u8>,
}

impl crate::Parse for LightProbeGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            LightProbeGroupParent::Component(Component::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            LightProbeGroupParent::Behaviour(Behaviour::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(LightProbeGroup {
            parent,
            m_game_object,
            m_enabled,
        })
    }
}
