//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

/// 版本枚举：MonoScriptMPropertiesHashVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoScriptMPropertiesHashVariant {
    Default(u32),
    V5_0(Box<Hash128>),
}

impl crate::Parse for MonoScriptMPropertiesHashVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({ Ok(Box::new(Hash128::parse(rdr, ver)?)) })?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoScriptMPropertiesHashVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MonoScript {
    pub m_name: String,
    pub m_execution_order: i32,
    pub m_properties_hash: MonoScriptMPropertiesHashVariant,
    pub m_class_name: String,
    pub m_namespace: String,
    pub m_assembly_name: String,
    pub m_is_editor_script: Option<bool>,
}

impl crate::Parse for MonoScript {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_execution_order = None;
        let mut m_properties_hash = None;
        let mut m_class_name = None;
        let mut m_namespace = None;
        let mut m_assembly_name = None;
        let mut m_is_editor_script = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_execution_order = Some(rdr.read_i32()?);
        m_properties_hash = Some(MonoScriptMPropertiesHashVariant::parse(rdr, ver)?);
        m_class_name = Some(rdr.read_aligned_string()?);
        m_namespace = Some(rdr.read_aligned_string()?);
        m_assembly_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
        } else {
            m_is_editor_script = Some(rdr.read_bool()?);
        }

        Ok(MonoScript {
            m_name: m_name.unwrap(),
            m_execution_order: m_execution_order.unwrap(),
            m_properties_hash: m_properties_hash.unwrap(),
            m_class_name: m_class_name.unwrap(),
            m_namespace: m_namespace.unwrap(),
            m_assembly_name: m_assembly_name.unwrap(),
            m_is_editor_script,
        })
    }
}

impl crate::ToValue for MonoScript {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ExecutionOrder".to_string(),
                self.m_execution_order.to_value(),
            )),
            Some((
                "m_PropertiesHash".to_string(),
                self.m_properties_hash.to_value(),
            )),
            Some(("m_ClassName".to_string(), self.m_class_name.to_value())),
            Some(("m_Namespace".to_string(), self.m_namespace.to_value())),
            Some((
                "m_AssemblyName".to_string(),
                self.m_assembly_name.to_value(),
            )),
            crate::opt_field("m_IsEditorScript", &self.m_is_editor_script),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_ExecutionOrder" => (self.m_execution_order).inspect_value(path, offset, limit),
                "m_PropertiesHash" => (self.m_properties_hash).inspect_value(path, offset, limit),
                "m_ClassName" => (self.m_class_name).inspect_value(path, offset, limit),
                "m_Namespace" => (self.m_namespace).inspect_value(path, offset, limit),
                "m_AssemblyName" => (self.m_assembly_name).inspect_value(path, offset, limit),
                "m_IsEditorScript" => (self.m_is_editor_script).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_execution_order).inspect_value(path, offset, limit))
                .map(|v| ("m_ExecutionOrder".to_string(), v)),
            ((self.m_properties_hash).inspect_value(path, offset, limit))
                .map(|v| ("m_PropertiesHash".to_string(), v)),
            ((self.m_class_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassName".to_string(), v)),
            ((self.m_namespace).inspect_value(path, offset, limit))
                .map(|v| ("m_Namespace".to_string(), v)),
            ((self.m_assembly_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssemblyName".to_string(), v)),
            ((self.m_is_editor_script).inspect_value(path, offset, limit))
                .map(|v| ("m_IsEditorScript".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
