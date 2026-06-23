//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Node {
    pub m_parent_id: i32,
    pub m_axes_id: i32,
}

impl crate::Parse for Node {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_parent_id = r.read_i32()?;
        let m_axes_id = r.read_i32()?;

        Ok(Node {
            m_parent_id,
            m_axes_id,
        })
    }
}
