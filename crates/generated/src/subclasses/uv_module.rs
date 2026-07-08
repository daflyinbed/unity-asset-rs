//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::sprite_data::SpriteData;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct UvModule {
    pub enabled: bool,
    pub frame_over_time: MinMaxCurve,
    pub mode: Option<i32>,
    pub tiles_x: i32,
    pub start_frame: Option<MinMaxCurve>,
    pub time_mode: Option<i32>,
    pub tiles_y: i32,
    pub fps: Option<f32>,
    pub animation_type: i32,
    pub row_index: i32,
    pub cycles: f32,
    pub speed_range: Option<Vector2f>,
    pub random_row: Option<bool>,
    pub uv_channel_mask: Option<i32>,
    pub flip_u: Option<f32>,
    pub flip_v: Option<f32>,
    pub sprites: Option<Vec<SpriteData>>,
    pub row_mode: Option<i32>,
}

impl crate::Parse for UvModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut frame_over_time = None;
        let mut mode = None;
        let mut tiles_x = None;
        let mut start_frame = None;
        let mut time_mode = None;
        let mut tiles_y = None;
        let mut fps = None;
        let mut animation_type = None;
        let mut row_index = None;
        let mut cycles = None;
        let mut speed_range = None;
        let mut random_row = None;
        let mut uv_channel_mask = None;
        let mut flip_u = None;
        let mut flip_v = None;
        let mut sprites = None;
        let mut row_mode = None;
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            mode = Some(rdr.read_i32()?);
            time_mode = Some(rdr.read_i32()?);
            fps = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            mode = Some(rdr.read_i32()?);
        } else {
        }
            frame_over_time = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            start_frame = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
            speed_range = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            start_frame = Some(MinMaxCurve::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
        }
            tiles_x = Some(rdr.read_i32()?);
            tiles_y = Some(rdr.read_i32()?);
            animation_type = Some(rdr.read_i32()?);
            row_index = Some(rdr.read_i32()?);
            cycles = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            row_mode = Some(rdr.read_i32()?);
            sprites = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SpriteData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
            sprites = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SpriteData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
            sprites = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SpriteData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(UvModule {
            enabled: enabled.unwrap(),
            frame_over_time: frame_over_time.unwrap(),
            mode,
            tiles_x: tiles_x.unwrap(),
            start_frame,
            time_mode,
            tiles_y: tiles_y.unwrap(),
            fps,
            animation_type: animation_type.unwrap(),
            row_index: row_index.unwrap(),
            cycles: cycles.unwrap(),
            speed_range,
            random_row,
            uv_channel_mask,
            flip_u,
            flip_v,
            sprites,
            row_mode,
        })
    }
}
