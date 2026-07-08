//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderId {
    pub r#type: i32,
    pub name: String,
}

impl crate::Parse for RayTracingShaderId {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = rdr.read_i32()?;
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(RayTracingShaderId {
            r#type,
            name,
        })
    }
}
