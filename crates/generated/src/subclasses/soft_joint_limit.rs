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
        let mut limit = None;
        let mut bounciness = None;
        let mut spring = None;
        let mut contact_distance = None;
        let mut damper = None;
        limit = Some(rdr.read_f32()?);
        bounciness = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            contact_distance = Some(rdr.read_f32()?);
        } else {
            spring = Some(rdr.read_f32()?);
            damper = Some(rdr.read_f32()?);
        }

        Ok(SoftJointLimit {
            limit: limit.unwrap(),
            bounciness: bounciness.unwrap(),
            spring,
            contact_distance,
            damper,
        })
    }
}
