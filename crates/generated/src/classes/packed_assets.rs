//! 自动生成。请勿手动修改。

use crate::subclasses::build_report_packed_asset_info::BuildReportPackedAssetInfo;

/// 版本枚举：PackedAssetsMContentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PackedAssetsMContentsVariant {
    Default(Vec<BuildReportPackedAssetInfo>),
    V2017_1(Vec<BuildReportPackedAssetInfo>),
}

impl crate::Parse for PackedAssetsMContentsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildReportPackedAssetInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildReportPackedAssetInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PackedAssetsMContentsVariant {
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
pub struct PackedAssets {
    pub m_file: Option<u32>,
    pub m_short_path: String,
    pub m_overhead: u64,
    pub m_contents: PackedAssetsMContentsVariant,
}

impl crate::Parse for PackedAssets {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_file = None;
        let mut m_short_path = None;
        let mut m_overhead = None;
        let mut m_contents = None;
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
        } else {
            m_file = Some(rdr.read_u32()?);
        }
        m_short_path = Some(rdr.read_aligned_string()?);
        m_overhead = Some(rdr.read_u64()?);
        m_contents = Some(PackedAssetsMContentsVariant::parse(rdr, ver)?);

        Ok(PackedAssets {
            m_file,
            m_short_path: m_short_path.unwrap(),
            m_overhead: m_overhead.unwrap(),
            m_contents: m_contents.unwrap(),
        })
    }
}

impl crate::ToValue for PackedAssets {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_File", &self.m_file),
            Some(("m_ShortPath".to_string(), self.m_short_path.to_value())),
            Some(("m_Overhead".to_string(), self.m_overhead.to_value())),
            Some(("m_Contents".to_string(), self.m_contents.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_File" => (self.m_file).inspect_value(path, offset, limit),
                "m_ShortPath" => (self.m_short_path).inspect_value(path, offset, limit),
                "m_Overhead" => (self.m_overhead).inspect_value(path, offset, limit),
                "m_Contents" => (self.m_contents).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_file).inspect_value(path, offset, limit)).map(|v| ("m_File".to_string(), v)),
            ((self.m_short_path).inspect_value(path, offset, limit))
                .map(|v| ("m_ShortPath".to_string(), v)),
            ((self.m_overhead).inspect_value(path, offset, limit))
                .map(|v| ("m_Overhead".to_string(), v)),
            ((self.m_contents).inspect_value(path, offset, limit))
                .map(|v| ("m_Contents".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
