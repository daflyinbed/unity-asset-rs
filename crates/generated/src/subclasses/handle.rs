//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct Handle {
    pub m_x: xform,
    pub m_parent_human_index: u32,
    pub m_id: u32,
}

impl crate::Parse for Handle {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(r, ver)?;
        let m_parent_human_index = r.read_u32()?;
        let m_id = r.read_u32()?;

        Ok(Handle {
            m_x,
            m_parent_human_index,
            m_id,
        })
    }
}
