//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GlTextureSettings {
    pub m_filter_mode: i32,
    pub m_aniso: i32,
    pub m_mip_bias: f32,
    pub m_wrap_mode: Option<i32>,
    pub m_wrap_u: Option<i32>,
    pub m_wrap_v: Option<i32>,
    pub m_wrap_w: Option<i32>,
}

impl crate::Parse for GlTextureSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_filter_mode = rdr.read_i32()?;
        let m_aniso = rdr.read_i32()?;
        let m_mip_bias = rdr.read_f32()?;
        let m_wrap_mode = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_wrap_u = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_wrap_v = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_wrap_w = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(GlTextureSettings {
            m_filter_mode,
            m_aniso,
            m_mip_bias,
            m_wrap_mode,
            m_wrap_u,
            m_wrap_v,
            m_wrap_w,
        })
    }
}
