//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::resource_manager_dependency::ResourceManager_Dependency;

/// 版本枚举：ResourceManagerMDependentAssetsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ResourceManagerMDependentAssetsVariant {
    Default(Vec<ResourceManager_Dependency>),
    V2017_1(Vec<ResourceManager_Dependency>),
}

impl crate::Parse for ResourceManagerMDependentAssetsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ResourceManager_Dependency::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ResourceManager_Dependency::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ResourceManagerMDependentAssetsVariant {
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
pub struct ResourceManager {
    pub m_container: Vec<(String, PPtr)>,
    pub m_dependent_assets: Option<ResourceManagerMDependentAssetsVariant>,
}

impl crate::Parse for ResourceManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_container = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = crate::PPtr::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_dependent_assets = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(ResourceManagerMDependentAssetsVariant::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(ResourceManager {
            m_container,
            m_dependent_assets,
        })
    }
}

impl crate::ToValue for ResourceManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Container".to_string(), self.m_container.to_value())),
            crate::opt_field("m_DependentAssets", &self.m_dependent_assets),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Container" => (self.m_container).inspect_value(path, offset, limit),
                "m_DependentAssets" => (self.m_dependent_assets).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_container).inspect_value(path, offset, limit))
                .map(|v| ("m_Container".to_string(), v)),
            ((self.m_dependent_assets).inspect_value(path, offset, limit))
                .map(|v| ("m_DependentAssets".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
