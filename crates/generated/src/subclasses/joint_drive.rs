//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointDrive {
    pub mode: Option<i32>,
    pub position_spring: f32,
    pub position_damper: f32,
    pub maximum_force: f32,
    pub use_acceleration: Option<i32>,
}

impl crate::Parse for JointDrive {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mode = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let position_spring = rdr.read_f32()?;
        let position_damper = rdr.read_f32()?;
        let maximum_force = rdr.read_f32()?;
        let use_acceleration = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(JointDrive {
            mode,
            position_spring,
            position_damper,
            maximum_force,
            use_acceleration,
        })
    }
}
