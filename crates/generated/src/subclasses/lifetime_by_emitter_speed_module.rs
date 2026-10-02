//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct LifetimeByEmitterSpeedModule {
    pub enabled: bool,
    pub m_curve: MinMaxCurve,
    pub m_range: Vector2f,
}

impl crate::Parse for LifetimeByEmitterSpeedModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_curve = MinMaxCurve::parse(rdr, ver)?;
        let m_range = Vector2f::parse(rdr, ver)?;

        Ok(LifetimeByEmitterSpeedModule {
            enabled,
            m_curve,
            m_range,
        })
    }
}

impl crate::ToValue for LifetimeByEmitterSpeedModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("m_Curve".to_string(), self.m_curve.to_value())),
            Some(("m_Range".to_string(), self.m_range.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "m_Curve" => (self.m_curve).inspect_value(path, offset, limit),
                "m_Range" => (self.m_range).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.m_curve).inspect_value(path, offset, limit)).map(|v| ("m_Curve".to_string(), v)),
            ((self.m_range).inspect_value(path, offset, limit)).map(|v| ("m_Range".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
