//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct MinMaxCurve {
    pub scalar: f32,
    pub max_curve: AnimationCurve,
    pub min_curve: AnimationCurve,
    pub min_max_state: i16,
}

impl crate::Parse for MinMaxCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let scalar = r.read_f32()?;
        let max_curve = AnimationCurve::parse(r, ver)?;
        let min_curve = AnimationCurve::parse(r, ver)?;
        let min_max_state = r.read_i16()?;
        r.align(4)?;

        Ok(MinMaxCurve {
            scalar,
            max_curve,
            min_curve,
            min_max_state,
        })
    }
}
