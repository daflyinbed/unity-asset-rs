//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct BlockShaderSourceArtifact {
    pub parent: Object,
    pub shader_name: String,
    pub shader_source: String,
}

impl crate::Parse for BlockShaderSourceArtifact {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let shader_name = r.read_aligned_string()?;
        r.align(4)?;
        let shader_source = r.read_aligned_string()?;
        r.align(4)?;

        Ok(BlockShaderSourceArtifact {
            parent,
            shader_name,
            shader_source,
        })
    }
}
