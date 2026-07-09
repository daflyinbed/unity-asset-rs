//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct NavMeshObsolete {
    pub m_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
}

impl crate::Parse for NavMeshObsolete {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hide_flags = rdr.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(rdr, ver)?;
        let m_prefab_internal = crate::PPtr::parse(rdr, ver)?;
        let m_name = rdr.read_aligned_string()?;

        Ok(NavMeshObsolete {
            m_hide_flags,
            m_corresponding_source_object,
            m_prefab_internal,
            m_name,
        })
    }
}
