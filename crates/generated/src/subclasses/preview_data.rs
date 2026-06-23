//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PreviewData {
    pub m_preview_data: Vec<f32>,
    pub m_orig_size: i32,
    pub m_comp_size: i32,
}

impl crate::Parse for PreviewData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_preview_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_orig_size = r.read_i32()?;
        let m_comp_size = r.read_i32()?;

        Ok(PreviewData {
            m_preview_data,
            m_orig_size,
            m_comp_size,
        })
    }
}
