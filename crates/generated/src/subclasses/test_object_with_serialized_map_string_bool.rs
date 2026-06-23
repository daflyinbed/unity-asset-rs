//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedMapStringBool {
    pub m_object_hide_flags: u32,
    pub m_map: Vec<(String, bool)>,
    pub m_string: String,
}

impl crate::Parse for TestObjectWithSerializedMapStringBool {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_map = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_bool()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_string = r.read_aligned_string()?;
        r.align(4)?;

        Ok(TestObjectWithSerializedMapStringBool {
            m_object_hide_flags,
            m_map,
            m_string,
        })
    }
}
