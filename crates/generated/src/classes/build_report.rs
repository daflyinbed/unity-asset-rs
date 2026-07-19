//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_report_file::BuildReportFile;
use crate::subclasses::build_step_info::BuildStepInfo;
use crate::subclasses::build_summary::BuildSummary;

/// 版本枚举：BuildReportMFilesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildReportMFilesVariant {
    Default(Vec<BuildReportFile>),
    V2017_1(Vec<BuildReportFile>),
}

impl crate::Parse for BuildReportMFilesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildReportFile::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildReportFile::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BuildReportMBuildStepsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildReportMBuildStepsVariant {
    Default(Vec<BuildStepInfo>),
    V2017_1(Vec<BuildStepInfo>),
}

impl crate::Parse for BuildReportMBuildStepsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildStepInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildStepInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BuildReportMAppendicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildReportMAppendicesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for BuildReportMAppendicesVariant {
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
#[derive(Debug, Clone)]
pub struct BuildReport {
    pub m_name: String,
    pub m_summary: BuildSummary,
    pub m_files: BuildReportMFilesVariant,
    pub m_build_steps: BuildReportMBuildStepsVariant,
    pub m_appendices: BuildReportMAppendicesVariant,
    pub m_root_asset_paths: Option<Vec<String>>,
}

impl crate::Parse for BuildReport {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_summary = None;
        let mut m_files = None;
        let mut m_build_steps = None;
        let mut m_appendices = None;
        let mut m_root_asset_paths = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_summary = Some(BuildSummary::parse(rdr, ver)?);
        m_files = Some(BuildReportMFilesVariant::parse(rdr, ver)?);
        m_build_steps = Some(BuildReportMBuildStepsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_root_asset_paths = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_appendices = Some(BuildReportMAppendicesVariant::parse(rdr, ver)?);

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
