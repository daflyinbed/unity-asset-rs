//! 自动生成。请勿手动修改。

use crate::subclasses::device_none::DeviceNone;
use crate::subclasses::google::Google;

#[derive(Debug, Clone)]
pub struct VrSettings {
    pub none: DeviceNone,
    pub cardboard: Google,
    pub daydream: Google,
}

impl crate::Parse for VrSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let none = DeviceNone::parse(r, ver)?;
        r.align(4)?;
        let cardboard = Google::parse(r, ver)?;
        r.align(4)?;
        let daydream = Google::parse(r, ver)?;
        r.align(4)?;

        Ok(VrSettings {
            none,
            cardboard,
            daydream,
        })
    }
}
