//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointMotor {
    pub target_velocity: f32,
    pub force: f32,
    pub free_spin: i32,
}

impl crate::Parse for JointMotor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_velocity = rdr.read_f32()?;
        let force = rdr.read_f32()?;
        let free_spin = rdr.read_i32()?;

        Ok(JointMotor {
            target_velocity,
            force,
            free_spin,
        })
    }
}
