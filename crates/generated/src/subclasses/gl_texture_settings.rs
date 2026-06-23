//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GlTextureSettings {
    pub m_filter_mode: i32,
    pub m_aniso: i32,
    pub m_mip_bias: f32,
    pub m_wrap_mode: i32,
}

impl crate::Parse for GlTextureSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_filter_mode = r.read_i32()?;
        let m_aniso = r.read_i32()?;
        let m_mip_bias = r.read_f32()?;
        let m_wrap_mode = r.read_i32()?;

        Ok(GlTextureSettings {
            m_filter_mode,
            m_aniso,
            m_mip_bias,
            m_wrap_mode,
        })
    }
}
