//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ClothAttachment {
    pub m_collider: PPtr,
    pub m_two_way_interaction: bool,
    pub m_tearable: bool,
}

impl crate::Parse for ClothAttachment {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_collider = crate::PPtr::parse(r, ver)?;
        let m_two_way_interaction = r.read_bool()?;
        let m_tearable = r.read_bool()?;

        Ok(ClothAttachment {
            m_collider,
            m_two_way_interaction,
            m_tearable,
        })
    }
}
