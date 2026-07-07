//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::resource_manager_dependency::ResourceManager_Dependency;

#[derive(Debug, Clone)]
pub struct ResourceManager {
    pub m_container: Vec<(String, PPtr)>,
    pub m_dependent_assets: Option<Vec<ResourceManager_Dependency>>,
}

impl crate::Parse for ResourceManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_container = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_dependent_assets = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ResourceManager_Dependency::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(ResourceManager {
            m_container,
            m_dependent_assets,
        })
    }
}
