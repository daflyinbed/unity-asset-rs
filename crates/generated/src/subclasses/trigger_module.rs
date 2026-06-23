//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TriggerModule {
    pub enabled: bool,
    pub collision_shape0: PPtr,
    pub collision_shape1: PPtr,
    pub collision_shape2: PPtr,
    pub collision_shape3: PPtr,
    pub collision_shape4: PPtr,
    pub collision_shape5: PPtr,
    pub inside: i32,
    pub outside: i32,
    pub enter: i32,
    pub exit: i32,
    pub radius_scale: f32,
}

impl crate::Parse for TriggerModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let collision_shape0 = crate::PPtr::parse(r, ver)?;
        let collision_shape1 = crate::PPtr::parse(r, ver)?;
        let collision_shape2 = crate::PPtr::parse(r, ver)?;
        let collision_shape3 = crate::PPtr::parse(r, ver)?;
        let collision_shape4 = crate::PPtr::parse(r, ver)?;
        let collision_shape5 = crate::PPtr::parse(r, ver)?;
        let inside = r.read_i32()?;
        let outside = r.read_i32()?;
        let enter = r.read_i32()?;
        let exit = r.read_i32()?;
        let radius_scale = r.read_f32()?;

        Ok(TriggerModule {
            enabled,
            collision_shape0,
            collision_shape1,
            collision_shape2,
            collision_shape3,
            collision_shape4,
            collision_shape5,
            inside,
            outside,
            enter,
            exit,
            radius_scale,
        })
    }
}
