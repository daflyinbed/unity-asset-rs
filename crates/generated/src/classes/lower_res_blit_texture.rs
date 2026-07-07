//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LowerResBlitTexture {
    pub m_name: String,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_downscale_fallback: Option<bool>,
}

impl crate::Parse for LowerResBlitTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_forced_fallback_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_alpha_channel_optional = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_downscale_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(LowerResBlitTexture {
            m_name,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_downscale_fallback,
        })
    }
}
