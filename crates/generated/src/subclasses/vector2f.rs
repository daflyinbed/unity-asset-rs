//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl crate::Parse for Vector2f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_f32()?;
        let y = rdr.read_f32()?;

        Ok(Vector2f { x, y })
    }
}
