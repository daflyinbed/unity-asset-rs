//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct InheritVelocityModule {
    pub enabled: bool,
    pub m_mode: i32,
    pub m_curve: MinMaxCurve,
}

impl crate::Parse for InheritVelocityModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_mode = rdr.read_i32()?;
        let m_curve = MinMaxCurve::parse(rdr, ver)?;

        Ok(InheritVelocityModule {
            enabled,
            m_mode,
            m_curve,
        })
    }
}

impl crate::ToValue for InheritVelocityModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("m_Mode".to_string(), self.m_mode.to_value())),
            Some(("m_Curve".to_string(), self.m_curve.to_value())),
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
                "m_Mode" => (self.m_mode).inspect_value(path, offset, limit),
                "m_Curve" => (self.m_curve).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.m_mode).inspect_value(path, offset, limit)).map(|v| ("m_Mode".to_string(), v)),
            ((self.m_curve).inspect_value(path, offset, limit)).map(|v| ("m_Curve".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
