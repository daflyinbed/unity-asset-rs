//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HumanTemplate {
    pub m_name: String,
    pub m_bone_template: Vec<(String, String)>,
}

impl crate::Parse for HumanTemplate {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_bone_template = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(HumanTemplate {
            m_name,
            m_bone_template,
        })
    }
}
