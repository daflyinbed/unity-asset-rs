//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PresetType {
    pub m_native_type_id: i32,
    pub m_managed_type_p_ptr: PPtr,
    pub m_managed_type_fallback: String,
}

impl crate::Parse for PresetType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_native_type_id = rdr.read_i32()?;
        let m_managed_type_p_ptr = crate::PPtr::parse(rdr, ver)?;
        let m_managed_type_fallback = rdr.read_aligned_string()?;

        Ok(PresetType {
            m_native_type_id,
            m_managed_type_p_ptr,
            m_managed_type_fallback,
        })
    }
}

impl crate::ToValue for PresetType {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_NativeTypeID".to_string(),
                self.m_native_type_id.to_value(),
            )),
            Some((
                "m_ManagedTypePPtr".to_string(),
                self.m_managed_type_p_ptr.to_value(),
            )),
            Some((
                "m_ManagedTypeFallback".to_string(),
                self.m_managed_type_fallback.to_value(),
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
                "m_NativeTypeID" => (self.m_native_type_id).inspect_value(path, offset, limit),
                "m_ManagedTypePPtr" => {
                    (self.m_managed_type_p_ptr).inspect_value(path, offset, limit)
                }
                "m_ManagedTypeFallback" => {
                    (self.m_managed_type_fallback).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_native_type_id).inspect_value(path, offset, limit))
                .map(|v| ("m_NativeTypeID".to_string(), v)),
            ((self.m_managed_type_p_ptr).inspect_value(path, offset, limit))
                .map(|v| ("m_ManagedTypePPtr".to_string(), v)),
            ((self.m_managed_type_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_ManagedTypeFallback".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
