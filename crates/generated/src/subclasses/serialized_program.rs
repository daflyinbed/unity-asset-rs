//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_player_sub_program::SerializedPlayerSubProgram;
use crate::subclasses::serialized_program_parameters::SerializedProgramParameters;
use crate::subclasses::serialized_sub_program::SerializedSubProgram;

/// 版本枚举：SerializedProgramMSubProgramsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedProgramMSubProgramsVariant {
    Default(Vec<SerializedSubProgram>),
    V2017_1(Vec<SerializedSubProgram>),
}

impl crate::Parse for SerializedProgramMSubProgramsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SerializedSubProgram::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SerializedSubProgram::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SerializedProgramMSubProgramsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedProgram {
    pub m_sub_programs: SerializedProgramMSubProgramsVariant,
    pub m_common_parameters: Option<SerializedProgramParameters>,
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
        m_sub_programs = Some(SerializedProgramMSubProgramsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 13) {
            m_player_sub_programs = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| SerializedPlayerSubProgram::parse(rdr, ver))
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_parameter_blob_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| rdr.read_u32())
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_common_parameters = Some(SerializedProgramParameters::parse(rdr, ver)?);
            m_serialized_keyword_state_mask = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_common_parameters = Some(SerializedProgramParameters::parse(rdr, ver)?);
            m_serialized_keyword_state_mask = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 10) {
            m_player_sub_programs = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| SerializedPlayerSubProgram::parse(rdr, ver))
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_parameter_blob_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| rdr.read_u32())
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_common_parameters = Some(SerializedProgramParameters::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            m_common_parameters = Some(SerializedProgramParameters::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2020, 3, 2) {
            m_common_parameters = Some(SerializedProgramParameters::parse(rdr, ver)?);
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

impl crate::ToValue for SerializedProgram {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_SubPrograms".to_string(), self.m_sub_programs.to_value())),
            crate::opt_field("m_CommonParameters", &self.m_common_parameters),
            crate::opt_field("m_PlayerSubPrograms", &self.m_player_sub_programs),
            crate::opt_field("m_ParameterBlobIndices", &self.m_parameter_blob_indices),
            crate::opt_field(
                "m_SerializedKeywordStateMask",
                &self.m_serialized_keyword_state_mask,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_SubPrograms" => (self.m_sub_programs).inspect_value(path, offset, limit),
                "m_CommonParameters" => {
                    (self.m_common_parameters).inspect_value(path, offset, limit)
                }
                "m_PlayerSubPrograms" => {
                    (self.m_player_sub_programs).inspect_value(path, offset, limit)
                }
                "m_ParameterBlobIndices" => {
                    (self.m_parameter_blob_indices).inspect_value(path, offset, limit)
                }
                "m_SerializedKeywordStateMask" => {
                    (self.m_serialized_keyword_state_mask).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_sub_programs).inspect_value(path, offset, limit))
                .map(|v| ("m_SubPrograms".to_string(), v)),
            ((self.m_common_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_CommonParameters".to_string(), v)),
            ((self.m_player_sub_programs).inspect_value(path, offset, limit))
                .map(|v| ("m_PlayerSubPrograms".to_string(), v)),
            ((self.m_parameter_blob_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_ParameterBlobIndices".to_string(), v)),
            ((self.m_serialized_keyword_state_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_SerializedKeywordStateMask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
