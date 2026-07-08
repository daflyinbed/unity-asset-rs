//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct NameToObjectMap {
    pub m_object_to_name: Vec<(PPtr, String)>,
}

impl crate::Parse for NameToObjectMap {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_to_name = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(rdr, ver)?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(NameToObjectMap {
            m_object_to_name,
        })
    }
}
