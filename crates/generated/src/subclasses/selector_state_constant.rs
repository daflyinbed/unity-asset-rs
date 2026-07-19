//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct SelectorStateConstant {
    pub m_transition_constant_array: Vec<OffsetPtr>,
    pub m_full_path_id: u32,
    pub m_is_entry: bool,
}

impl crate::Parse for SelectorStateConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_transition_constant_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| OffsetPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_full_path_id = rdr.read_u32()?;
        let m_is_entry = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(SelectorStateConstant {
            m_transition_constant_array,
            m_full_path_id,
            m_is_entry,
        })
    }
}
