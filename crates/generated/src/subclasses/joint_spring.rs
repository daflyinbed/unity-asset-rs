//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointSpring {
    pub spring: f32,
    pub damper: f32,
    pub target_position: f32,
}

impl crate::Parse for JointSpring {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let spring = rdr.read_f32()?;
        let damper = rdr.read_f32()?;
        let target_position = rdr.read_f32()?;

        Ok(JointSpring {
            spring,
            damper,
            target_position,
        })
    }
}
