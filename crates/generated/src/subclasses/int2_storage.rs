//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct int2_storage {
    pub x: i32,
    pub y: i32,
}

impl crate::Parse for int2_storage {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_i32()?;
        let y = rdr.read_i32()?;

        Ok(int2_storage {
            x,
            y,
        })
    }
}
