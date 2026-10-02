//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：ResourceManager_DependencyMDependenciesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ResourceManager_DependencyMDependenciesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for ResourceManager_DependencyMDependenciesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ResourceManager_DependencyMDependenciesVariant {
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
pub struct ResourceManager_Dependency {
    pub m_object: PPtr,
    pub m_dependencies: ResourceManager_DependencyMDependenciesVariant,
}

impl crate::Parse for ResourceManager_Dependency {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object = crate::PPtr::parse(rdr, ver)?;
        let m_dependencies = ResourceManager_DependencyMDependenciesVariant::parse(rdr, ver)?;

        Ok(ResourceManager_Dependency {
            m_object,
            m_dependencies,
        })
    }
}

impl crate::ToValue for ResourceManager_Dependency {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Object".to_string(), self.m_object.to_value())),
            Some(("m_Dependencies".to_string(), self.m_dependencies.to_value())),
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
                "m_Dependencies" => (self.m_dependencies).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_object).inspect_value(path, offset, limit))
                .map(|v| ("m_Object".to_string(), v)),
            ((self.m_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("m_Dependencies".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
