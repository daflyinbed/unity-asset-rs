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
