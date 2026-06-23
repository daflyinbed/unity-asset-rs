//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformSettings {
    pub m_build_target: String,
    pub m_max_texture_size: i32,
    pub m_texture_format: i32,
    pub m_texture_compression: i32,
    pub m_compression_quality: i32,
    pub m_crunched_compression: bool,
    pub m_allows_alpha_splitting: bool,
    pub m_overridden: bool,
}

impl crate::Parse for PlatformSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_max_texture_size = r.read_i32()?;
        let m_texture_format = r.read_i32()?;
        let m_texture_compression = r.read_i32()?;
        let m_compression_quality = r.read_i32()?;
        let m_crunched_compression = r.read_bool()?;
        let m_allows_alpha_splitting = r.read_bool()?;
        let m_overridden = r.read_bool()?;
        r.align(4)?;

        Ok(PlatformSettings {
            m_build_target,
            m_max_texture_size,
            m_texture_format,
            m_texture_compression,
            m_compression_quality,
            m_crunched_compression,
            m_allows_alpha_splitting,
            m_overridden,
        })
    }
}
