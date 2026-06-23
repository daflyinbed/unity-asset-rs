//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Matrix4x4f {
    pub e00: f32,
    pub e01: f32,
    pub e02: f32,
    pub e03: f32,
    pub e10: f32,
    pub e11: f32,
    pub e12: f32,
    pub e13: f32,
    pub e20: f32,
    pub e21: f32,
    pub e22: f32,
    pub e23: f32,
    pub e30: f32,
    pub e31: f32,
    pub e32: f32,
    pub e33: f32,
}

impl crate::Parse for Matrix4x4f {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let e00 = r.read_f32()?;
        let e01 = r.read_f32()?;
        let e02 = r.read_f32()?;
        let e03 = r.read_f32()?;
        let e10 = r.read_f32()?;
        let e11 = r.read_f32()?;
        let e12 = r.read_f32()?;
        let e13 = r.read_f32()?;
        let e20 = r.read_f32()?;
        let e21 = r.read_f32()?;
        let e22 = r.read_f32()?;
        let e23 = r.read_f32()?;
        let e30 = r.read_f32()?;
        let e31 = r.read_f32()?;
        let e32 = r.read_f32()?;
        let e33 = r.read_f32()?;

        Ok(Matrix4x4f {
            e00,
            e01,
            e02,
            e03,
            e10,
            e11,
            e12,
            e13,
            e20,
            e21,
            e22,
            e23,
            e30,
            e31,
            e32,
            e33,
        })
    }
}
