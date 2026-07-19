//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;

/// 版本枚举：TilemapRefCountedData_ColorRgbAfMRefCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TilemapRefCountedData_ColorRgbAfMRefCountVariant {
    Default(u32),
    V6000_4(i32),
}

impl crate::Parse for TilemapRefCountedData_ColorRgbAfMRefCountVariant {
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
pub struct TilemapRefCountedData_ColorRgbAf {
    pub m_ref_count: TilemapRefCountedData_ColorRgbAfMRefCountVariant,
    pub m_data: ColorRgbAf,
}

impl crate::Parse for TilemapRefCountedData_ColorRgbAf {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_ref_count = TilemapRefCountedData_ColorRgbAfMRefCountVariant::parse(rdr, ver)?;
        let m_data = ColorRgbAf::parse(rdr, ver)?;

        Ok(TilemapRefCountedData_ColorRgbAf {
            m_ref_count,
            m_data,
        })
    }
}
