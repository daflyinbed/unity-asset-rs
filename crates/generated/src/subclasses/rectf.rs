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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = r.read_f32()?;
        let y = r.read_f32()?;
        let width = r.read_f32()?;
        let height = r.read_f32()?;

        Ok(Rectf {
            x,
            y,
            width,
            height,
        })
    }
}
