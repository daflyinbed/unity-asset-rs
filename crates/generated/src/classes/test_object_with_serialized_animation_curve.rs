//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedAnimationCurve {
    pub m_curve: AnimationCurve,
}

impl crate::Parse for TestObjectWithSerializedAnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_curve = AnimationCurve::parse(rdr, ver)?;

        Ok(TestObjectWithSerializedAnimationCurve { m_curve })
    }
}
