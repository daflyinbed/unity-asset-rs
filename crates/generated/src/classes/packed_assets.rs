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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildReportPackedAssetInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildReportPackedAssetInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
