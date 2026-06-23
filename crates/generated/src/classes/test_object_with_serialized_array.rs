//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedArray {
    pub parent: Object,
    pub m_integer_array: Vec<i32>,
    pub m_clamp_test_value: f32,
}

impl crate::Parse for TestObjectWithSerializedArray {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_integer_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_clamp_test_value = r.read_f32()?;

        Ok(TestObjectWithSerializedArray {
            parent,
            m_integer_array,
            m_clamp_test_value,
        })
    }
}
