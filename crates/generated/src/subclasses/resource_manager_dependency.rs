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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
