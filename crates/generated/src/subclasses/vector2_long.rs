//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Vector2Long {
    pub x: i64,
    pub y: i64,
}

impl crate::Parse for Vector2Long {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_i64()?;
        let y = rdr.read_i64()?;

        Ok(Vector2Long {
            x,
            y,
        })
    }
}
