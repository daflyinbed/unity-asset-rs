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
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            start_frame = Some(MinMaxCurve::parse(rdr, ver)?);
            speed_range = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            start_frame = Some(MinMaxCurve::parse(rdr, ver)?);
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
            sprites = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
            sprites = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            uv_channel_mask = Some(rdr.read_i32()?);
            flip_u = Some(rdr.read_f32()?);
            flip_v = Some(rdr.read_f32()?);
            random_row = Some(rdr.read_bool()?);
            rdr.align(4)?;
            sprites = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for UvModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("frameOverTime".to_string(), self.frame_over_time.to_value())),
            crate::opt_field("mode", &self.mode),
            Some(("tilesX".to_string(), self.tiles_x.to_value())),
            crate::opt_field("startFrame", &self.start_frame),
            crate::opt_field("timeMode", &self.time_mode),
            Some(("tilesY".to_string(), self.tiles_y.to_value())),
            crate::opt_field("fps", &self.fps),
            Some(("animationType".to_string(), self.animation_type.to_value())),
            Some(("rowIndex".to_string(), self.row_index.to_value())),
            Some(("cycles".to_string(), self.cycles.to_value())),
            crate::opt_field("speedRange", &self.speed_range),
            crate::opt_field("randomRow", &self.random_row),
            crate::opt_field("uvChannelMask", &self.uv_channel_mask),
            crate::opt_field("flipU", &self.flip_u),
            crate::opt_field("flipV", &self.flip_v),
            crate::opt_field("sprites", &self.sprites),
            crate::opt_field("rowMode", &self.row_mode),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "frameOverTime" => (self.frame_over_time).inspect_value(path, offset, limit),
                "mode" => (self.mode).inspect_value(path, offset, limit),
                "tilesX" => (self.tiles_x).inspect_value(path, offset, limit),
                "startFrame" => (self.start_frame).inspect_value(path, offset, limit),
                "timeMode" => (self.time_mode).inspect_value(path, offset, limit),
                "tilesY" => (self.tiles_y).inspect_value(path, offset, limit),
                "fps" => (self.fps).inspect_value(path, offset, limit),
                "animationType" => (self.animation_type).inspect_value(path, offset, limit),
                "rowIndex" => (self.row_index).inspect_value(path, offset, limit),
                "cycles" => (self.cycles).inspect_value(path, offset, limit),
                "speedRange" => (self.speed_range).inspect_value(path, offset, limit),
                "randomRow" => (self.random_row).inspect_value(path, offset, limit),
                "uvChannelMask" => (self.uv_channel_mask).inspect_value(path, offset, limit),
                "flipU" => (self.flip_u).inspect_value(path, offset, limit),
                "flipV" => (self.flip_v).inspect_value(path, offset, limit),
                "sprites" => (self.sprites).inspect_value(path, offset, limit),
                "rowMode" => (self.row_mode).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.frame_over_time).inspect_value(path, offset, limit))
                .map(|v| ("frameOverTime".to_string(), v)),
            ((self.mode).inspect_value(path, offset, limit)).map(|v| ("mode".to_string(), v)),
            ((self.tiles_x).inspect_value(path, offset, limit)).map(|v| ("tilesX".to_string(), v)),
            ((self.start_frame).inspect_value(path, offset, limit))
                .map(|v| ("startFrame".to_string(), v)),
            ((self.time_mode).inspect_value(path, offset, limit))
                .map(|v| ("timeMode".to_string(), v)),
            ((self.tiles_y).inspect_value(path, offset, limit)).map(|v| ("tilesY".to_string(), v)),
            ((self.fps).inspect_value(path, offset, limit)).map(|v| ("fps".to_string(), v)),
            ((self.animation_type).inspect_value(path, offset, limit))
                .map(|v| ("animationType".to_string(), v)),
            ((self.row_index).inspect_value(path, offset, limit))
                .map(|v| ("rowIndex".to_string(), v)),
            ((self.cycles).inspect_value(path, offset, limit)).map(|v| ("cycles".to_string(), v)),
            ((self.speed_range).inspect_value(path, offset, limit))
                .map(|v| ("speedRange".to_string(), v)),
            ((self.random_row).inspect_value(path, offset, limit))
                .map(|v| ("randomRow".to_string(), v)),
            ((self.uv_channel_mask).inspect_value(path, offset, limit))
                .map(|v| ("uvChannelMask".to_string(), v)),
            ((self.flip_u).inspect_value(path, offset, limit)).map(|v| ("flipU".to_string(), v)),
            ((self.flip_v).inspect_value(path, offset, limit)).map(|v| ("flipV".to_string(), v)),
            ((self.sprites).inspect_value(path, offset, limit)).map(|v| ("sprites".to_string(), v)),
            ((self.row_mode).inspect_value(path, offset, limit))
                .map(|v| ("rowMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
