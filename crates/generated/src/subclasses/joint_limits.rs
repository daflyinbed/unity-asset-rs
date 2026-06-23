//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointLimits {
    pub min: f32,
    pub max: f32,
    pub min_bounce: f32,
    pub max_bounce: f32,
}

impl crate::Parse for JointLimits {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let min = r.read_f32()?;
        let max = r.read_f32()?;
        let min_bounce = r.read_f32()?;
        let max_bounce = r.read_f32()?;

        Ok(JointLimits {
            min,
            max,
            min_bounce,
            max_bounce,
        })
    }
}
