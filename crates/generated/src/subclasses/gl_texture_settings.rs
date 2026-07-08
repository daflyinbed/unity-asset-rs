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
        let mut m_filter_mode = None;
        let mut m_aniso = None;
        let mut m_mip_bias = None;
        let mut m_wrap_mode = None;
        let mut m_wrap_u = None;
        let mut m_wrap_v = None;
        let mut m_wrap_w = None;
            m_filter_mode = Some(rdr.read_i32()?);
            m_aniso = Some(rdr.read_i32()?);
            m_mip_bias = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_wrap_u = Some(rdr.read_i32()?);
            m_wrap_v = Some(rdr.read_i32()?);
            m_wrap_w = Some(rdr.read_i32()?);
        } else {
            m_wrap_mode = Some(rdr.read_i32()?);
        }

        Ok(GlTextureSettings {
            m_filter_mode: m_filter_mode.unwrap(),
            m_aniso: m_aniso.unwrap(),
            m_mip_bias: m_mip_bias.unwrap(),
            m_wrap_mode,
            m_wrap_u,
            m_wrap_v,
            m_wrap_w,
        })
    }
}
