//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_property::SerializedProperty;

/// 版本枚举：SerializedPropertiesMPropsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedPropertiesMPropsVariant {
    Default(Vec<SerializedProperty>),
    V2017_1(Vec<SerializedProperty>),
}

impl crate::Parse for SerializedPropertiesMPropsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SerializedProperty::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SerializedProperty::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedProperties {
    pub m_props: SerializedPropertiesMPropsVariant,
}

impl crate::Parse for SerializedProperties {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_props = SerializedPropertiesMPropsVariant::parse(rdr, ver)?;

        Ok(SerializedProperties { m_props })
    }
}
