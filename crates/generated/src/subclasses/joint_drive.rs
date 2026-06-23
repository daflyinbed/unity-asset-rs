//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointDrive {
    pub mode: i32,
    pub position_spring: f32,
    pub position_damper: f32,
    pub maximum_force: f32,
}

impl crate::Parse for JointDrive {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mode = r.read_i32()?;
        let position_spring = r.read_f32()?;
        let position_damper = r.read_f32()?;
        let maximum_force = r.read_f32()?;

        Ok(JointDrive {
            mode,
            position_spring,
            position_damper,
            maximum_force,
        })
    }
}
