//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ResourceManager_Dependency {
    pub m_object: PPtr,
    pub m_dependencies: Vec<PPtr>,
}

impl crate::Parse for ResourceManager_Dependency {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object = crate::PPtr::parse(r, ver)?;
        let m_dependencies = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ResourceManager_Dependency {
            m_object,
            m_dependencies,
        })
    }
}
