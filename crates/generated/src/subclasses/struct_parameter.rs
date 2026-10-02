//! 自动生成。请勿手动修改。

use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct StructParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_offset_in_constant_buffer: Option<i32>,
    pub m_array_size: i32,
    pub m_struct_size: i32,
    pub m_vector_members: Vec<VectorParameter>,
    pub m_matrix_members: Vec<MatrixParameter>,
}

impl crate::Parse for StructParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_offset_in_constant_buffer = None;
        let mut m_array_size = None;
        let mut m_struct_size = None;
        let mut m_vector_members = None;
        let mut m_matrix_members = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_offset_in_constant_buffer = Some(rdr.read_i32()?);
        } else {
            m_index = Some(rdr.read_i32()?);
        }
        m_array_size = Some(rdr.read_i32()?);
        m_struct_size = Some(rdr.read_i32()?);
        m_vector_members = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VectorParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_matrix_members = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MatrixParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;

        Ok(StructParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_offset_in_constant_buffer,
            m_array_size: m_array_size.unwrap(),
            m_struct_size: m_struct_size.unwrap(),
            m_vector_members: m_vector_members.unwrap(),
            m_matrix_members: m_matrix_members.unwrap(),
        })
    }
}

impl crate::ToValue for StructParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NameIndex".to_string(), self.m_name_index.to_value())),
            crate::opt_field("m_Index", &self.m_index),
            crate::opt_field(
                "m_OffsetInConstantBuffer",
                &self.m_offset_in_constant_buffer,
            ),
            Some(("m_ArraySize".to_string(), self.m_array_size.to_value())),
            Some(("m_StructSize".to_string(), self.m_struct_size.to_value())),
            Some((
                "m_VectorMembers".to_string(),
                self.m_vector_members.to_value(),
            )),
            Some((
                "m_MatrixMembers".to_string(),
                self.m_matrix_members.to_value(),
            )),
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
                "m_Index" => (self.m_index).inspect_value(path, offset, limit),
                "m_OffsetInConstantBuffer" => {
                    (self.m_offset_in_constant_buffer).inspect_value(path, offset, limit)
                }
                "m_ArraySize" => (self.m_array_size).inspect_value(path, offset, limit),
                "m_StructSize" => (self.m_struct_size).inspect_value(path, offset, limit),
                "m_VectorMembers" => (self.m_vector_members).inspect_value(path, offset, limit),
                "m_MatrixMembers" => (self.m_matrix_members).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name_index).inspect_value(path, offset, limit))
                .map(|v| ("m_NameIndex".to_string(), v)),
            ((self.m_index).inspect_value(path, offset, limit)).map(|v| ("m_Index".to_string(), v)),
            ((self.m_offset_in_constant_buffer).inspect_value(path, offset, limit))
                .map(|v| ("m_OffsetInConstantBuffer".to_string(), v)),
            ((self.m_array_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ArraySize".to_string(), v)),
            ((self.m_struct_size).inspect_value(path, offset, limit))
                .map(|v| ("m_StructSize".to_string(), v)),
            ((self.m_vector_members).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorMembers".to_string(), v)),
            ((self.m_matrix_members).inspect_value(path, offset, limit))
                .map(|v| ("m_MatrixMembers".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
