//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MeshBlendShapeChannel {
    pub name: String,
    pub name_hash: u32,
    pub frame_index: i32,
    pub frame_count: i32,
}

impl crate::Parse for MeshBlendShapeChannel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let name_hash = rdr.read_u32()?;
        let frame_index = rdr.read_i32()?;
        let frame_count = rdr.read_i32()?;

        Ok(MeshBlendShapeChannel {
            name,
            name_hash,
            frame_index,
            frame_count,
        })
    }
}
