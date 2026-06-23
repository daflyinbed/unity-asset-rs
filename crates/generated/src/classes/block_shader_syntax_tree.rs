//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct BlockShaderSyntaxTree {
    pub parent: NamedObject,
    pub m_name: String,
    pub file_path: String,
    pub source: String,
}

impl crate::Parse for BlockShaderSyntaxTree {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let file_path = r.read_aligned_string()?;
        r.align(4)?;
        let source = r.read_aligned_string()?;
        r.align(4)?;

        Ok(BlockShaderSyntaxTree {
            parent,
            m_name,
            file_path,
            source,
        })
    }
}
