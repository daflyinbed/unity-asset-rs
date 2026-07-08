//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TypeStats {
    pub class_id: i32,
    pub size: u64,
    pub object_count: i32,
    pub resource_count: i32,
}

impl crate::Parse for TypeStats {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let class_id = rdr.read_i32()?;
        let size = rdr.read_u64()?;
        let object_count = rdr.read_i32()?;
        let resource_count = rdr.read_i32()?;

        Ok(TypeStats {
            class_id,
            size,
            object_count,
            resource_count,
        })
    }
}
