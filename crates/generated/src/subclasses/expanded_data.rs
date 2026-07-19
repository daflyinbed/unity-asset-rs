//! 自动生成。请勿手动修改。

/// 版本枚举：ExpandedDataMExpandedPropertiesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ExpandedDataMExpandedPropertiesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for ExpandedDataMExpandedPropertiesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExpandedData {
    pub m_inspector_expanded: bool,
    pub m_class_id: i32,
    pub m_script_class: String,
    pub m_expanded_properties: ExpandedDataMExpandedPropertiesVariant,
}

impl crate::Parse for ExpandedData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_inspector_expanded = rdr.read_bool()?;
        rdr.align(4)?;
        let m_class_id = rdr.read_i32()?;
        let m_script_class = rdr.read_aligned_string()?;
        let m_expanded_properties = ExpandedDataMExpandedPropertiesVariant::parse(rdr, ver)?;

        Ok(ExpandedData {
            m_inspector_expanded,
            m_class_id,
            m_script_class,
            m_expanded_properties,
        })
    }
}
