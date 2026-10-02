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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PreloadDataMAssetsVariant {
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

/// 版本枚举：PreloadDataMDependenciesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PreloadDataMDependenciesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for PreloadDataMDependenciesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PreloadDataMDependenciesVariant {
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
pub struct PreloadData {
    pub m_name: String,
    pub m_assets: PreloadDataMAssetsVariant,
    pub m_dependencies: Option<PreloadDataMDependenciesVariant>,
    pub m_explicit_data_layout: Option<bool>,
}

impl crate::Parse for PreloadData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_assets = PreloadDataMAssetsVariant::parse(rdr, ver)?;
        let m_dependencies = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(PreloadDataMDependenciesVariant::parse(rdr, ver)?)
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

impl crate::ToValue for PreloadData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Assets".to_string(), self.m_assets.to_value())),
            crate::opt_field("m_Dependencies", &self.m_dependencies),
            crate::opt_field("m_ExplicitDataLayout", &self.m_explicit_data_layout),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Assets" => (self.m_assets).inspect_value(path, offset, limit),
                "m_Dependencies" => (self.m_dependencies).inspect_value(path, offset, limit),
                "m_ExplicitDataLayout" => {
                    (self.m_explicit_data_layout).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_assets).inspect_value(path, offset, limit))
                .map(|v| ("m_Assets".to_string(), v)),
            ((self.m_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("m_Dependencies".to_string(), v)),
            ((self.m_explicit_data_layout).inspect_value(path, offset, limit))
                .map(|v| ("m_ExplicitDataLayout".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
