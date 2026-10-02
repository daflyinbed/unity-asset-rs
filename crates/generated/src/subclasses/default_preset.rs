//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct DefaultPreset {
    pub m_preset: PPtr,
    pub m_filter: Option<String>,
    pub m_disabled: Option<bool>,
}

impl crate::Parse for DefaultPreset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_preset = crate::PPtr::parse(rdr, ver)?;
        let m_filter = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(rdr.read_aligned_string()?)
        } else {
            None
        };
        let m_disabled = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(DefaultPreset {
            m_preset,
            m_filter,
            m_disabled,
        })
    }
}

impl crate::ToValue for DefaultPreset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Preset".to_string(), self.m_preset.to_value())),
            crate::opt_field("m_Filter", &self.m_filter),
            crate::opt_field("m_Disabled", &self.m_disabled),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Preset" => (self.m_preset).inspect_value(path, offset, limit),
                "m_Filter" => (self.m_filter).inspect_value(path, offset, limit),
                "m_Disabled" => (self.m_disabled).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_preset).inspect_value(path, offset, limit))
                .map(|v| ("m_Preset".to_string(), v)),
            ((self.m_filter).inspect_value(path, offset, limit))
                .map(|v| ("m_Filter".to_string(), v)),
            ((self.m_disabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Disabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
