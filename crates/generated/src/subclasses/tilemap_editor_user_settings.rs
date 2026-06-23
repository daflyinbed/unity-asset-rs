//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TilemapEditorUserSettings {
    pub m_object_hide_flags: u32,
    pub m_last_used_palette: PPtr,
    pub m_focus_mode: i32,
}

impl crate::Parse for TilemapEditorUserSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_last_used_palette = crate::PPtr::parse(r, ver)?;
        let m_focus_mode = r.read_i32()?;
        r.align(4)?;

        Ok(TilemapEditorUserSettings {
            m_object_hide_flags,
            m_last_used_palette,
            m_focus_mode,
        })
    }
}
