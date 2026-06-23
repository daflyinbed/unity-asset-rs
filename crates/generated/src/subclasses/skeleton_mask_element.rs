//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SkeletonMaskElement {
    pub m_index: u32,
    pub m_weight: f32,
}

impl crate::Parse for SkeletonMaskElement {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_index = r.read_u32()?;
        let m_weight = r.read_f32()?;

        Ok(SkeletonMaskElement {
            m_index,
            m_weight,
        })
    }
}
