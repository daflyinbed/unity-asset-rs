//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_report_file::BuildReportFile;
use crate::subclasses::build_step_info::BuildStepInfo;
use crate::subclasses::build_summary::BuildSummary;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct BuildReport {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_summary: BuildSummary,
    pub m_files: Vec<BuildReportFile>,
    pub m_build_steps: Vec<BuildStepInfo>,
    pub m_appendices: Vec<PPtr>,
    pub m_root_asset_paths: Option<Vec<String>>,
}

impl crate::Parse for BuildReport {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_summary = BuildSummary::parse(r, ver)?;
        r.align(4)?;
        let m_files = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildReportFile::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_build_steps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildStepInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_appendices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_root_asset_paths = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(BuildReport {
            parent,
            m_name,
            m_summary,
            m_files,
            m_build_steps,
            m_appendices,
            m_root_asset_paths,
        })
    }
}
