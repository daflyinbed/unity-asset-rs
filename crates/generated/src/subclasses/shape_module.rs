//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ShapeModule {
    pub enabled: bool,
    pub r#type: i32,
    pub radius: f32,
    pub angle: f32,
    pub box_x: f32,
    pub box_y: f32,
    pub box_z: f32,
    pub placement_mode: i32,
    pub m_mesh: PPtr,
    pub random_direction: bool,
}

impl crate::Parse for ShapeModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let r#type = r.read_i32()?;
        let radius = r.read_f32()?;
        let angle = r.read_f32()?;
        let box_x = r.read_f32()?;
        let box_y = r.read_f32()?;
        let box_z = r.read_f32()?;
        let placement_mode = r.read_i32()?;
        let m_mesh = crate::PPtr::parse(r, ver)?;
        let random_direction = r.read_bool()?;
        r.align(4)?;

        Ok(ShapeModule {
            enabled,
            r#type,
            radius,
            angle,
            box_x,
            box_y,
            box_z,
            placement_mode,
            m_mesh,
            random_direction,
        })
    }
}
