//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ExpandedData {
    pub m_inspector_expanded: bool,
    pub m_class_id: i32,
    pub m_script_class: String,
    pub m_expanded_properties: Vec<String>,
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
        rdr.align(4)?;
        let m_expanded_properties = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(ExpandedData {
            m_inspector_expanded,
            m_class_id,
            m_script_class,
            m_expanded_properties,
        })
    }
}
