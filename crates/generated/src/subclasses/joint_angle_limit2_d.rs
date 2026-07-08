//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointAngleLimit2D {
    pub m_lower_angle: f32,
    pub m_upper_angle: f32,
}

impl crate::Parse for JointAngleLimit2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lower_angle = rdr.read_f32()?;
        let m_upper_angle = rdr.read_f32()?;

        Ok(JointAngleLimit2D {
            m_lower_angle,
            m_upper_angle,
        })
    }
}
