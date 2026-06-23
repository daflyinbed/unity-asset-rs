//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_property::SerializedProperty;

#[derive(Debug, Clone)]
pub struct SerializedProperties {
    pub m_props: Vec<SerializedProperty>,
}

impl crate::Parse for SerializedProperties {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_props = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SerializedProperty::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SerializedProperties {
            m_props,
        })
    }
}
