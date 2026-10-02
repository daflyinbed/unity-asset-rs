//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_shader_float_value::SerializedShaderFloatValue;

#[derive(Debug, Clone)]
pub struct SerializedStencilOp {
    pub pass: SerializedShaderFloatValue,
    pub fail: SerializedShaderFloatValue,
    pub z_fail: SerializedShaderFloatValue,
    pub comp: SerializedShaderFloatValue,
}

impl crate::Parse for SerializedStencilOp {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let pass = SerializedShaderFloatValue::parse(rdr, ver)?;
        let fail = SerializedShaderFloatValue::parse(rdr, ver)?;
        let z_fail = SerializedShaderFloatValue::parse(rdr, ver)?;
        let comp = SerializedShaderFloatValue::parse(rdr, ver)?;

        Ok(SerializedStencilOp {
            pass,
            fail,
            z_fail,
            comp,
        })
    }
}

impl crate::ToValue for SerializedStencilOp {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("pass".to_string(), self.pass.to_value())),
            Some(("fail".to_string(), self.fail.to_value())),
            Some(("zFail".to_string(), self.z_fail.to_value())),
            Some(("comp".to_string(), self.comp.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "pass" => (self.pass).inspect_value(path, offset, limit),
                "fail" => (self.fail).inspect_value(path, offset, limit),
                "zFail" => (self.z_fail).inspect_value(path, offset, limit),
                "comp" => (self.comp).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.pass).inspect_value(path, offset, limit)).map(|v| ("pass".to_string(), v)),
            ((self.fail).inspect_value(path, offset, limit)).map(|v| ("fail".to_string(), v)),
            ((self.z_fail).inspect_value(path, offset, limit)).map(|v| ("zFail".to_string(), v)),
            ((self.comp).inspect_value(path, offset, limit)).map(|v| ("comp".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
