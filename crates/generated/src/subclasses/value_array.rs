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

impl crate::ToValue for ValueArrayMPositionValuesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ValueArrayMScaleValuesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ValueArray {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BoolValues".to_string(), self.m_bool_values.to_value())),
            crate::opt_field("m_PositionValues", &self.m_position_values),
            Some(("m_IntValues".to_string(), self.m_int_values.to_value())),
            crate::opt_field("m_QuaternionValues", &self.m_quaternion_values),
            Some(("m_FloatValues".to_string(), self.m_float_values.to_value())),
            crate::opt_field("m_ScaleValues", &self.m_scale_values),
            crate::opt_field("m_VectorValues", &self.m_vector_values),
            crate::opt_field("m_EntityIdValues", &self.m_entity_id_values),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BoolValues" => (self.m_bool_values).inspect_value(path, offset, limit),
                "m_PositionValues" => (self.m_position_values).inspect_value(path, offset, limit),
                "m_IntValues" => (self.m_int_values).inspect_value(path, offset, limit),
                "m_QuaternionValues" => {
                    (self.m_quaternion_values).inspect_value(path, offset, limit)
                }
                "m_FloatValues" => (self.m_float_values).inspect_value(path, offset, limit),
                "m_ScaleValues" => (self.m_scale_values).inspect_value(path, offset, limit),
                "m_VectorValues" => (self.m_vector_values).inspect_value(path, offset, limit),
                "m_EntityIdValues" => (self.m_entity_id_values).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_bool_values).inspect_value(path, offset, limit))
                .map(|v| ("m_BoolValues".to_string(), v)),
            ((self.m_position_values).inspect_value(path, offset, limit))
                .map(|v| ("m_PositionValues".to_string(), v)),
            ((self.m_int_values).inspect_value(path, offset, limit))
                .map(|v| ("m_IntValues".to_string(), v)),
            ((self.m_quaternion_values).inspect_value(path, offset, limit))
                .map(|v| ("m_QuaternionValues".to_string(), v)),
            ((self.m_float_values).inspect_value(path, offset, limit))
                .map(|v| ("m_FloatValues".to_string(), v)),
            ((self.m_scale_values).inspect_value(path, offset, limit))
                .map(|v| ("m_ScaleValues".to_string(), v)),
            ((self.m_vector_values).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorValues".to_string(), v)),
            ((self.m_entity_id_values).inspect_value(path, offset, limit))
                .map(|v| ("m_EntityIdValues".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
