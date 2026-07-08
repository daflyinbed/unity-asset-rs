//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupHdrCubemapEncodingQuality {
    pub m_build_target: String,
    pub m_encoding_quality: i32,
}

impl crate::Parse for BuildTargetGroupHdrCubemapEncodingQuality {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_encoding_quality = rdr.read_i32()?;

        Ok(BuildTargetGroupHdrCubemapEncodingQuality {
            m_build_target,
            m_encoding_quality,
        })
    }
}
