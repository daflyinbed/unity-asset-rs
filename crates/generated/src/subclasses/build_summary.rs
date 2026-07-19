//! 自动生成。请勿手动修改。

use crate::subclasses::date_time::DateTime;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;

/// 版本枚举：BuildSummaryBuildResultVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildSummaryBuildResultVariant {
    Default(i32),
    V2023_1(i32),
}

impl crate::Parse for BuildSummaryBuildResultVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BuildSummaryMultiProcessEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildSummaryMultiProcessEnabledVariant {
    Default(bool),
    V6000_5(bool),
}

impl crate::Parse for BuildSummaryMultiProcessEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuildSummary {
    pub name: Option<String>,
    pub build_start_time: Option<DateTime>,
    pub platform_name: String,
    pub build_guid: Option<Guid>,
    pub options: i32,
    pub build_session_guid: Option<Guid>,
    pub asset_bundle_options: i32,
    pub platform_group_name: Option<String>,
    pub output_path: String,
    pub subtarget: Option<i32>,
    pub crc: u32,
    pub total_size: u64,
    pub total_time_ms: Option<u64>,
    pub build_content_options: Option<i32>,
    pub total_errors: i32,
    pub data_path: Option<String>,
    pub total_warnings: i32,
    pub total_time_ticks: Option<u64>,
    pub build_type: Option<i32>,
    pub success: Option<bool>,
    pub build_name: Option<String>,
    pub build_result: Option<BuildSummaryBuildResultVariant>,
    pub multi_process_enabled: Option<BuildSummaryMultiProcessEnabledVariant>,
    pub build_manifest_hash: Option<Hash128>,
}

impl crate::Parse for BuildSummary {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut build_start_time = None;
        let mut platform_name = None;
        let mut build_guid = None;
        let mut options = None;
        let mut build_session_guid = None;
        let mut asset_bundle_options = None;
        let mut platform_group_name = None;
        let mut output_path = None;
        let mut subtarget = None;
        let mut crc = None;
        let mut total_size = None;
        let mut total_time_ms = None;
        let mut build_content_options = None;
        let mut total_errors = None;
        let mut data_path = None;
        let mut total_warnings = None;
        let mut total_time_ticks = None;
        let mut build_type = None;
        let mut success = None;
        let mut build_name = None;
        let mut build_result = None;
        let mut multi_process_enabled = None;
        let mut build_manifest_hash = None;
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            build_start_time = Some(DateTime::parse(rdr, ver)?);
            build_guid = Some(Guid::parse(rdr, ver)?);
            build_session_guid = Some(Guid::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            build_start_time = Some(DateTime::parse(rdr, ver)?);
            build_guid = Some(Guid::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            name = Some(rdr.read_aligned_string()?);
            build_guid = Some(Guid::parse(rdr, ver)?);
        } else {
            name = Some(rdr.read_aligned_string()?);
        }
        platform_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            platform_group_name = Some(rdr.read_aligned_string()?);
            subtarget = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            platform_group_name = Some(rdr.read_aligned_string()?);
        } else {
        }
        options = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            build_content_options = Some(rdr.read_i32()?);
        } else {
        }
        asset_bundle_options = Some(rdr.read_i32()?);
        output_path = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            data_path = Some(rdr.read_aligned_string()?);
            build_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            data_path = Some(rdr.read_aligned_string()?);
        } else {
        }
        crc = Some(rdr.read_u32()?);
        total_size = Some(rdr.read_u64()?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            total_time_ticks = Some(rdr.read_u64()?);
        } else {
            total_time_ms = Some(rdr.read_u64()?);
        }
        total_errors = Some(rdr.read_i32()?);
        total_warnings = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            build_type = Some(rdr.read_i32()?);
            build_result = Some(BuildSummaryBuildResultVariant::parse(rdr, ver)?);
            multi_process_enabled = Some(BuildSummaryMultiProcessEnabledVariant::parse(rdr, ver)?);
            build_manifest_hash = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            build_type = Some(rdr.read_i32()?);
            build_result = Some(BuildSummaryBuildResultVariant::parse(rdr, ver)?);
            multi_process_enabled = Some(BuildSummaryMultiProcessEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            build_type = Some(rdr.read_i32()?);
            build_result = Some(BuildSummaryBuildResultVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            build_type = Some(rdr.read_i32()?);
            success = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(BuildSummary {
            name,
            build_start_time,
            platform_name: platform_name.unwrap(),
            build_guid,
            options: options.unwrap(),
            build_session_guid,
            asset_bundle_options: asset_bundle_options.unwrap(),
            platform_group_name,
            output_path: output_path.unwrap(),
            subtarget,
            crc: crc.unwrap(),
            total_size: total_size.unwrap(),
            total_time_ms,
            build_content_options,
            total_errors: total_errors.unwrap(),
            data_path,
            total_warnings: total_warnings.unwrap(),
            total_time_ticks,
            build_type,
            success,
            build_name,
            build_result,
            multi_process_enabled,
            build_manifest_hash,
        })
    }
}
