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
