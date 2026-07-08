//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LuminIcon {
    pub m_name: String,
    pub m_model_folder_path: String,
    pub m_portal_folder_path: String,
}

impl crate::Parse for LuminIcon {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_model_folder_path = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_portal_folder_path = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(LuminIcon {
            m_name,
            m_model_folder_path,
            m_portal_folder_path,
        })
    }
}
