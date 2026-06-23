//! 自动生成。请勿手动修改。

use crate::subclasses::expanded_data::ExpandedData;

#[derive(Debug, Clone)]
pub struct InspectorExpandedState {
    pub m_object_hide_flags: u32,
    pub m_expanded_data: Vec<ExpandedData>,
}

impl crate::Parse for InspectorExpandedState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_expanded_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ExpandedData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(InspectorExpandedState {
            m_object_hide_flags,
            m_expanded_data,
        })
    }
}
