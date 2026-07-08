//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;


/// 版本枚举：MonoScriptMPropertiesHashVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoScriptMPropertiesHashVariant {
    Default(u32),
    V5_0(Hash128),
}

impl crate::Parse for MonoScriptMPropertiesHashVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = Hash128::parse(r, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_u32()?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_execution_order = r.read_i32()?;
        let m_properties_hash = MonoScriptMPropertiesHashVariant::parse(r, ver)?;
        let m_class_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_namespace = r.read_aligned_string()?;
        r.align(4)?;
        let m_assembly_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_is_editor_script = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(MonoScript {
            m_name,
            m_execution_order,
            m_properties_hash,
            m_class_name,
            m_namespace,
            m_assembly_name,
            m_is_editor_script,
        })
    }
}
