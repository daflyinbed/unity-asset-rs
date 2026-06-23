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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let class_id = r.read_i32()?;
        let size = r.read_u64()?;
        let object_count = r.read_i32()?;
        let resource_count = r.read_i32()?;

        Ok(TypeStats {
            class_id,
            size,
            object_count,
            resource_count,
        })
    }
}
