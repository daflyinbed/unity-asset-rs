//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct DenseClip {
    pub m_frame_count: i32,
    pub m_curve_count: u32,
    pub m_sample_rate: f32,
    pub m_begin_time: f32,
    pub m_sample_array: Vec<f32>,
}

impl crate::Parse for DenseClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_frame_count = rdr.read_i32()?;
        let m_curve_count = rdr.read_u32()?;
        let m_sample_rate = rdr.read_f32()?;
        let m_begin_time = rdr.read_f32()?;
        let m_sample_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(DenseClip {
            m_frame_count,
            m_curve_count,
            m_sample_rate,
            m_begin_time,
            m_sample_array,
        })
    }
}
