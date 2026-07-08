//! 自动生成。请勿手动修改。

use crate::subclasses::program_parameters::ProgramParameters;
use crate::subclasses::serialized_player_sub_program::SerializedPlayerSubProgram;
use crate::subclasses::serialized_program_parameters::SerializedProgramParameters;
use crate::subclasses::serialized_sub_program::SerializedSubProgram;


/// 版本枚举：SerializedProgramMCommonParametersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedProgramMCommonParametersVariant {
    Default(Box<SerializedProgramParameters>),
    V6000_5(Box<ProgramParameters>),
}

impl crate::Parse for SerializedProgramMCommonParametersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ( { Ok(Box::new(ProgramParameters::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V6000_5(v))
        } else {
            let v = ( { Ok(Box::new(SerializedProgramParameters::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedProgram {
    pub m_sub_programs: Vec<SerializedSubProgram>,
    pub m_common_parameters: Option<SerializedProgramMCommonParametersVariant>,
    pub m_player_sub_programs: Option<Vec<Vec<SerializedPlayerSubProgram>>>,
    pub m_parameter_blob_indices: Option<Vec<Vec<u32>>>,
    pub m_serialized_keyword_state_mask: Option<Vec<u16>>,
}

impl crate::Parse for SerializedProgram {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_sub_programs = None;
        let mut m_common_parameters = None;
        let mut m_player_sub_programs = None;
        let mut m_parameter_blob_indices = None;
        let mut m_serialized_keyword_state_mask = None;
            m_sub_programs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SerializedSubProgram::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 1, 13) {
            m_player_sub_programs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SerializedPlayerSubProgram::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_parameter_blob_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_common_parameters = Some(SerializedProgramMCommonParametersVariant::parse(rdr, ver)?);
            m_serialized_keyword_state_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_common_parameters = Some(SerializedProgramMCommonParametersVariant::parse(rdr, ver)?);
            m_serialized_keyword_state_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 10) {
            m_player_sub_programs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SerializedPlayerSubProgram::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_parameter_blob_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_common_parameters = Some(SerializedProgramMCommonParametersVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            m_common_parameters = Some(SerializedProgramMCommonParametersVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2020, 3, 2) {
            m_common_parameters = Some(SerializedProgramMCommonParametersVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(SerializedProgram {
            m_sub_programs: m_sub_programs.unwrap(),
            m_common_parameters,
            m_player_sub_programs,
            m_parameter_blob_indices,
            m_serialized_keyword_state_mask,
        })
    }
}
