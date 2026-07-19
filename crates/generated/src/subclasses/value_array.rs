//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;

/// 版本枚举：ValueArrayMPositionValuesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ValueArrayMPositionValuesVariant {
    Default(Vec<float4>),
    V5_4(Vec<float3>),
}

impl crate::Parse for ValueArrayMPositionValuesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| float3::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| float4::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ValueArrayMScaleValuesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ValueArrayMScaleValuesVariant {
    Default(Vec<float4>),
    V5_4(Vec<float3>),
}

impl crate::Parse for ValueArrayMScaleValuesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| float3::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| float4::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ValueArray {
    pub m_bool_values: Vec<bool>,
    pub m_position_values: Option<ValueArrayMPositionValuesVariant>,
    pub m_int_values: Vec<i32>,
    pub m_quaternion_values: Option<Vec<float4>>,
    pub m_float_values: Vec<f32>,
    pub m_scale_values: Option<ValueArrayMScaleValuesVariant>,
    pub m_vector_values: Option<Vec<float4>>,
    pub m_entity_id_values: Option<Vec<i32>>,
}

impl crate::Parse for ValueArray {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_bool_values = None;
        let mut m_position_values = None;
        let mut m_int_values = None;
        let mut m_quaternion_values = None;
        let mut m_float_values = None;
        let mut m_scale_values = None;
        let mut m_vector_values = None;
        let mut m_entity_id_values = None;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_position_values = Some(ValueArrayMPositionValuesVariant::parse(rdr, ver)?);
            m_quaternion_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| float4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_scale_values = Some(ValueArrayMScaleValuesVariant::parse(rdr, ver)?);
        } else {
            m_bool_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_int_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_float_values = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_int_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_bool_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_entity_id_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_int_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_bool_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_position_values = Some(ValueArrayMPositionValuesVariant::parse(rdr, ver)?);
            m_quaternion_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| float4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_scale_values = Some(ValueArrayMScaleValuesVariant::parse(rdr, ver)?);
        } else {
            m_vector_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| float4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }

        Ok(ValueArray {
            m_bool_values: m_bool_values.unwrap(),
            m_position_values,
            m_int_values: m_int_values.unwrap(),
            m_quaternion_values,
            m_float_values: m_float_values.unwrap(),
            m_scale_values,
            m_vector_values,
            m_entity_id_values,
        })
    }
}
