//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct CollisionModule {
    pub enabled: bool,
    pub r#type: i32,
    pub plane0: PPtr,
    pub plane1: PPtr,
    pub plane2: PPtr,
    pub plane3: PPtr,
    pub plane4: PPtr,
    pub plane5: PPtr,
    pub dampen: f32,
    pub bounce: f32,
    pub energy_loss_on_collision: f32,
    pub min_kill_speed: f32,
}

impl crate::Parse for CollisionModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let r#type = r.read_i32()?;
        let plane0 = crate::PPtr::parse(r, ver)?;
        let plane1 = crate::PPtr::parse(r, ver)?;
        let plane2 = crate::PPtr::parse(r, ver)?;
        let plane3 = crate::PPtr::parse(r, ver)?;
        let plane4 = crate::PPtr::parse(r, ver)?;
        let plane5 = crate::PPtr::parse(r, ver)?;
        let dampen = r.read_f32()?;
        let bounce = r.read_f32()?;
        let energy_loss_on_collision = r.read_f32()?;
        let min_kill_speed = r.read_f32()?;

        Ok(CollisionModule {
            enabled,
            r#type,
            plane0,
            plane1,
            plane2,
            plane3,
            plane4,
            plane5,
            dampen,
            bounce,
            energy_loss_on_collision,
            min_kill_speed,
        })
    }
}
