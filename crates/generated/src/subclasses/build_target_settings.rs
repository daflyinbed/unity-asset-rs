//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetSettings {
    pub m_build_target: String,
    pub m_max_texture_size: i32,
    pub m_texture_format: i32,
}

impl crate::Parse for BuildTargetSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_max_texture_size = r.read_i32()?;
        let m_texture_format = r.read_i32()?;

        Ok(BuildTargetSettings {
            m_build_target,
            m_max_texture_size,
            m_texture_format,
        })
    }
}
