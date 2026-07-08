//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_exposed::VfxEntryExposed;
use crate::subclasses::vfx_entry_expression_value::VfxEntryExpressionValue;


/// 版本枚举：VfxFieldMArrayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxFieldMArrayVariant {
    Default(Vec<VfxEntryExposed>),
    V2018_4_13(Vec<VfxEntryExpressionValue>),
    V2019_3(Vec<VfxEntryExposed>),
    V2020_1(Vec<VfxEntryExpressionValue>),
    V2020_2(Vec<VfxEntryExposed>),
    V2021_2(Vec<VfxEntryExpressionValue>),
    V2022_2(Vec<VfxEntryExposed>),
    V2022_3_6(Vec<VfxEntryExpressionValue>),
    V6000_5(Vec<VfxEntryExposed>),
    V6000_6(Vec<VfxEntryExpressionValue>),
}

impl crate::Parse for VfxFieldMArrayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2022_3_6(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2020_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct VfxField {
    pub m_array: VfxFieldMArrayVariant,
}

impl crate::Parse for VfxField {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = VfxFieldMArrayVariant::parse(rdr, ver)?;

        Ok(VfxField {
            m_array,
        })
    }
}
