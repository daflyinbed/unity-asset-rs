//! 自动生成。请勿手动修改。

use crate::subclasses::expanded_data::ExpandedData;

#[derive(Debug, Clone)]
pub struct InspectorExpandedState {
    pub m_expanded_data: Vec<ExpandedData>,
}

impl crate::Parse for InspectorExpandedState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expanded_data = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ExpandedData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(InspectorExpandedState {
            m_expanded_data,
        })
    }
}
