//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TilemapEditorUserSettings {
    pub m_last_used_palette: PPtr,
    pub m_focus_mode: i32,
}

impl crate::Parse for TilemapEditorUserSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_last_used_palette = crate::PPtr::parse(rdr, ver)?;
        let m_focus_mode = rdr.read_i32()?;
        rdr.align(4)?;

        Ok(TilemapEditorUserSettings {
            m_last_used_palette,
            m_focus_mode,
        })
    }
}

impl crate::ToValue for TilemapEditorUserSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_LastUsedPalette".to_string(),
                self.m_last_used_palette.to_value(),
            )),
            Some(("m_FocusMode".to_string(), self.m_focus_mode.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_LastUsedPalette" => {
                    (self.m_last_used_palette).inspect_value(path, offset, limit)
                }
                "m_FocusMode" => (self.m_focus_mode).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_last_used_palette).inspect_value(path, offset, limit))
                .map(|v| ("m_LastUsedPalette".to_string(), v)),
            ((self.m_focus_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_FocusMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
