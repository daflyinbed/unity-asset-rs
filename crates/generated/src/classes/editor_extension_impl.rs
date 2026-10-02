//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bitset::bitset;

#[derive(Debug, Clone)]
pub struct EditorExtensionImpl {
    pub m_object: Option<PPtr>,
    pub m_template_father: Option<PPtr>,
    pub m_data_template: Option<PPtr>,
    pub m_override_variable: Option<bitset>,
    pub g_flattened_type_tree: Option<Vec<u8>>,
}

impl crate::Parse for EditorExtensionImpl {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_object = None;
        let mut m_template_father = None;
        let mut m_data_template = None;
        let mut m_override_variable = None;
        let mut g_flattened_type_tree = None;
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_template_father = Some(crate::PPtr::parse(rdr, ver)?);
            m_data_template = Some(crate::PPtr::parse(rdr, ver)?);
            m_override_variable = Some(bitset::parse(rdr, ver)?);
            g_flattened_type_tree = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }

        Ok(EditorExtensionImpl {
            m_object,
            m_template_father,
            m_data_template,
            m_override_variable,
            g_flattened_type_tree,
        })
    }
}

impl crate::ToValue for EditorExtensionImpl {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Object", &self.m_object),
            crate::opt_field("m_TemplateFather", &self.m_template_father),
            crate::opt_field("m_DataTemplate", &self.m_data_template),
            crate::opt_field("m_OverrideVariable", &self.m_override_variable),
            crate::opt_field("gFlattenedTypeTree", &self.g_flattened_type_tree),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Object" => (self.m_object).inspect_value(path, offset, limit),
                "m_TemplateFather" => (self.m_template_father).inspect_value(path, offset, limit),
                "m_DataTemplate" => (self.m_data_template).inspect_value(path, offset, limit),
                "m_OverrideVariable" => {
                    (self.m_override_variable).inspect_value(path, offset, limit)
                }
                "gFlattenedTypeTree" => {
                    (self.g_flattened_type_tree).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_object).inspect_value(path, offset, limit))
                .map(|v| ("m_Object".to_string(), v)),
            ((self.m_template_father).inspect_value(path, offset, limit))
                .map(|v| ("m_TemplateFather".to_string(), v)),
            ((self.m_data_template).inspect_value(path, offset, limit))
                .map(|v| ("m_DataTemplate".to_string(), v)),
            ((self.m_override_variable).inspect_value(path, offset, limit))
                .map(|v| ("m_OverrideVariable".to_string(), v)),
            ((self.g_flattened_type_tree).inspect_value(path, offset, limit))
                .map(|v| ("gFlattenedTypeTree".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
