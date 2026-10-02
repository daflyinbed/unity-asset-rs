//! 自动生成。请勿手动修改。

use crate::subclasses::expanded_data::ExpandedData;

#[derive(Debug, Clone)]
pub struct InspectorExpandedState {
    pub m_expanded_data: Vec<ExpandedData>,
}

impl crate::Parse for InspectorExpandedState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expanded_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ExpandedData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(InspectorExpandedState { m_expanded_data })
    }
}

impl crate::ToValue for InspectorExpandedState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_ExpandedData".to_string(),
            self.m_expanded_data.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ExpandedData" => (self.m_expanded_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_expanded_data).inspect_value(path, offset, limit))
                .map(|v| ("m_ExpandedData".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
