//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Rectf {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl crate::Parse for Rectf {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_f32()?;
        let y = rdr.read_f32()?;
        let width = rdr.read_f32()?;
        let height = rdr.read_f32()?;

        Ok(Rectf {
            x,
            y,
            width,
            height,
        })
    }
}
