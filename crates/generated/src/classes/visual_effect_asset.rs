//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_system_desc::VfxSystemDesc;
use crate::subclasses::visual_effect_info::VisualEffectInfo;
use super::named_object::NamedObject;
use super::visual_effect_object::VisualEffectObject;

#[derive(Debug, Clone)]
pub enum VisualEffectAssetParent {
    NamedObject(NamedObject),
    VisualEffectObject(VisualEffectObject),
}

#[derive(Debug, Clone)]
pub struct VisualEffectAsset {
    pub parent: VisualEffectAssetParent,
    pub m_name: String,
    pub m_infos: VisualEffectInfo,
    pub m_systems: Vec<VfxSystemDesc>,
}

impl crate::Parse for VisualEffectAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            VisualEffectAssetParent::NamedObject(NamedObject::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            VisualEffectAssetParent::VisualEffectObject(VisualEffectObject::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_infos = VisualEffectInfo::parse(r, ver)?;
        r.align(4)?;
        let m_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxSystemDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VisualEffectAsset {
            parent,
            m_name,
            m_infos,
            m_systems,
        })
    }
}
