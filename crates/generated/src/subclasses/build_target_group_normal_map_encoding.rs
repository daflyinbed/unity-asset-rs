//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupNormalMapEncoding {
    pub m_build_target: String,
    pub m_encoding: i32,
}

impl crate::Parse for BuildTargetGroupNormalMapEncoding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_encoding = rdr.read_i32()?;

        Ok(BuildTargetGroupNormalMapEncoding {
            m_build_target,
            m_encoding,
        })
    }
}
