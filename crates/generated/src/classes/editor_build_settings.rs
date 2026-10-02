//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::scene::Scene;

/// 版本枚举：EditorBuildSettingsMScenesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorBuildSettingsMScenesVariant {
    Default(Vec<Scene>),
    V2017_1(Vec<Scene>),
}

impl crate::Parse for EditorBuildSettingsMScenesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Scene::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Scene::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorBuildSettingsMScenesVariant {
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
pub struct EditorBuildSettings {
    pub m_scenes: EditorBuildSettingsMScenesVariant,
    pub m_config_objects: Option<Vec<(String, PPtr)>>,
    pub m_use_ucbp_for_asset_bundles: Option<bool>,
}

impl crate::Parse for EditorBuildSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_scenes = None;
        let mut m_config_objects = None;
        let mut m_use_ucbp_for_asset_bundles = None;
        m_scenes = Some(EditorBuildSettingsMScenesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_config_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_config_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_use_ucbp_for_asset_bundles = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_config_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }

        Ok(EditorBuildSettings {
            m_scenes: m_scenes.unwrap(),
            m_config_objects,
            m_use_ucbp_for_asset_bundles,
        })
    }
}

impl crate::ToValue for EditorBuildSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Scenes".to_string(), self.m_scenes.to_value())),
            crate::opt_field("m_configObjects", &self.m_config_objects),
            crate::opt_field(
                "m_UseUCBPForAssetBundles",
                &self.m_use_ucbp_for_asset_bundles,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Scenes" => (self.m_scenes).inspect_value(path, offset, limit),
                "m_configObjects" => (self.m_config_objects).inspect_value(path, offset, limit),
                "m_UseUCBPForAssetBundles" => {
                    (self.m_use_ucbp_for_asset_bundles).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_scenes).inspect_value(path, offset, limit))
                .map(|v| ("m_Scenes".to_string(), v)),
            ((self.m_config_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_configObjects".to_string(), v)),
            ((self.m_use_ucbp_for_asset_bundles).inspect_value(path, offset, limit))
                .map(|v| ("m_UseUCBPForAssetBundles".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
