//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SceneAsset {
    pub m_name: String,
    pub m_message: Option<String>,
    pub m_error_code: Option<i32>,
    pub m_is_warning: Option<bool>,
}

impl crate::Parse for SceneAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_message = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_error_code = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_warning = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(SceneAsset {
            m_name,
            m_message,
            m_error_code,
            m_is_warning,
        })
    }
}
