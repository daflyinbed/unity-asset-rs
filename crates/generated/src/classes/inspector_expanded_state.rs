//! 自动生成。请勿手动修改。

use crate::subclasses::expanded_data::ExpandedData;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct InspectorExpandedState {
    pub parent: Object,
    pub m_expanded_data: Vec<ExpandedData>,
}

impl crate::Parse for InspectorExpandedState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_expanded_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ExpandedData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(InspectorExpandedState {
            parent,
            m_expanded_data,
        })
    }
}
