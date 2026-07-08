//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetSettings {
    pub m_build_target: String,
    pub m_max_texture_size: Option<i32>,
    pub m_texture_width: Option<i32>,
    pub m_texture_format: i32,
    pub m_texture_height: Option<i32>,
    pub m_compression_quality: Option<i32>,
    pub m_loading_behavior: Option<i32>,
    pub m_allows_alpha_splitting: Option<bool>,
}

impl crate::Parse for BuildTargetSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_build_target = None;
        let mut m_max_texture_size = None;
        let mut m_texture_width = None;
        let mut m_texture_format = None;
        let mut m_texture_height = None;
        let mut m_compression_quality = None;
        let mut m_loading_behavior = None;
        let mut m_allows_alpha_splitting = None;
            m_build_target = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_texture_width = Some(rdr.read_i32()?);
            m_texture_height = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_max_texture_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
            m_texture_width = Some(rdr.read_i32()?);
            m_texture_height = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_max_texture_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_texture_width = Some(rdr.read_i32()?);
            m_texture_height = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 2) {
            m_max_texture_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_texture_width = Some(rdr.read_i32()?);
            m_texture_height = Some(rdr.read_i32()?);
        } else {
            m_max_texture_size = Some(rdr.read_i32()?);
        }
            m_texture_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_loading_behavior = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_allows_alpha_splitting = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 4) {
            m_loading_behavior = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_allows_alpha_splitting = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_loading_behavior = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 2) {
            m_compression_quality = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_loading_behavior = Some(rdr.read_i32()?);
        } else {
        }

        Ok(BuildTargetSettings {
            m_build_target: m_build_target.unwrap(),
            m_max_texture_size,
            m_texture_width,
            m_texture_format: m_texture_format.unwrap(),
            m_texture_height,
            m_compression_quality,
            m_loading_behavior,
            m_allows_alpha_splitting,
        })
    }
}
