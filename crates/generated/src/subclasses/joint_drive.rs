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
        let mut mode = None;
        let mut position_spring = None;
        let mut position_damper = None;
        let mut maximum_force = None;
        let mut use_acceleration = None;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            mode = Some(rdr.read_i32()?);
        }
        position_spring = Some(rdr.read_f32()?);
        position_damper = Some(rdr.read_f32()?);
        maximum_force = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            use_acceleration = Some(rdr.read_i32()?);
        } else {
        }

        Ok(JointDrive {
            mode,
            position_spring: position_spring.unwrap(),
            position_damper: position_damper.unwrap(),
            maximum_force: maximum_force.unwrap(),
            use_acceleration,
        })
    }
}
