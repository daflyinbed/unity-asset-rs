//! 自动生成。请勿手动修改。

use crate::subclasses::hint::Hint;

#[derive(Debug, Clone)]
pub struct ReflectedParameter {
    pub m_direction: i32,
    pub m_type_name: String,
    pub m_name: String,
    pub m_hints: Vec<Hint>,
}

impl crate::Parse for ReflectedParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_direction = rdr.read_i32()?;
        let m_type_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_hints = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Hint::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(ReflectedParameter {
            m_direction,
            m_type_name,
            m_name,
            m_hints,
        })
    }
}
