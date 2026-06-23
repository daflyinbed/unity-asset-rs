//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedAnimationCurve {
    pub parent: Object,
    pub m_curve: AnimationCurve,
}

impl crate::Parse for TestObjectWithSerializedAnimationCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_curve = AnimationCurve::parse(r, ver)?;
        r.align(4)?;

        Ok(TestObjectWithSerializedAnimationCurve {
            parent,
            m_curve,
        })
    }
}
