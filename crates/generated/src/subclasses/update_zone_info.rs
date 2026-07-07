//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct UpdateZoneInfo {
    pub update_zone_center: Vector3f,
    pub update_zone_size: Vector3f,
    pub rotation: f32,
    pub pass_index: i32,
    pub need_swap: bool,
}

impl crate::Parse for UpdateZoneInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let update_zone_center = Vector3f::parse(r, ver)?;
        let update_zone_size = Vector3f::parse(r, ver)?;
        let rotation = r.read_f32()?;
        let pass_index = r.read_i32()?;
        let need_swap = r.read_bool()?;
        r.align(4)?;

        Ok(UpdateZoneInfo {
            update_zone_center,
            update_zone_size,
            rotation,
            pass_index,
            need_swap,
        })
    }
}
