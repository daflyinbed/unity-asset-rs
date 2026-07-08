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
