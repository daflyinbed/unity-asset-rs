//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetBatching {
    pub m_build_target: String,
    pub m_static_batching: bool,
    pub m_dynamic_batching: bool,
}

impl crate::Parse for BuildTargetBatching {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_static_batching = rdr.read_bool()?;
        let m_dynamic_batching = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetBatching {
            m_build_target,
            m_static_batching,
            m_dynamic_batching,
        })
    }
}
