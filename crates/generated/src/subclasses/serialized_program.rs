//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_sub_program::SerializedSubProgram;

#[derive(Debug, Clone)]
pub struct SerializedProgram {
    pub m_sub_programs: Vec<SerializedSubProgram>,
}

impl crate::Parse for SerializedProgram {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_sub_programs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SerializedSubProgram::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SerializedProgram {
            m_sub_programs,
        })
    }
}
