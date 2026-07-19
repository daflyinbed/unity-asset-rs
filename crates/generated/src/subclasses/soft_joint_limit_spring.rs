//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SoftJointLimitSpring {
    pub spring: f32,
    pub damper: f32,
}

impl crate::Parse for SoftJointLimitSpring {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let spring = rdr.read_f32()?;
        let damper = rdr.read_f32()?;

        Ok(SoftJointLimitSpring { spring, damper })
    }
}
