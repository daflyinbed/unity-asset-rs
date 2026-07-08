//! 自动生成。请勿手动修改。

use crate::subclasses::matrix4x4f::Matrix4x4f;

#[derive(Debug, Clone)]
pub struct VfxEntryExposed_Matrix4x4f {
    pub m_value: Matrix4x4f,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed_Matrix4x4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = Matrix4x4f::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        let m_overridden = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExposed_Matrix4x4f {
            m_value,
            m_name,
            m_overridden,
        })
    }
}
