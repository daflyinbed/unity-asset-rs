//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointMotor2D {
    pub m_motor_speed: f32,
    pub m_maximum_motor_force: f32,
}

impl crate::Parse for JointMotor2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_motor_speed = r.read_f32()?;
        let m_maximum_motor_force = r.read_f32()?;

        Ok(JointMotor2D {
            m_motor_speed,
            m_maximum_motor_force,
        })
    }
}
