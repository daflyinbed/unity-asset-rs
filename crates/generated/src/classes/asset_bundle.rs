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

impl crate::ToValue for AssetBundleMPreloadTableVariant {
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

impl crate::ToValue for AssetBundleMDependenciesVariant {
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

impl crate::ToValue for AssetBundle {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_PreloadTable".to_string(),
                self.m_preload_table.to_value(),
            )),
            Some(("m_Container".to_string(), self.m_container.to_value())),
            Some(("m_MainAsset".to_string(), self.m_main_asset.to_value())),
            crate::opt_field("m_ScriptCompatibility", &self.m_script_compatibility),
            crate::opt_field("m_RuntimeCompatibility", &self.m_runtime_compatibility),
            crate::opt_field("m_ClassVersionMap", &self.m_class_version_map),
            crate::opt_field("m_ClassCompatibility", &self.m_class_compatibility),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("m_Dependencies", &self.m_dependencies),
            crate::opt_field(
                "m_IsStreamedSceneAssetBundle",
                &self.m_is_streamed_scene_asset_bundle,
            ),
            crate::opt_field("m_PathFlags", &self.m_path_flags),
            crate::opt_field("m_ExplicitDataLayout", &self.m_explicit_data_layout),
            crate::opt_field("m_SceneHashes", &self.m_scene_hashes),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_PreloadTable" => (self.m_preload_table).inspect_value(path, offset, limit),
                "m_Container" => (self.m_container).inspect_value(path, offset, limit),
                "m_MainAsset" => (self.m_main_asset).inspect_value(path, offset, limit),
                "m_ScriptCompatibility" => {
                    (self.m_script_compatibility).inspect_value(path, offset, limit)
                }
                "m_RuntimeCompatibility" => {
                    (self.m_runtime_compatibility).inspect_value(path, offset, limit)
                }
                "m_ClassVersionMap" => {
                    (self.m_class_version_map).inspect_value(path, offset, limit)
                }
                "m_ClassCompatibility" => {
                    (self.m_class_compatibility).inspect_value(path, offset, limit)
                }
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_Dependencies" => (self.m_dependencies).inspect_value(path, offset, limit),
                "m_IsStreamedSceneAssetBundle" => {
                    (self.m_is_streamed_scene_asset_bundle).inspect_value(path, offset, limit)
                }
                "m_PathFlags" => (self.m_path_flags).inspect_value(path, offset, limit),
                "m_ExplicitDataLayout" => {
                    (self.m_explicit_data_layout).inspect_value(path, offset, limit)
                }
                "m_SceneHashes" => (self.m_scene_hashes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_preload_table).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadTable".to_string(), v)),
            ((self.m_container).inspect_value(path, offset, limit))
                .map(|v| ("m_Container".to_string(), v)),
            ((self.m_main_asset).inspect_value(path, offset, limit))
                .map(|v| ("m_MainAsset".to_string(), v)),
            ((self.m_script_compatibility).inspect_value(path, offset, limit))
                .map(|v| ("m_ScriptCompatibility".to_string(), v)),
            ((self.m_runtime_compatibility).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimeCompatibility".to_string(), v)),
            ((self.m_class_version_map).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassVersionMap".to_string(), v)),
            ((self.m_class_compatibility).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassCompatibility".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("m_Dependencies".to_string(), v)),
            ((self.m_is_streamed_scene_asset_bundle).inspect_value(path, offset, limit))
                .map(|v| ("m_IsStreamedSceneAssetBundle".to_string(), v)),
            ((self.m_path_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_PathFlags".to_string(), v)),
            ((self.m_explicit_data_layout).inspect_value(path, offset, limit))
                .map(|v| ("m_ExplicitDataLayout".to_string(), v)),
            ((self.m_scene_hashes).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneHashes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
