//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UvAnimation {
    pub x_tile: i32,
    pub y_tile: i32,
    pub cycles: f32,
}

impl crate::Parse for UvAnimation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x_tile = rdr.read_i32()?;
        let y_tile = rdr.read_i32()?;
        let cycles = rdr.read_f32()?;

        Ok(UvAnimation {
            x_tile,
            y_tile,
            cycles,
        })
    }
}
