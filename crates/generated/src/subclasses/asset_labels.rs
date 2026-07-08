//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetLabels {
    pub m_labels: Vec<String>,
}

impl crate::Parse for AssetLabels {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_labels = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(AssetLabels {
            m_labels,
        })
    }
}
