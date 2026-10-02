//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AnimationClipOverride {
    pub m_original_clip: PPtr,
    pub m_override_clip: PPtr,
}

impl crate::Parse for AnimationClipOverride {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_original_clip = crate::PPtr::parse(rdr, ver)?;
        let m_override_clip = crate::PPtr::parse(rdr, ver)?;

        Ok(AnimationClipOverride {
            m_original_clip,
            m_override_clip,
        })
    }
}

impl crate::ToValue for AnimationClipOverride {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_OriginalClip".to_string(),
                self.m_original_clip.to_value(),
            )),
            Some((
                "m_OverrideClip".to_string(),
                self.m_override_clip.to_value(),
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
                "m_OriginalClip" => (self.m_original_clip).inspect_value(path, offset, limit),
                "m_OverrideClip" => (self.m_override_clip).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_original_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_OriginalClip".to_string(), v)),
            ((self.m_override_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_OverrideClip".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
