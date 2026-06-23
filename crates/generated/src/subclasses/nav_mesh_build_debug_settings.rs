//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshBuildDebugSettings {
    pub m_flags: u8,
}

impl crate::Parse for NavMeshBuildDebugSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_flags = r.read_u8()?;
        r.align(4)?;

        Ok(NavMeshBuildDebugSettings {
            m_flags,
        })
    }
}
