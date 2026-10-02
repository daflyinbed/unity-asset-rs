//! 自动生成。请勿手动修改。

use crate::subclasses::property_modifications_target_test_native_object::PropertyModificationsTargetTestNativeObject;

#[derive(Debug, Clone)]
pub struct PropertyModificationsTargetTestObject {
    pub m_data: PropertyModificationsTargetTestNativeObject,
    pub m_array: Vec<PropertyModificationsTargetTestNativeObject>,
    pub m_float_test_value: f32,
    pub m_bytes: Option<Vec<u8>>,
    pub m_floats: Option<Vec<f32>>,
    pub m_bytes_size: Option<u32>,
    pub byte_data: Option<Vec<u8>>,
}

impl crate::Parse for PropertyModificationsTargetTestObject {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_data = PropertyModificationsTargetTestNativeObject::parse(rdr, ver)?;
        let m_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| PropertyModificationsTargetTestNativeObject::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_float_test_value = rdr.read_f32()?;
        let m_bytes = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_floats = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_bytes_size = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let byte_data = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                rdr.read_bytes(n)
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PropertyModificationsTargetTestObject {
            m_data,
            m_array,
            m_float_test_value,
            m_bytes,
            m_floats,
            m_bytes_size,
            byte_data,
        })
    }
}

impl crate::ToValue for PropertyModificationsTargetTestObject {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Data".to_string(), self.m_data.to_value())),
            Some(("m_Array".to_string(), self.m_array.to_value())),
            Some((
                "m_FloatTestValue".to_string(),
                self.m_float_test_value.to_value(),
            )),
            crate::opt_field("m_Bytes", &self.m_bytes),
            crate::opt_field("m_Floats", &self.m_floats),
            crate::opt_field("m_BytesSize", &self.m_bytes_size),
            self.byte_data
                .as_ref()
                .map(|v| ("byte data".to_string(), crate::Value::Bytes(v.clone()))),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Data" => (self.m_data).inspect_value(path, offset, limit),
                "m_Array" => (self.m_array).inspect_value(path, offset, limit),
                "m_FloatTestValue" => (self.m_float_test_value).inspect_value(path, offset, limit),
                "m_Bytes" => (self.m_bytes).inspect_value(path, offset, limit),
                "m_Floats" => (self.m_floats).inspect_value(path, offset, limit),
                "m_BytesSize" => (self.m_bytes_size).inspect_value(path, offset, limit),
                "byte data" => (self.byte_data)
                    .as_ref()
                    .and_then(|v| crate::inspect_bytes(&(v), path, offset, limit)),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_data).inspect_value(path, offset, limit)).map(|v| ("m_Data".to_string(), v)),
            ((self.m_array).inspect_value(path, offset, limit)).map(|v| ("m_Array".to_string(), v)),
            ((self.m_float_test_value).inspect_value(path, offset, limit))
                .map(|v| ("m_FloatTestValue".to_string(), v)),
            ((self.m_bytes).inspect_value(path, offset, limit)).map(|v| ("m_Bytes".to_string(), v)),
            ((self.m_floats).inspect_value(path, offset, limit))
                .map(|v| ("m_Floats".to_string(), v)),
            ((self.m_bytes_size).inspect_value(path, offset, limit))
                .map(|v| ("m_BytesSize".to_string(), v)),
            ((self.byte_data)
                .as_ref()
                .and_then(|v| crate::inspect_bytes(&(v), path, offset, limit)))
            .map(|v| ("byte data".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
