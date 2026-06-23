//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderParam {
    pub name: String,
    pub r#type: i32,
    pub offset: i32,
    pub array_size: i32,
    pub row_count: i32,
    pub col_count: i32,
}

impl crate::Parse for RayTracingShaderParam {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let r#type = r.read_i32()?;
        let offset = r.read_i32()?;
        let array_size = r.read_i32()?;
        let row_count = r.read_i32()?;
        let col_count = r.read_i32()?;

        Ok(RayTracingShaderParam {
            name,
            r#type,
            offset,
            array_size,
            row_count,
            col_count,
        })
    }
}
