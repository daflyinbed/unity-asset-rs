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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_infos = VisualEffectInfo::parse(rdr, ver)?;
        let m_systems = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxSystemDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(VisualEffectAsset {
            m_name,
            m_infos,
            m_systems,
        })
    }
}
