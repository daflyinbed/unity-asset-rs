//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::speed_tree_wind::SpeedTreeWind;

#[derive(Debug, Clone)]
pub struct SpeedTreeWindAsset {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_wind: SpeedTreeWind,
}

impl crate::Parse for SpeedTreeWindAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_wind = SpeedTreeWind::parse(r, ver)?;

        Ok(SpeedTreeWindAsset {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_wind,
        })
    }
}
