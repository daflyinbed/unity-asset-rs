//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct SelectorTransitionConstant {
    pub m_destination: u32,
    pub m_condition_constant_array: Vec<OffsetPtr>,
}

impl crate::Parse for SelectorTransitionConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_destination = rdr.read_u32()?;
        let m_condition_constant_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SelectorTransitionConstant {
            m_destination,
            m_condition_constant_array,
        })
    }
}
