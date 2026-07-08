//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub in_slope: f32,
    pub out_slope: f32,
    pub weighted_mode: Option<i32>,
    pub in_weight: Option<f32>,
    pub out_weight: Option<f32>,
}

impl crate::Parse for Keyframe {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = rdr.read_f32()?;
        let value = rdr.read_f32()?;
        let in_slope = rdr.read_f32()?;
        let out_slope = rdr.read_f32()?;
        let weighted_mode = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let in_weight = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let out_weight = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(Keyframe {
            time,
            value,
            in_slope,
            out_slope,
            weighted_mode,
            in_weight,
            out_weight,
        })
    }
}
