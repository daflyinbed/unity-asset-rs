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

impl crate::ToValue for TestObjectWithSerializedAnimationCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("m_Curve".to_string(), self.m_curve.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Curve" => (self.m_curve).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_curve).inspect_value(path, offset, limit)).map(|v| ("m_Curve".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
