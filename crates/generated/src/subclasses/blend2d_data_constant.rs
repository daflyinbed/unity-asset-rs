//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Blend2dDataConstant {
    pub m_child_threshold_array: Vec<f32>,
}

impl crate::Parse for Blend2dDataConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_child_threshold_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Blend2dDataConstant {
            m_child_threshold_array,
        })
    }
}
