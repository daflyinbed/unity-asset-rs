//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ColorRgbAf {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl crate::Parse for ColorRgbAf {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r = rdr.read_f32()?;
        let g = rdr.read_f32()?;
        let b = rdr.read_f32()?;
        let a = rdr.read_f32()?;

        Ok(ColorRgbAf { r, g, b, a })
    }
}
