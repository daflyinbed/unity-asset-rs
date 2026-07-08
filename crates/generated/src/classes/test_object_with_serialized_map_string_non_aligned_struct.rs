//! 自动生成。请勿手动修改。

use crate::subclasses::non_aligned_struct::NonAlignedStruct;

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedMapStringNonAlignedStruct {
    pub m_map: Vec<(String, NonAlignedStruct)>,
    pub m_string: String,
}

impl crate::Parse for TestObjectWithSerializedMapStringNonAlignedStruct {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_map = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = NonAlignedStruct::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_string = rdr.read_aligned_string()?;

        Ok(TestObjectWithSerializedMapStringNonAlignedStruct {
            m_map,
            m_string,
        })
    }
}
