//! 自动生成。请勿手动修改。

use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::struct_parameter::StructParameter;
use crate::subclasses::vector_parameter::VectorParameter;

/// 版本枚举：ConstantBufferMMatrixParamsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ConstantBufferMMatrixParamsVariant {
    Default(Vec<MatrixParameter>),
    V2017_1(Vec<MatrixParameter>),
}

impl crate::Parse for ConstantBufferMMatrixParamsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MatrixParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MatrixParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ConstantBufferMMatrixParamsVariant {
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

/// 版本枚举：ConstantBufferMVectorParamsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ConstantBufferMVectorParamsVariant {
    Default(Vec<VectorParameter>),
    V2017_1(Vec<VectorParameter>),
}

impl crate::Parse for ConstantBufferMVectorParamsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VectorParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VectorParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ConstantBufferMVectorParamsVariant {
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
pub struct ConstantBuffer {
    pub m_name_index: i32,
    pub m_matrix_params: ConstantBufferMMatrixParamsVariant,
    pub m_vector_params: ConstantBufferMVectorParamsVariant,
    pub m_size: i32,
    pub m_struct_params: Option<Vec<StructParameter>>,
    pub m_is_partial_cb: Option<bool>,
}

impl crate::Parse for ConstantBuffer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_matrix_params = None;
        let mut m_vector_params = None;
        let mut m_size = None;
        let mut m_struct_params = None;
        let mut m_is_partial_cb = None;
        m_name_index = Some(rdr.read_i32()?);
        m_matrix_params = Some(ConstantBufferMMatrixParamsVariant::parse(rdr, ver)?);
        m_vector_params = Some(ConstantBufferMVectorParamsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_struct_params = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| StructParameter::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            m_is_partial_cb = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2020, 3, 2) {
            m_is_partial_cb = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(ConstantBuffer {
            m_name_index: m_name_index.unwrap(),
            m_matrix_params: m_matrix_params.unwrap(),
            m_vector_params: m_vector_params.unwrap(),
            m_size: m_size.unwrap(),
            m_struct_params,
            m_is_partial_cb,
        })
    }
}

impl crate::ToValue for ConstantBuffer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NameIndex".to_string(), self.m_name_index.to_value())),
            Some((
                "m_MatrixParams".to_string(),
                self.m_matrix_params.to_value(),
            )),
            Some((
                "m_VectorParams".to_string(),
                self.m_vector_params.to_value(),
            )),
            Some(("m_Size".to_string(), self.m_size.to_value())),
            crate::opt_field("m_StructParams", &self.m_struct_params),
            crate::opt_field("m_IsPartialCB", &self.m_is_partial_cb),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NameIndex" => (self.m_name_index).inspect_value(path, offset, limit),
                "m_MatrixParams" => (self.m_matrix_params).inspect_value(path, offset, limit),
                "m_VectorParams" => (self.m_vector_params).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                "m_StructParams" => (self.m_struct_params).inspect_value(path, offset, limit),
                "m_IsPartialCB" => (self.m_is_partial_cb).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name_index).inspect_value(path, offset, limit))
                .map(|v| ("m_NameIndex".to_string(), v)),
            ((self.m_matrix_params).inspect_value(path, offset, limit))
                .map(|v| ("m_MatrixParams".to_string(), v)),
            ((self.m_vector_params).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorParams".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
            ((self.m_struct_params).inspect_value(path, offset, limit))
                .map(|v| ("m_StructParams".to_string(), v)),
            ((self.m_is_partial_cb).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPartialCB".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
