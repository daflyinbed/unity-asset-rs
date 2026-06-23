//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointTranslationLimits2D {
    pub m_lower_translation: f32,
    pub m_upper_translation: f32,
}

impl crate::Parse for JointTranslationLimits2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lower_translation = r.read_f32()?;
        let m_upper_translation = r.read_f32()?;

        Ok(JointTranslationLimits2D {
            m_lower_translation,
            m_upper_translation,
        })
    }
}
