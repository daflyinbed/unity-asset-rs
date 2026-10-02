//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PPtrKeyframe {
    pub time: f32,
    pub value: PPtr,
}

impl crate::Parse for PPtrKeyframe {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = rdr.read_f32()?;
        let value = crate::PPtr::parse(rdr, ver)?;

        Ok(PPtrKeyframe { time, value })
    }
}

impl crate::ToValue for PPtrKeyframe {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("time".to_string(), self.time.to_value())),
            Some(("value".to_string(), self.value.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "time" => (self.time).inspect_value(path, offset, limit),
                "value" => (self.value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.time).inspect_value(path, offset, limit)).map(|v| ("time".to_string(), v)),
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
