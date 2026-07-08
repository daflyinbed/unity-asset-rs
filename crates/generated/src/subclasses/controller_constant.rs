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
        let mut m_human_layer_array = None;
        let mut m_layer_array = None;
        let mut m_state_machine_array = None;
        let mut m_values = None;
        let mut m_default_values = None;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_layer_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
            m_human_layer_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_state_machine_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_values = Some(OffsetPtr::parse(rdr, ver)?);
            m_default_values = Some(OffsetPtr::parse(rdr, ver)?);

        Ok(ControllerConstant {
            m_human_layer_array,
            m_layer_array,
            m_state_machine_array: m_state_machine_array.unwrap(),
            m_values: m_values.unwrap(),
            m_default_values: m_default_values.unwrap(),
        })
    }
}
