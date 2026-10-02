//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;

/// 版本枚举：MonoManagerMAssemblyNamesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoManagerMAssemblyNamesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for MonoManagerMAssemblyNamesVariant {
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

impl crate::ToValue for MonoManagerMAssemblyNamesVariant {
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
pub struct MonoManager {
    pub m_scripts: Vec<PPtr>,
    pub m_script_hashes: Option<Vec<(Hash128, Hash128)>>,
    pub m_assembly_names: Option<MonoManagerMAssemblyNamesVariant>,
    pub m_runtime_class_hashes: Option<Vec<(i32, Hash128)>>,
    pub m_assembly_types: Option<Vec<i32>>,
}

impl crate::Parse for MonoManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_scripts = None;
        let mut m_script_hashes = None;
        let mut m_assembly_names = None;
        let mut m_runtime_class_hashes = None;
        let mut m_assembly_types = None;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_script_hashes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = Hash128::parse(rdr, ver)?;
                                let b = Hash128::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_runtime_class_hashes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = Hash128::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }
        m_scripts = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_assembly_names = Some(MonoManagerMAssemblyNamesVariant::parse(rdr, ver)?);
            m_assembly_types = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            m_assembly_names = Some(MonoManagerMAssemblyNamesVariant::parse(rdr, ver)?);
        }

        Ok(MonoManager {
            m_scripts: m_scripts.unwrap(),
            m_script_hashes,
            m_assembly_names,
            m_runtime_class_hashes,
            m_assembly_types,
        })
    }
}

impl crate::ToValue for MonoManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Scripts".to_string(), self.m_scripts.to_value())),
            crate::opt_field("m_ScriptHashes", &self.m_script_hashes),
            crate::opt_field("m_AssemblyNames", &self.m_assembly_names),
            crate::opt_field("m_RuntimeClassHashes", &self.m_runtime_class_hashes),
            crate::opt_field("m_AssemblyTypes", &self.m_assembly_types),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Scripts" => (self.m_scripts).inspect_value(path, offset, limit),
                "m_ScriptHashes" => (self.m_script_hashes).inspect_value(path, offset, limit),
                "m_AssemblyNames" => (self.m_assembly_names).inspect_value(path, offset, limit),
                "m_RuntimeClassHashes" => {
                    (self.m_runtime_class_hashes).inspect_value(path, offset, limit)
                }
                "m_AssemblyTypes" => (self.m_assembly_types).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_scripts).inspect_value(path, offset, limit))
                .map(|v| ("m_Scripts".to_string(), v)),
            ((self.m_script_hashes).inspect_value(path, offset, limit))
                .map(|v| ("m_ScriptHashes".to_string(), v)),
            ((self.m_assembly_names).inspect_value(path, offset, limit))
                .map(|v| ("m_AssemblyNames".to_string(), v)),
            ((self.m_runtime_class_hashes).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimeClassHashes".to_string(), v)),
            ((self.m_assembly_types).inspect_value(path, offset, limit))
                .map(|v| ("m_AssemblyTypes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
