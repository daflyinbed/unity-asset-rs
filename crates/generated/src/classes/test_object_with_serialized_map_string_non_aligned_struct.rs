//! 自动生成。请勿手动修改。

use crate::subclasses::non_aligned_struct::NonAlignedStruct;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedMapStringNonAlignedStruct {
    pub parent: Object,
    pub m_map: Vec<(String, NonAlignedStruct)>,
    pub m_string: String,
}

impl crate::Parse for TestObjectWithSerializedMapStringNonAlignedStruct {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_map = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = NonAlignedStruct::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_string = r.read_aligned_string()?;
        r.align(4)?;

        Ok(TestObjectWithSerializedMapStringNonAlignedStruct {
            parent,
            m_map,
            m_string,
        })
    }
}
