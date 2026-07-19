//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct int3_storage {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl crate::Parse for int3_storage {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_i32()?;
        let y = rdr.read_i32()?;
        let z = rdr.read_i32()?;

        Ok(int3_storage { x, y, z })
    }
}
