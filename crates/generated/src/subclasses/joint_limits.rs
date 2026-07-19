//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointLimits {
    pub min: f32,
    pub max: f32,
    pub min_bounce: Option<f32>,
    pub bounciness: Option<f32>,
    pub max_bounce: Option<f32>,
    pub bounce_min_velocity: Option<f32>,
    pub contact_distance: Option<f32>,
}

impl crate::Parse for JointLimits {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut min = None;
        let mut max = None;
        let mut min_bounce = None;
        let mut bounciness = None;
        let mut max_bounce = None;
        let mut bounce_min_velocity = None;
        let mut contact_distance = None;
        min = Some(rdr.read_f32()?);
        max = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            bounciness = Some(rdr.read_f32()?);
            bounce_min_velocity = Some(rdr.read_f32()?);
            contact_distance = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            min_bounce = Some(rdr.read_f32()?);
            max_bounce = Some(rdr.read_f32()?);
            contact_distance = Some(rdr.read_f32()?);
        } else {
            min_bounce = Some(rdr.read_f32()?);
            max_bounce = Some(rdr.read_f32()?);
        }

        Ok(JointLimits {
            min: min.unwrap(),
            max: max.unwrap(),
            min_bounce,
            bounciness,
            max_bounce,
            bounce_min_velocity,
            contact_distance,
        })
    }
}
