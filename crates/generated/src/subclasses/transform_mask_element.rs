//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TransformMaskElement {
    pub m_path: String,
    pub m_weight: f32,
}

impl crate::Parse for TransformMaskElement {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_weight = r.read_f32()?;

        Ok(TransformMaskElement {
            m_path,
            m_weight,
        })
    }
}
