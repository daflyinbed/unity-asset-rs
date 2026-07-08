//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SkeletonMaskElement {
    pub m_index: Option<u32>,
    pub m_path_hash: Option<u32>,
    pub m_weight: f32,
}

impl crate::Parse for SkeletonMaskElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_index = None;
        let mut m_path_hash = None;
        let mut m_weight = None;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_path_hash = Some(rdr.read_u32()?);
        } else {
            m_index = Some(rdr.read_u32()?);
        }
            m_weight = Some(rdr.read_f32()?);

        Ok(SkeletonMaskElement {
            m_index,
            m_path_hash,
            m_weight: m_weight.unwrap(),
        })
    }
}
