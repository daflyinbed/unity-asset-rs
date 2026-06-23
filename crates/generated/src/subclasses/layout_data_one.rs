//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LayoutDataOne {
    pub m_float_array: Vec<f32>,
}

impl crate::Parse for LayoutDataOne {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_float_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(LayoutDataOne {
            m_float_array,
        })
    }
}
