//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct UvModule {
    pub enabled: bool,
    pub frame_over_time: MinMaxCurve,
    pub tiles_x: i32,
    pub tiles_y: i32,
    pub animation_type: i32,
    pub row_index: i32,
    pub cycles: f32,
    pub random_row: bool,
}

impl crate::Parse for UvModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let frame_over_time = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let tiles_x = r.read_i32()?;
        let tiles_y = r.read_i32()?;
        let animation_type = r.read_i32()?;
        let row_index = r.read_i32()?;
        let cycles = r.read_f32()?;
        let random_row = r.read_bool()?;
        r.align(4)?;

        Ok(UvModule {
            enabled,
            frame_over_time,
            tiles_x,
            tiles_y,
            animation_type,
            row_index,
            cycles,
            random_row,
        })
    }
}
