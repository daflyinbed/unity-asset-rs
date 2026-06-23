//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ShadowSettings {
    pub m_type: i32,
    pub m_resolution: i32,
    pub m_strength: f32,
    pub m_bias: f32,
    pub m_softness: f32,
    pub m_softness_fade: f32,
}

impl crate::Parse for ShadowSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_type = r.read_i32()?;
        let m_resolution = r.read_i32()?;
        let m_strength = r.read_f32()?;
        let m_bias = r.read_f32()?;
        let m_softness = r.read_f32()?;
        let m_softness_fade = r.read_f32()?;

        Ok(ShadowSettings {
            m_type,
            m_resolution,
            m_strength,
            m_bias,
            m_softness,
            m_softness_fade,
        })
    }
}
