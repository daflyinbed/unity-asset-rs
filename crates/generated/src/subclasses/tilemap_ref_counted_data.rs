//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;


/// 版本枚举：TilemapRefCountedDataMRefCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRefCountedDataMRefCountVariant {
    Default(u32),
    V6000_4(i32),
}

impl crate::Parse for TilemapRefCountedDataMRefCountVariant {
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

/// 版本枚举：TilemapRefCountedDataMDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRefCountedDataMDataVariant {
    Default(Box<ColorRgba>),
    V2019_3(PPtr),
}

impl crate::Parse for TilemapRefCountedDataMDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V2019_3(v))
        } else {
            let v = ( { Ok(Box::new(ColorRgba::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TilemapRefCountedData {
    pub m_ref_count: TilemapRefCountedDataMRefCountVariant,
    pub m_data: TilemapRefCountedDataMDataVariant,
}

impl crate::Parse for TilemapRefCountedData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_ref_count = TilemapRefCountedDataMRefCountVariant::parse(rdr, ver)?;
        let m_data = TilemapRefCountedDataMDataVariant::parse(rdr, ver)?;

        Ok(TilemapRefCountedData {
            m_ref_count,
            m_data,
        })
    }
}
