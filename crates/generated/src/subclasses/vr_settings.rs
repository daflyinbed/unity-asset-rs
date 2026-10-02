//! 自动生成。请勿手动修改。

use crate::subclasses::device_none::DeviceNone;
use crate::subclasses::google::Google;
use crate::subclasses::holo_lens::HoloLens;
use crate::subclasses::lumin::Lumin;
use crate::subclasses::oculus::Oculus;

#[derive(Debug, Clone)]
pub struct VrSettings {
    pub none: Option<DeviceNone>,
    pub enable360_stereo_capture: Option<bool>,
    pub cardboard: Option<Google>,
    pub daydream: Option<Google>,
    pub hololens: Option<HoloLens>,
    pub oculus: Option<Oculus>,
    pub lumin: Option<Lumin>,
}

impl crate::Parse for VrSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut none = None;
        let mut enable360_stereo_capture = None;
        let mut cardboard = None;
        let mut daydream = None;
        let mut hololens = None;
        let mut oculus = None;
        let mut lumin = None;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            enable360_stereo_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            none = Some(DeviceNone::parse(rdr, ver)?);
            rdr.align(4)?;
            cardboard = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            daydream = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            hololens = Some(HoloLens::parse(rdr, ver)?);
            rdr.align(4)?;
            lumin = Some(Lumin::parse(rdr, ver)?);
            rdr.align(4)?;
            oculus = Some(Oculus::parse(rdr, ver)?);
            rdr.align(4)?;
            enable360_stereo_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            none = Some(DeviceNone::parse(rdr, ver)?);
            rdr.align(4)?;
            cardboard = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            daydream = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            hololens = Some(HoloLens::parse(rdr, ver)?);
            rdr.align(4)?;
            oculus = Some(Oculus::parse(rdr, ver)?);
            rdr.align(4)?;
            enable360_stereo_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            none = Some(DeviceNone::parse(rdr, ver)?);
            rdr.align(4)?;
            cardboard = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            daydream = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            hololens = Some(HoloLens::parse(rdr, ver)?);
            rdr.align(4)?;
            enable360_stereo_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            oculus = Some(Oculus::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            none = Some(DeviceNone::parse(rdr, ver)?);
            rdr.align(4)?;
            cardboard = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            daydream = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            hololens = Some(HoloLens::parse(rdr, ver)?);
            rdr.align(4)?;
            oculus = Some(Oculus::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
            none = Some(DeviceNone::parse(rdr, ver)?);
            rdr.align(4)?;
            cardboard = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            daydream = Some(Google::parse(rdr, ver)?);
            rdr.align(4)?;
            hololens = Some(HoloLens::parse(rdr, ver)?);
            rdr.align(4)?;
        }

        Ok(VrSettings {
            none,
            enable360_stereo_capture,
            cardboard,
            daydream,
            hololens,
            oculus,
            lumin,
        })
    }
}

impl crate::ToValue for VrSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("none", &self.none),
            crate::opt_field("enable360StereoCapture", &self.enable360_stereo_capture),
            crate::opt_field("cardboard", &self.cardboard),
            crate::opt_field("daydream", &self.daydream),
            crate::opt_field("hololens", &self.hololens),
            crate::opt_field("oculus", &self.oculus),
            crate::opt_field("lumin", &self.lumin),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "none" => (self.none).inspect_value(path, offset, limit),
                "enable360StereoCapture" => {
                    (self.enable360_stereo_capture).inspect_value(path, offset, limit)
                }
                "cardboard" => (self.cardboard).inspect_value(path, offset, limit),
                "daydream" => (self.daydream).inspect_value(path, offset, limit),
                "hololens" => (self.hololens).inspect_value(path, offset, limit),
                "oculus" => (self.oculus).inspect_value(path, offset, limit),
                "lumin" => (self.lumin).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.none).inspect_value(path, offset, limit)).map(|v| ("none".to_string(), v)),
            ((self.enable360_stereo_capture).inspect_value(path, offset, limit))
                .map(|v| ("enable360StereoCapture".to_string(), v)),
            ((self.cardboard).inspect_value(path, offset, limit))
                .map(|v| ("cardboard".to_string(), v)),
            ((self.daydream).inspect_value(path, offset, limit))
                .map(|v| ("daydream".to_string(), v)),
            ((self.hololens).inspect_value(path, offset, limit))
                .map(|v| ("hololens".to_string(), v)),
            ((self.oculus).inspect_value(path, offset, limit)).map(|v| ("oculus".to_string(), v)),
            ((self.lumin).inspect_value(path, offset, limit)).map(|v| ("lumin".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
