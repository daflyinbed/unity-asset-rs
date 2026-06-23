//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupLightmapEncodingQuality {
    pub m_build_target: String,
    pub m_encoding_quality: i32,
}

impl crate::Parse for BuildTargetGroupLightmapEncodingQuality {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_encoding_quality = r.read_i32()?;

        Ok(BuildTargetGroupLightmapEncodingQuality {
            m_build_target,
            m_encoding_quality,
        })
    }
}
