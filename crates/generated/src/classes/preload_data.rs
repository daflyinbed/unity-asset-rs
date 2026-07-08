//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：PreloadDataMAssetsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PreloadDataMAssetsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for PreloadDataMAssetsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct PreloadData {
    pub m_name: String,
    pub m_assets: PreloadDataMAssetsVariant,
    pub m_dependencies: Option<Vec<String>>,
    pub m_explicit_data_layout: Option<bool>,
}

impl crate::Parse for PreloadData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_assets = PreloadDataMAssetsVariant::parse(rdr, ver)?;
        let m_dependencies = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_explicit_data_layout = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(PreloadData {
            m_name,
            m_assets,
            m_dependencies,
            m_explicit_data_layout,
        })
    }
}
