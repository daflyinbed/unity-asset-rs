//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGraphicsJobMode {
    pub m_build_target: String,
    pub m_graphics_job_mode: i32,
}

impl crate::Parse for BuildTargetGraphicsJobMode {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_graphics_job_mode = r.read_i32()?;

        Ok(BuildTargetGraphicsJobMode {
            m_build_target,
            m_graphics_job_mode,
        })
    }
}
