//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::min_max_gradient::MinMaxGradient;

#[derive(Debug, Clone)]
pub struct CustomDataModule {
    pub enabled: bool,
    pub mode0: i32,
    pub vector_component_count0: i32,
    pub color0: MinMaxGradient,
    pub vector0_0: MinMaxCurve,
    pub vector0_1: MinMaxCurve,
    pub vector0_2: MinMaxCurve,
    pub vector0_3: MinMaxCurve,
    pub mode1: i32,
    pub vector_component_count1: i32,
    pub color1: MinMaxGradient,
    pub vector1_0: MinMaxCurve,
    pub vector1_1: MinMaxCurve,
    pub vector1_2: MinMaxCurve,
    pub vector1_3: MinMaxCurve,
}

impl crate::Parse for CustomDataModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let mode0 = r.read_i32()?;
        let vector_component_count0 = r.read_i32()?;
        let color0 = MinMaxGradient::parse(r, ver)?;
        r.align(4)?;
        let vector0_0 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let vector0_1 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let vector0_2 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let vector0_3 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let mode1 = r.read_i32()?;
        let vector_component_count1 = r.read_i32()?;
        let color1 = MinMaxGradient::parse(r, ver)?;
        r.align(4)?;
        let vector1_0 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let vector1_1 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let vector1_2 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let vector1_3 = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;

        Ok(CustomDataModule {
            enabled,
            mode0,
            vector_component_count0,
            color0,
            vector0_0,
            vector0_1,
            vector0_2,
            vector0_3,
            mode1,
            vector_component_count1,
            color1,
            vector1_0,
            vector1_1,
            vector1_2,
            vector1_3,
        })
    }
}
