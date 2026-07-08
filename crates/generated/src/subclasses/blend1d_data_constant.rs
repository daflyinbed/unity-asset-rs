//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Blend1dDataConstant {
    pub m_child_threshold_array: Vec<f32>,
}

impl crate::Parse for Blend1dDataConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_child_threshold_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Blend1dDataConstant {
            m_child_threshold_array,
        })
    }
}
