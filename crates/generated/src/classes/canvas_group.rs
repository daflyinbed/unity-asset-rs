//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct CanvasGroup {
    pub m_game_object: PPtr,
    pub m_alpha: f32,
    pub m_enabled: Option<u8>,
    pub m_interactable: bool,
    pub m_blocks_raycasts: bool,
    pub m_ignore_parent_groups: bool,
}

impl crate::Parse for CanvasGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_alpha = None;
        let mut m_enabled = None;
        let mut m_interactable = None;
        let mut m_blocks_raycasts = None;
        let mut m_ignore_parent_groups = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 1) {
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else {
        }
        m_alpha = Some(rdr.read_f32()?);
        m_interactable = Some(rdr.read_bool()?);
        m_blocks_raycasts = Some(rdr.read_bool()?);
        m_ignore_parent_groups = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(CanvasGroup {
            m_game_object: m_game_object.unwrap(),
            m_alpha: m_alpha.unwrap(),
            m_enabled,
            m_interactable: m_interactable.unwrap(),
            m_blocks_raycasts: m_blocks_raycasts.unwrap(),
            m_ignore_parent_groups: m_ignore_parent_groups.unwrap(),
        })
    }
}

impl crate::ToValue for CanvasGroup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Alpha".to_string(), self.m_alpha.to_value())),
            crate::opt_field("m_Enabled", &self.m_enabled),
            Some(("m_Interactable".to_string(), self.m_interactable.to_value())),
            Some((
                "m_BlocksRaycasts".to_string(),
                self.m_blocks_raycasts.to_value(),
            )),
            Some((
                "m_IgnoreParentGroups".to_string(),
                self.m_ignore_parent_groups.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Alpha" => (self.m_alpha).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Interactable" => (self.m_interactable).inspect_value(path, offset, limit),
                "m_BlocksRaycasts" => (self.m_blocks_raycasts).inspect_value(path, offset, limit),
                "m_IgnoreParentGroups" => {
                    (self.m_ignore_parent_groups).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_alpha).inspect_value(path, offset, limit)).map(|v| ("m_Alpha".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_interactable).inspect_value(path, offset, limit))
                .map(|v| ("m_Interactable".to_string(), v)),
            ((self.m_blocks_raycasts).inspect_value(path, offset, limit))
                .map(|v| ("m_BlocksRaycasts".to_string(), v)),
            ((self.m_ignore_parent_groups).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreParentGroups".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
