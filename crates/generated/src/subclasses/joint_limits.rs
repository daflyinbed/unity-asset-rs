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
        let min = rdr.read_f32()?;
        let max = rdr.read_f32()?;
        let min_bounce = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let bounciness = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let max_bounce = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let bounce_min_velocity = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let contact_distance = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(JointLimits {
            min,
            max,
            min_bounce,
            bounciness,
            max_bounce,
            bounce_min_velocity,
            contact_distance,
        })
    }
}
