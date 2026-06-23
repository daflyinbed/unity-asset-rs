//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ProceduralMaterialInformation {
    pub m_offset: Vector2f,
    pub m_scale: Vector2f,
    pub m_generated_at_loading: i32,
}

impl crate::Parse for ProceduralMaterialInformation {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_offset = Vector2f::parse(r, ver)?;
        let m_scale = Vector2f::parse(r, ver)?;
        let m_generated_at_loading = r.read_i32()?;

        Ok(ProceduralMaterialInformation {
            m_offset,
            m_scale,
            m_generated_at_loading,
        })
    }
}
