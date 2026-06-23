//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGraphicsJobs {
    pub m_build_target: String,
    pub m_graphics_jobs: bool,
}

impl crate::Parse for BuildTargetGraphicsJobs {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_graphics_jobs = r.read_bool()?;
        r.align(4)?;

        Ok(BuildTargetGraphicsJobs {
            m_build_target,
            m_graphics_jobs,
        })
    }
}
