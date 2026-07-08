//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct ControllerConstant {
    pub m_human_layer_array: Option<Vec<OffsetPtr>>,
    pub m_layer_array: Option<Vec<OffsetPtr>>,
    pub m_state_machine_array: Vec<OffsetPtr>,
    pub m_values: OffsetPtr,
    pub m_default_values: OffsetPtr,
}

impl crate::Parse for ControllerConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_human_layer_array = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_layer_array = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_state_machine_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_values = OffsetPtr::parse(rdr, ver)?;
        let m_default_values = OffsetPtr::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(ControllerConstant {
            m_human_layer_array,
            m_layer_array,
            m_state_machine_array,
            m_values,
            m_default_values,
        })
    }
}
