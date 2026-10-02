//! 自动生成。请勿手动修改。

/// 版本枚举：ExpandedDataMExpandedPropertiesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ExpandedDataMExpandedPropertiesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for ExpandedDataMExpandedPropertiesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ExpandedDataMExpandedPropertiesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExpandedData {
    pub m_inspector_expanded: bool,
    pub m_class_id: i32,
    pub m_script_class: String,
    pub m_expanded_properties: ExpandedDataMExpandedPropertiesVariant,
}

impl crate::Parse for ExpandedData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_inspector_expanded = rdr.read_bool()?;
        rdr.align(4)?;
        let m_class_id = rdr.read_i32()?;
        let m_script_class = rdr.read_aligned_string()?;
        let m_expanded_properties = ExpandedDataMExpandedPropertiesVariant::parse(rdr, ver)?;

        Ok(ExpandedData {
            m_inspector_expanded,
            m_class_id,
            m_script_class,
            m_expanded_properties,
        })
    }
}

impl crate::ToValue for ExpandedData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_InspectorExpanded".to_string(),
                self.m_inspector_expanded.to_value(),
            )),
            Some(("m_ClassID".to_string(), self.m_class_id.to_value())),
            Some(("m_ScriptClass".to_string(), self.m_script_class.to_value())),
            Some((
                "m_ExpandedProperties".to_string(),
                self.m_expanded_properties.to_value(),
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
                "m_InspectorExpanded" => {
                    (self.m_inspector_expanded).inspect_value(path, offset, limit)
                }
                "m_ClassID" => (self.m_class_id).inspect_value(path, offset, limit),
                "m_ScriptClass" => (self.m_script_class).inspect_value(path, offset, limit),
                "m_ExpandedProperties" => {
                    (self.m_expanded_properties).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_inspector_expanded).inspect_value(path, offset, limit))
                .map(|v| ("m_InspectorExpanded".to_string(), v)),
            ((self.m_class_id).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassID".to_string(), v)),
            ((self.m_script_class).inspect_value(path, offset, limit))
                .map(|v| ("m_ScriptClass".to_string(), v)),
            ((self.m_expanded_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_ExpandedProperties".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
