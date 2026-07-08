//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedArray {
    pub m_integer_array: Vec<i32>,
    pub m_clamp_test_value: f32,
}

impl crate::Parse for TestObjectWithSerializedArray {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_integer_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_clamp_test_value = rdr.read_f32()?;

        Ok(TestObjectWithSerializedArray {
            m_integer_array,
            m_clamp_test_value,
        })
    }
}
