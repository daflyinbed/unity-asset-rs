//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_renderer_settings::VfxRendererSettings;
use crate::subclasses::vfx_system_desc::VfxSystemDesc;
use crate::subclasses::visual_effect_info::VisualEffectInfo;

#[derive(Debug, Clone)]
pub struct VisualEffectAsset {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_infos: VisualEffectInfo,
    pub m_systems: Vec<VfxSystemDesc>,
    pub m_renderer_settings: VfxRendererSettings,
    pub m_culling_flags: i32,
}

impl crate::Parse for VisualEffectAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_infos = VisualEffectInfo::parse(r, ver)?;
        r.align(4)?;
        let m_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxSystemDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_renderer_settings = VfxRendererSettings::parse(r, ver)?;
        r.align(4)?;
        let m_culling_flags = r.read_i32()?;

        Ok(VisualEffectAsset {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_infos,
            m_systems,
            m_renderer_settings,
            m_culling_flags,
        })
    }
}
