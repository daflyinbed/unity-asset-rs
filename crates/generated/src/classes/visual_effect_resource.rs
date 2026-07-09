//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_editor_system_desc::VfxEditorSystemDesc;
use crate::subclasses::vfx_shader_source_desc::VfxShaderSourceDesc;
use crate::subclasses::visual_effect_info::VisualEffectInfo;
use crate::subclasses::visual_effect_settings::VisualEffectSettings;

#[derive(Debug, Clone)]
pub struct VisualEffectResource {
    pub m_name: String,
    pub m_graph: PPtr,
    pub m_shader_sources: Option<Vec<VfxShaderSourceDesc>>,
    pub m_settings: Option<VisualEffectSettings>,
    pub m_infos: Option<VisualEffectInfo>,
    pub m_systems: Option<Vec<VfxEditorSystemDesc>>,
}

impl crate::Parse for VisualEffectResource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_graph = None;
        let mut m_shader_sources = None;
        let mut m_settings = None;
        let mut m_infos = None;
        let mut m_systems = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_graph = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_settings = Some(VisualEffectSettings::parse(rdr, ver)?);
        } else {
            m_shader_sources = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxShaderSourceDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_infos = Some(VisualEffectInfo::parse(rdr, ver)?);
            m_systems = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEditorSystemDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }

        Ok(VisualEffectResource {
            m_name: m_name.unwrap(),
            m_graph: m_graph.unwrap(),
            m_shader_sources,
            m_settings,
            m_infos,
            m_systems,
        })
    }
}
