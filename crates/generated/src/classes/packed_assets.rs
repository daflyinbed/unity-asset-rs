//! 自动生成。请勿手动修改。

use crate::subclasses::build_report_packed_asset_info::BuildReportPackedAssetInfo;


/// 版本枚举：PackedAssetsMContentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PackedAssetsMContentsVariant {
    Default(Vec<BuildReportPackedAssetInfo>),
    V5_4_2(Vec<BuildReportPackedAssetInfo>),
}

impl crate::Parse for PackedAssetsMContentsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 2) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildReportPackedAssetInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V5_4_2(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildReportPackedAssetInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_short_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_overhead = r.read_u64()?;
        let m_contents = PackedAssetsMContentsVariant::parse(r, ver)?;

        Ok(PackedAssets {
            m_file,
            m_short_path,
            m_overhead,
            m_contents,
        })
    }
}
