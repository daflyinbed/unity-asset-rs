//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl crate::Parse for Vector4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = rdr.read_f32()?;
        let y = rdr.read_f32()?;
        let z = rdr.read_f32()?;
        let w = rdr.read_f32()?;

        Ok(Vector4f {
            x,
            y,
            z,
            w,
        })
    }
}
