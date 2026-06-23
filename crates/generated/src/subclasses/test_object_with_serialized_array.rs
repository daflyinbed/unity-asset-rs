//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedArray {
    pub m_object_hide_flags: u32,
    pub m_integer_array: Vec<i32>,
    pub m_clamp_test_value: f32,
}

impl crate::Parse for TestObjectWithSerializedArray {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_integer_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_clamp_test_value = r.read_f32()?;

        Ok(TestObjectWithSerializedArray {
            m_object_hide_flags,
            m_integer_array,
            m_clamp_test_value,
        })
    }
}
