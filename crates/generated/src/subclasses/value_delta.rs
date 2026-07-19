//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ValueDelta {
    pub m_start: f32,
    pub m_stop: f32,
}

impl crate::Parse for ValueDelta {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start = rdr.read_f32()?;
        let m_stop = rdr.read_f32()?;

        Ok(ValueDelta { m_start, m_stop })
    }
}
