//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SoftJointLimit {
    pub limit: f32,
    pub bounciness: f32,
    pub spring: Option<f32>,
    pub contact_distance: Option<f32>,
    pub damper: Option<f32>,
}

impl crate::Parse for SoftJointLimit {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let limit = rdr.read_f32()?;
        let bounciness = rdr.read_f32()?;
        let spring = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let contact_distance = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let damper = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(SoftJointLimit {
            limit,
            bounciness,
            spring,
            contact_distance,
            damper,
        })
    }
}
