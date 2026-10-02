//! 自动生成。请勿手动修改。

use crate::subclasses::matrix4x4f::Matrix4x4f;

/// 版本枚举：TilemapRefCountedData_Matrix4x4fMRefCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRefCountedData_Matrix4x4fMRefCountVariant {
    Default(u32),
    V6000_4(i32),
}

impl crate::Parse for TilemapRefCountedData_Matrix4x4fMRefCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V6000_4(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TilemapRefCountedData_Matrix4x4fMRefCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TilemapRefCountedData_Matrix4x4f {
    pub m_ref_count: TilemapRefCountedData_Matrix4x4fMRefCountVariant,
    pub m_data: Matrix4x4f,
}

impl crate::Parse for TilemapRefCountedData_Matrix4x4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_ref_count = TilemapRefCountedData_Matrix4x4fMRefCountVariant::parse(rdr, ver)?;
        let m_data = Matrix4x4f::parse(rdr, ver)?;

        Ok(TilemapRefCountedData_Matrix4x4f {
            m_ref_count,
            m_data,
        })
    }
}

impl crate::ToValue for TilemapRefCountedData_Matrix4x4f {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_RefCount".to_string(), self.m_ref_count.to_value())),
            Some(("m_Data".to_string(), self.m_data.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_RefCount" => (self.m_ref_count).inspect_value(path, offset, limit),
                "m_Data" => (self.m_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_ref_count).inspect_value(path, offset, limit))
                .map(|v| ("m_RefCount".to_string(), v)),
            ((self.m_data).inspect_value(path, offset, limit)).map(|v| ("m_Data".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
