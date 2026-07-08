//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：TilemapRefCountedData_PPtr_Object_MRefCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRefCountedData_PPtr_Object_MRefCountVariant {
    Default(u32),
    V6000_4(i32),
}

impl crate::Parse for TilemapRefCountedData_PPtr_Object_MRefCountVariant {
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
pub struct TilemapRefCountedData_PPtr_Object_ {
    pub m_ref_count: TilemapRefCountedData_PPtr_Object_MRefCountVariant,
    pub m_data: PPtr,
}

impl crate::Parse for TilemapRefCountedData_PPtr_Object_ {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_ref_count = TilemapRefCountedData_PPtr_Object_MRefCountVariant::parse(rdr, ver)?;
        let m_data = crate::PPtr::parse(rdr, ver)?;

        Ok(TilemapRefCountedData_PPtr_Object_ {
            m_ref_count,
            m_data,
        })
    }
}
