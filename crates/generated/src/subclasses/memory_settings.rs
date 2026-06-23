//! 自动生成。请勿手动修改。

use crate::subclasses::platform_memory_settings::PlatformMemorySettings;

#[derive(Debug, Clone)]
pub struct MemorySettings {
    pub m_object_hide_flags: u32,
    pub m_editor_memory_settings: PlatformMemorySettings,
    pub m_platform_memory_settings: Vec<(i32, PlatformMemorySettings)>,
}

impl crate::Parse for MemorySettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_editor_memory_settings = PlatformMemorySettings::parse(r, ver)?;
        let m_platform_memory_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = PlatformMemorySettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(MemorySettings {
            m_object_hide_flags,
            m_editor_memory_settings,
            m_platform_memory_settings,
        })
    }
}
