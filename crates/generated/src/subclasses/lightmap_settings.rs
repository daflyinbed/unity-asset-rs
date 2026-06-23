//! 自动生成。请勿手动修改。

use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::lightmap_editor_settings::LightmapEditorSettings;

#[derive(Debug, Clone)]
pub struct LightmapSettings {
    pub m_object_hide_flags: u32,
    pub m_lightmaps: Vec<LightmapData>,
    pub m_lightmaps_mode: i32,
    pub m_use_dual_lightmaps_in_forward: bool,
    pub m_lightmap_editor_settings: LightmapEditorSettings,
}

impl crate::Parse for LightmapSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_lightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_lightmaps_mode = r.read_i32()?;
        let m_use_dual_lightmaps_in_forward = r.read_bool()?;
        r.align(4)?;
        let m_lightmap_editor_settings = LightmapEditorSettings::parse(r, ver)?;

        Ok(LightmapSettings {
            m_object_hide_flags,
            m_lightmaps,
            m_lightmaps_mode,
            m_use_dual_lightmaps_in_forward,
            m_lightmap_editor_settings,
        })
    }
}
