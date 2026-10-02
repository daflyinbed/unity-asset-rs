//! 自动生成。请勿手动修改。

use crate::subclasses::ticks_per_second::TicksPerSecond;

#[derive(Debug, Clone)]
pub struct RationalTime {
    pub m_count: i64,
    pub m_rate: TicksPerSecond,
}

impl crate::Parse for RationalTime {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_count = rdr.read_i64()?;
        let m_rate = TicksPerSecond::parse(rdr, ver)?;

        Ok(RationalTime { m_count, m_rate })
    }
}

impl crate::ToValue for RationalTime {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Count".to_string(), self.m_count.to_value())),
            Some(("m_Rate".to_string(), self.m_rate.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Count" => (self.m_count).inspect_value(path, offset, limit),
                "m_Rate" => (self.m_rate).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_count).inspect_value(path, offset, limit)).map(|v| ("m_Count".to_string(), v)),
            ((self.m_rate).inspect_value(path, offset, limit)).map(|v| ("m_Rate".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
