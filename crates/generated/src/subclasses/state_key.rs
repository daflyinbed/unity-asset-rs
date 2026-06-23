//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StateKey {
    pub m_state_id: u32,
    pub m_layer_index: i32,
}

impl crate::Parse for StateKey {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_id = r.read_u32()?;
        let m_layer_index = r.read_i32()?;

        Ok(StateKey {
            m_state_id,
            m_layer_index,
        })
    }
}
