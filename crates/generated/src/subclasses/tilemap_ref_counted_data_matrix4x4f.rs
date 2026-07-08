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
