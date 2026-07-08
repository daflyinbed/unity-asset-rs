//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlendDirectDataConstant {
    pub m_child_blend_event_id_array: Vec<u32>,
    pub m_normalized_blend_values: bool,
}

impl crate::Parse for BlendDirectDataConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_child_blend_event_id_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_normalized_blend_values = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BlendDirectDataConstant {
            m_child_blend_event_id_array,
            m_normalized_blend_values,
        })
    }
}
