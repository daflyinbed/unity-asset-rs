//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_report_file::BuildReportFile;
use crate::subclasses::build_step_info::BuildStepInfo;
use crate::subclasses::build_summary::BuildSummary;

#[derive(Debug, Clone)]
pub struct BuildReport {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_summary: BuildSummary,
    pub m_files: Vec<BuildReportFile>,
    pub m_build_steps: Vec<BuildStepInfo>,
    pub m_appendices: Vec<PPtr>,
}

impl crate::Parse for BuildReport {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_summary = BuildSummary::parse(r, ver)?;
        r.align(4)?;
        let m_files = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildReportFile::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_build_steps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildStepInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_appendices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(BuildReport {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_summary,
            m_files,
            m_build_steps,
            m_appendices,
        })
    }
}
