//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_report_file::BuildReportFile;
use crate::subclasses::build_step_info::BuildStepInfo;
use crate::subclasses::build_summary::BuildSummary;


/// 版本枚举：BuildReportMAppendicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildReportMAppendicesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for BuildReportMAppendicesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuildReport {
    pub m_name: String,
    pub m_summary: BuildSummary,
    pub m_files: Vec<BuildReportFile>,
    pub m_build_steps: Vec<BuildStepInfo>,
    pub m_appendices: BuildReportMAppendicesVariant,
    pub m_root_asset_paths: Option<Vec<String>>,
}

impl crate::Parse for BuildReport {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_summary = None;
        let mut m_files = None;
        let mut m_build_steps = None;
        let mut m_appendices = None;
        let mut m_root_asset_paths = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_summary = Some(BuildSummary::parse(r, ver)?);
            r.align(4)?;
            m_files = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildReportFile::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_build_steps = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildStepInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_root_asset_paths = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }
            m_appendices = Some(BuildReportMAppendicesVariant::parse(r, ver)?);

        Ok(BuildReport {
            m_name: m_name.unwrap(),
            m_summary: m_summary.unwrap(),
            m_files: m_files.unwrap(),
            m_build_steps: m_build_steps.unwrap(),
            m_appendices: m_appendices.unwrap(),
            m_root_asset_paths,
        })
    }
}
