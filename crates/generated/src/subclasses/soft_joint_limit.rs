//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SoftJointLimit {
    pub limit: f32,
    pub bounciness: f32,
    pub spring: f32,
    pub damper: f32,
}

impl crate::Parse for SoftJointLimit {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let limit = r.read_f32()?;
        let bounciness = r.read_f32()?;
        let spring = r.read_f32()?;
        let damper = r.read_f32()?;

        Ok(SoftJointLimit {
            limit,
            bounciness,
            spring,
            damper,
        })
    }
}
