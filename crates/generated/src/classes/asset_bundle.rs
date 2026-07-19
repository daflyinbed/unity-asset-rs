//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::asset_bundle_script_info::AssetBundleScriptInfo;
use crate::subclasses::asset_info::AssetInfo;

/// 版本枚举：AssetBundleMPreloadTableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetBundleMPreloadTableVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AssetBundleMPreloadTableVariant {
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

/// 版本枚举：AssetBundleMDependenciesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetBundleMDependenciesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for AssetBundleMDependenciesVariant {
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
#[derive(Debug, Clone)]
pub struct AssetBundle {
    pub m_name: String,
    pub m_preload_table: AssetBundleMPreloadTableVariant,
    pub m_container: Vec<(String, AssetInfo)>,
    pub m_main_asset: AssetInfo,
    pub m_script_compatibility: Option<Vec<AssetBundleScriptInfo>>,
    pub m_runtime_compatibility: Option<u32>,
    pub m_class_version_map: Option<Vec<(i32, i32)>>,
    pub m_class_compatibility: Option<Vec<(i32, u32)>>,
    pub m_asset_bundle_name: Option<String>,
    pub m_dependencies: Option<AssetBundleMDependenciesVariant>,
    pub m_is_streamed_scene_asset_bundle: Option<bool>,
    pub m_path_flags: Option<i32>,
    pub m_explicit_data_layout: Option<i32>,
    pub m_scene_hashes: Option<Vec<(String, String)>>,
}

impl crate::Parse for AssetBundle {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_preload_table = None;
        let mut m_container = None;
        let mut m_main_asset = None;
        let mut m_script_compatibility = None;
        let mut m_runtime_compatibility = None;
        let mut m_class_version_map = None;
        let mut m_class_compatibility = None;
        let mut m_asset_bundle_name = None;
        let mut m_dependencies = None;
        let mut m_is_streamed_scene_asset_bundle = None;
        let mut m_path_flags = None;
        let mut m_explicit_data_layout = None;
        let mut m_scene_hashes = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_preload_table = Some(AssetBundleMPreloadTableVariant::parse(rdr, ver)?);
        m_container = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = AssetInfo::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_main_asset = Some(AssetInfo::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_runtime_compatibility = Some(rdr.read_u32()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_dependencies = Some(AssetBundleMDependenciesVariant::parse(rdr, ver)?);
            m_is_streamed_scene_asset_bundle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_explicit_data_layout = Some(rdr.read_i32()?);
            m_path_flags = Some(rdr.read_i32()?);
            m_scene_hashes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_runtime_compatibility = Some(rdr.read_u32()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_dependencies = Some(AssetBundleMDependenciesVariant::parse(rdr, ver)?);
            m_is_streamed_scene_asset_bundle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_path_flags = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_runtime_compatibility = Some(rdr.read_u32()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_dependencies = Some(AssetBundleMDependenciesVariant::parse(rdr, ver)?);
            m_is_streamed_scene_asset_bundle = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_class_version_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_runtime_compatibility = Some(rdr.read_u32()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_dependencies = Some(AssetBundleMDependenciesVariant::parse(rdr, ver)?);
            m_is_streamed_scene_asset_bundle = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_runtime_compatibility = Some(rdr.read_u32()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_dependencies = Some(AssetBundleMDependenciesVariant::parse(rdr, ver)?);
            m_is_streamed_scene_asset_bundle = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_script_compatibility = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AssetBundleScriptInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_class_compatibility = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_u32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_runtime_compatibility = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_script_compatibility = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AssetBundleScriptInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_class_compatibility = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_u32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_script_compatibility = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AssetBundleScriptInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }

        Ok(AssetBundle {
            m_name: m_name.unwrap(),
            m_preload_table: m_preload_table.unwrap(),
            m_container: m_container.unwrap(),
            m_main_asset: m_main_asset.unwrap(),
            m_script_compatibility,
            m_runtime_compatibility,
            m_class_version_map,
            m_class_compatibility,
            m_asset_bundle_name,
            m_dependencies,
            m_is_streamed_scene_asset_bundle,
            m_path_flags,
            m_explicit_data_layout,
            m_scene_hashes,
        })
    }
}
