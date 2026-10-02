//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AnimationEvent {
    pub time: f32,
    pub function_name: String,
    pub data: String,
    pub object_reference_parameter: PPtr,
    pub float_parameter: f32,
    pub int_parameter: i32,
    pub message_options: i32,
}

impl crate::Parse for AnimationEvent {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = rdr.read_f32()?;
        let function_name = rdr.read_aligned_string()?;
        let data = rdr.read_aligned_string()?;
        let object_reference_parameter = crate::PPtr::parse(rdr, ver)?;
        let float_parameter = rdr.read_f32()?;
        let int_parameter = rdr.read_i32()?;
        let message_options = rdr.read_i32()?;

        Ok(AnimationEvent {
            time,
            function_name,
            data,
            object_reference_parameter,
            float_parameter,
            int_parameter,
            message_options,
        })
    }
}

impl crate::ToValue for AnimationEvent {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("time".to_string(), self.time.to_value())),
            Some(("functionName".to_string(), self.function_name.to_value())),
            Some(("data".to_string(), self.data.to_value())),
            Some((
                "objectReferenceParameter".to_string(),
                self.object_reference_parameter.to_value(),
            )),
            Some((
                "floatParameter".to_string(),
                self.float_parameter.to_value(),
            )),
            Some(("intParameter".to_string(), self.int_parameter.to_value())),
            Some((
                "messageOptions".to_string(),
                self.message_options.to_value(),
            )),
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
                "functionName" => (self.function_name).inspect_value(path, offset, limit),
                "data" => (self.data).inspect_value(path, offset, limit),
                "objectReferenceParameter" => {
                    (self.object_reference_parameter).inspect_value(path, offset, limit)
                }
                "floatParameter" => (self.float_parameter).inspect_value(path, offset, limit),
                "intParameter" => (self.int_parameter).inspect_value(path, offset, limit),
                "messageOptions" => (self.message_options).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.time).inspect_value(path, offset, limit)).map(|v| ("time".to_string(), v)),
            ((self.function_name).inspect_value(path, offset, limit))
                .map(|v| ("functionName".to_string(), v)),
            ((self.data).inspect_value(path, offset, limit)).map(|v| ("data".to_string(), v)),
            ((self.object_reference_parameter).inspect_value(path, offset, limit))
                .map(|v| ("objectReferenceParameter".to_string(), v)),
            ((self.float_parameter).inspect_value(path, offset, limit))
                .map(|v| ("floatParameter".to_string(), v)),
            ((self.int_parameter).inspect_value(path, offset, limit))
                .map(|v| ("intParameter".to_string(), v)),
            ((self.message_options).inspect_value(path, offset, limit))
                .map(|v| ("messageOptions".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
