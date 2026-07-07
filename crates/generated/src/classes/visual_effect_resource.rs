//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_editor_system_desc::VfxEditorSystemDesc;
use crate::subclasses::vfx_shader_source_desc::VfxShaderSourceDesc;
use crate::subclasses::visual_effect_settings::VisualEffectSettings;

#[derive(Debug, Clone)]
pub struct VisualEffectResource {
    pub m_name: String,
    pub m_graph: PPtr,
    pub m_shader_sources: Option<Vec<VfxShaderSourceDesc>>,
    pub m_infos: VisualEffectSettings,
    pub m_systems: Option<Vec<VfxEditorSystemDesc>>,
}

impl crate::Parse for VisualEffectResource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_graph = crate::PPtr::parse(r, ver)?;
        let m_shader_sources = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxShaderSourceDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_infos = VisualEffectSettings::parse(r, ver)?;
        r.align(4)?;
        let m_systems = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxEditorSystemDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VisualEffectResource {
            m_name,
            m_graph,
            m_shader_sources,
            m_infos,
            m_systems,
        })
    }
}
