//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_editor_system_desc::VfxEditorSystemDesc;
use crate::subclasses::vfx_shader_source_desc::VfxShaderSourceDesc;
use crate::subclasses::visual_effect_info::VisualEffectInfo;

#[derive(Debug, Clone)]
pub struct VisualEffectResource {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_graph: PPtr,
    pub m_shader_sources: Vec<VfxShaderSourceDesc>,
    pub m_infos: VisualEffectInfo,
    pub m_systems: Vec<VfxEditorSystemDesc>,
}

impl crate::Parse for VisualEffectResource {
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
        let m_graph = crate::PPtr::parse(r, ver)?;
        let m_shader_sources = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxShaderSourceDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_infos = VisualEffectInfo::parse(r, ver)?;
        r.align(4)?;
        let m_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxEditorSystemDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VisualEffectResource {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_graph,
            m_shader_sources,
            m_infos,
            m_systems,
        })
    }
}
