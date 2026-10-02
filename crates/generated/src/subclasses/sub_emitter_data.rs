//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubEmitterData {
    pub emitter: PPtr,
    pub r#type: i32,
    pub properties: i32,
    pub emit_probability: Option<f32>,
}

impl crate::Parse for SubEmitterData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let emitter = crate::PPtr::parse(rdr, ver)?;
        let r#type = rdr.read_i32()?;
        let properties = rdr.read_i32()?;
        let emit_probability = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(SubEmitterData {
            emitter,
            r#type,
            properties,
            emit_probability,
        })
    }
}

impl crate::ToValue for SubEmitterData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("emitter".to_string(), self.emitter.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("properties".to_string(), self.properties.to_value())),
            crate::opt_field("emitProbability", &self.emit_probability),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "emitter" => (self.emitter).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "properties" => (self.properties).inspect_value(path, offset, limit),
                "emitProbability" => (self.emit_probability).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.emitter).inspect_value(path, offset, limit)).map(|v| ("emitter".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.properties).inspect_value(path, offset, limit))
                .map(|v| ("properties".to_string(), v)),
            ((self.emit_probability).inspect_value(path, offset, limit))
                .map(|v| ("emitProbability".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
