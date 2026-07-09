//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::property_name32::PropertyName32;
use crate::subclasses::property_name_nested::PropertyNameNested;


/// 版本枚举：ExposedReferenceTableMReferencesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ExposedReferenceTableMReferencesVariant {
    Default(Vec<(PropertyName32, PPtr)>),
    V2018_4_25(Vec<(PropertyNameNested, PPtr)>),
    V2019_1(Vec<(PropertyName32, PPtr)>),
    V2019_4_3(Vec<(PropertyNameNested, PPtr)>),
}

impl crate::Parse for ExposedReferenceTableMReferencesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 4, 3) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = PropertyNameNested::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2019_4_3(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = PropertyName32::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 25) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = PropertyNameNested::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2018_4_25(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = PropertyName32::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExposedReferenceTable {
    pub m_references: ExposedReferenceTableMReferencesVariant,
}

impl crate::Parse for ExposedReferenceTable {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_references = ExposedReferenceTableMReferencesVariant::parse(rdr, ver)?;

        Ok(ExposedReferenceTable {
            m_references,
        })
    }
}
