//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EmbeddedNativeType {
    pub m_float_array: Vec<f32>,
    pub m_string: String,
}

impl crate::Parse for EmbeddedNativeType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_float_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_string = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(EmbeddedNativeType {
            m_float_array,
            m_string,
        })
    }
}
