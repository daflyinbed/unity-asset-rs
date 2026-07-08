//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformSettings {
    pub m_build_target: String,
    pub m_max_texture_size: i32,
    pub m_texture_format: i32,
    pub m_resize_algorithm: Option<i32>,
    pub m_texture_compression: i32,
    pub m_compression_quality: i32,
    pub m_crunched_compression: bool,
    pub m_allows_alpha_splitting: bool,
    pub m_overridden: bool,
}

impl crate::Parse for PlatformSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_build_target = None;
        let mut m_max_texture_size = None;
        let mut m_texture_format = None;
        let mut m_resize_algorithm = None;
        let mut m_texture_compression = None;
        let mut m_compression_quality = None;
        let mut m_crunched_compression = None;
        let mut m_allows_alpha_splitting = None;
        let mut m_overridden = None;
            m_build_target = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_max_texture_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_resize_algorithm = Some(rdr.read_i32()?);
        } else {
        }
            m_texture_format = Some(rdr.read_i32()?);
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting = Some(rdr.read_bool()?);
            m_overridden = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(PlatformSettings {
            m_build_target: m_build_target.unwrap(),
            m_max_texture_size: m_max_texture_size.unwrap(),
            m_texture_format: m_texture_format.unwrap(),
            m_resize_algorithm,
            m_texture_compression: m_texture_compression.unwrap(),
            m_compression_quality: m_compression_quality.unwrap(),
            m_crunched_compression: m_crunched_compression.unwrap(),
            m_allows_alpha_splitting: m_allows_alpha_splitting.unwrap(),
            m_overridden: m_overridden.unwrap(),
        })
    }
}
