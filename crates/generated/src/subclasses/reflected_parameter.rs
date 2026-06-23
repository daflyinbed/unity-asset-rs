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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_direction = r.read_i32()?;
        let m_type_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_hints = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Hint::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ReflectedParameter {
            m_direction,
            m_type_name,
            m_name,
            m_hints,
        })
    }
}
