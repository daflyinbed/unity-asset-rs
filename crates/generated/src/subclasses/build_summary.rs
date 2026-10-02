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

impl crate::ToValue for BuildSummaryBuildResultVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for BuildSummaryMultiProcessEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for BuildSummary {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("name", &self.name),
            crate::opt_field("buildStartTime", &self.build_start_time),
            Some(("platformName".to_string(), self.platform_name.to_value())),
            crate::opt_field("buildGUID", &self.build_guid),
            Some(("options".to_string(), self.options.to_value())),
            crate::opt_field("buildSessionGUID", &self.build_session_guid),
            Some((
                "assetBundleOptions".to_string(),
                self.asset_bundle_options.to_value(),
            )),
            crate::opt_field("platformGroupName", &self.platform_group_name),
            Some(("outputPath".to_string(), self.output_path.to_value())),
            crate::opt_field("subtarget", &self.subtarget),
            Some(("crc".to_string(), self.crc.to_value())),
            Some(("totalSize".to_string(), self.total_size.to_value())),
            crate::opt_field("totalTimeMS", &self.total_time_ms),
            crate::opt_field("buildContentOptions", &self.build_content_options),
            Some(("totalErrors".to_string(), self.total_errors.to_value())),
            crate::opt_field("dataPath", &self.data_path),
            Some(("totalWarnings".to_string(), self.total_warnings.to_value())),
            crate::opt_field("totalTimeTicks", &self.total_time_ticks),
            crate::opt_field("buildType", &self.build_type),
            crate::opt_field("success", &self.success),
            crate::opt_field("buildName", &self.build_name),
            crate::opt_field("buildResult", &self.build_result),
            crate::opt_field("multiProcessEnabled", &self.multi_process_enabled),
            crate::opt_field("buildManifestHash", &self.build_manifest_hash),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "buildStartTime" => (self.build_start_time).inspect_value(path, offset, limit),
                "platformName" => (self.platform_name).inspect_value(path, offset, limit),
                "buildGUID" => (self.build_guid).inspect_value(path, offset, limit),
                "options" => (self.options).inspect_value(path, offset, limit),
                "buildSessionGUID" => (self.build_session_guid).inspect_value(path, offset, limit),
                "assetBundleOptions" => {
                    (self.asset_bundle_options).inspect_value(path, offset, limit)
                }
                "platformGroupName" => {
                    (self.platform_group_name).inspect_value(path, offset, limit)
                }
                "outputPath" => (self.output_path).inspect_value(path, offset, limit),
                "subtarget" => (self.subtarget).inspect_value(path, offset, limit),
                "crc" => (self.crc).inspect_value(path, offset, limit),
                "totalSize" => (self.total_size).inspect_value(path, offset, limit),
                "totalTimeMS" => (self.total_time_ms).inspect_value(path, offset, limit),
                "buildContentOptions" => {
                    (self.build_content_options).inspect_value(path, offset, limit)
                }
                "totalErrors" => (self.total_errors).inspect_value(path, offset, limit),
                "dataPath" => (self.data_path).inspect_value(path, offset, limit),
                "totalWarnings" => (self.total_warnings).inspect_value(path, offset, limit),
                "totalTimeTicks" => (self.total_time_ticks).inspect_value(path, offset, limit),
                "buildType" => (self.build_type).inspect_value(path, offset, limit),
                "success" => (self.success).inspect_value(path, offset, limit),
                "buildName" => (self.build_name).inspect_value(path, offset, limit),
                "buildResult" => (self.build_result).inspect_value(path, offset, limit),
                "multiProcessEnabled" => {
                    (self.multi_process_enabled).inspect_value(path, offset, limit)
                }
                "buildManifestHash" => {
                    (self.build_manifest_hash).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.build_start_time).inspect_value(path, offset, limit))
                .map(|v| ("buildStartTime".to_string(), v)),
            ((self.platform_name).inspect_value(path, offset, limit))
                .map(|v| ("platformName".to_string(), v)),
            ((self.build_guid).inspect_value(path, offset, limit))
                .map(|v| ("buildGUID".to_string(), v)),
            ((self.options).inspect_value(path, offset, limit)).map(|v| ("options".to_string(), v)),
            ((self.build_session_guid).inspect_value(path, offset, limit))
                .map(|v| ("buildSessionGUID".to_string(), v)),
            ((self.asset_bundle_options).inspect_value(path, offset, limit))
                .map(|v| ("assetBundleOptions".to_string(), v)),
            ((self.platform_group_name).inspect_value(path, offset, limit))
                .map(|v| ("platformGroupName".to_string(), v)),
            ((self.output_path).inspect_value(path, offset, limit))
                .map(|v| ("outputPath".to_string(), v)),
            ((self.subtarget).inspect_value(path, offset, limit))
                .map(|v| ("subtarget".to_string(), v)),
            ((self.crc).inspect_value(path, offset, limit)).map(|v| ("crc".to_string(), v)),
            ((self.total_size).inspect_value(path, offset, limit))
                .map(|v| ("totalSize".to_string(), v)),
            ((self.total_time_ms).inspect_value(path, offset, limit))
                .map(|v| ("totalTimeMS".to_string(), v)),
            ((self.build_content_options).inspect_value(path, offset, limit))
                .map(|v| ("buildContentOptions".to_string(), v)),
            ((self.total_errors).inspect_value(path, offset, limit))
                .map(|v| ("totalErrors".to_string(), v)),
            ((self.data_path).inspect_value(path, offset, limit))
                .map(|v| ("dataPath".to_string(), v)),
            ((self.total_warnings).inspect_value(path, offset, limit))
                .map(|v| ("totalWarnings".to_string(), v)),
            ((self.total_time_ticks).inspect_value(path, offset, limit))
                .map(|v| ("totalTimeTicks".to_string(), v)),
            ((self.build_type).inspect_value(path, offset, limit))
                .map(|v| ("buildType".to_string(), v)),
            ((self.success).inspect_value(path, offset, limit)).map(|v| ("success".to_string(), v)),
            ((self.build_name).inspect_value(path, offset, limit))
                .map(|v| ("buildName".to_string(), v)),
            ((self.build_result).inspect_value(path, offset, limit))
                .map(|v| ("buildResult".to_string(), v)),
            ((self.multi_process_enabled).inspect_value(path, offset, limit))
                .map(|v| ("multiProcessEnabled".to_string(), v)),
            ((self.build_manifest_hash).inspect_value(path, offset, limit))
                .map(|v| ("buildManifestHash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
