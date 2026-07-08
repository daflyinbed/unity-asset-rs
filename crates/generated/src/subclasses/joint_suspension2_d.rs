//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointSuspension2D {
    pub m_damping_ratio: f32,
    pub m_frequency: f32,
    pub m_angle: f32,
}

impl crate::Parse for JointSuspension2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_damping_ratio = rdr.read_f32()?;
        let m_frequency = rdr.read_f32()?;
        let m_angle = rdr.read_f32()?;

        Ok(JointSuspension2D {
            m_damping_ratio,
            m_frequency,
            m_angle,
        })
    }
}
