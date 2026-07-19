//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedMapStringBool {
    pub m_map: Vec<(String, bool)>,
    pub m_string: String,
}

impl crate::Parse for TestObjectWithSerializedMapStringBool {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = rdr.read_bool()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_string = rdr.read_aligned_string()?;

        Ok(TestObjectWithSerializedMapStringBool { m_map, m_string })
    }
}
