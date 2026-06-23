//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::property_modification::PropertyModification;

#[derive(Debug, Clone)]
pub struct PrefabModification {
    pub m_transform_parent: PPtr,
    pub m_modifications: Vec<PropertyModification>,
    pub m_removed_components: Vec<PPtr>,
}

impl crate::Parse for PrefabModification {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_transform_parent = crate::PPtr::parse(r, ver)?;
        let m_modifications = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PropertyModification::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_removed_components = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(PrefabModification {
            m_transform_parent,
            m_modifications,
            m_removed_components,
        })
    }
}
