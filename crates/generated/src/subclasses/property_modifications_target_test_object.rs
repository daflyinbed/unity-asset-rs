//! 自动生成。请勿手动修改。

use crate::subclasses::property_modifications_target_test_native_object::PropertyModificationsTargetTestNativeObject;

#[derive(Debug, Clone)]
pub struct PropertyModificationsTargetTestObject {
    pub m_object_hide_flags: u32,
    pub m_data: PropertyModificationsTargetTestNativeObject,
    pub m_array: Vec<PropertyModificationsTargetTestNativeObject>,
    pub m_float_test_value: f32,
}

impl crate::Parse for PropertyModificationsTargetTestObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_data = PropertyModificationsTargetTestNativeObject::parse(r, ver)?;
        let m_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PropertyModificationsTargetTestNativeObject::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_float_test_value = r.read_f32()?;

        Ok(PropertyModificationsTargetTestObject {
            m_object_hide_flags,
            m_data,
            m_array,
            m_float_test_value,
        })
    }
}
