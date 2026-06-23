//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetLabels {
    pub m_labels: Vec<String>,
}

impl crate::Parse for AssetLabels {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_labels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AssetLabels {
            m_labels,
        })
    }
}
