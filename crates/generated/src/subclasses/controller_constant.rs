//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct ControllerConstant {
    pub m_layer_array: Vec<OffsetPtr>,
    pub m_state_machine_array: Vec<OffsetPtr>,
    pub m_values: OffsetPtr,
    pub m_default_values: OffsetPtr,
}

impl crate::Parse for ControllerConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_layer_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_state_machine_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_values = OffsetPtr::parse(rdr, ver)?;
        let m_default_values = OffsetPtr::parse(rdr, ver)?;

        Ok(ControllerConstant {
            m_layer_array,
            m_state_machine_array,
            m_values,
            m_default_values,
        })
    }
}
