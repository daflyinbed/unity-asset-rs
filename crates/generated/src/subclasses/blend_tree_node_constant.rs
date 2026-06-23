//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlendTreeNodeConstant {
    pub m_blend_event_id: u32,
    pub m_child_indices: Vec<u32>,
    pub m_child_threshold_array: Vec<f32>,
    pub m_clip_id: u32,
    pub m_duration: f32,
}

impl crate::Parse for BlendTreeNodeConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_blend_event_id = r.read_u32()?;
        let m_child_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_child_threshold_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_clip_id = r.read_u32()?;
        let m_duration = r.read_f32()?;

        Ok(BlendTreeNodeConstant {
            m_blend_event_id,
            m_child_indices,
            m_child_threshold_array,
            m_clip_id,
            m_duration,
        })
    }
}
