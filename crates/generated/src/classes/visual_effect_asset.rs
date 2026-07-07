//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_system_desc::VfxSystemDesc;
use crate::subclasses::visual_effect_info::VisualEffectInfo;

#[derive(Debug, Clone)]
pub struct VisualEffectAsset {
    pub m_name: String,
    pub m_infos: VisualEffectInfo,
    pub m_systems: Vec<VfxSystemDesc>,
}

impl crate::Parse for VisualEffectAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_infos = VisualEffectInfo::parse(r, ver)?;
        r.align(4)?;
        let m_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxSystemDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VisualEffectAsset {
            m_name,
            m_infos,
            m_systems,
        })
    }
}
