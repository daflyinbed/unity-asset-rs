//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl crate::Parse for Vector3f {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x = r.read_f32()?;
        let y = r.read_f32()?;
        let z = r.read_f32()?;

        Ok(Vector3f {
            x,
            y,
            z,
        })
    }
}
