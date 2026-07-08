//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MaterialBuildTargetSettings {
    pub m_build_target: String,
    pub m_texture_width: i32,
    pub m_texture_height: i32,
    pub m_texture_format: i32,
    pub m_loading_behavior: Option<i32>,
}

impl crate::Parse for MaterialBuildTargetSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_texture_width = rdr.read_i32()?;
        let m_texture_height = rdr.read_i32()?;
        let m_texture_format = rdr.read_i32()?;
        let m_loading_behavior = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(MaterialBuildTargetSettings {
            m_build_target,
            m_texture_width,
            m_texture_height,
            m_texture_format,
            m_loading_behavior,
        })
    }
}
