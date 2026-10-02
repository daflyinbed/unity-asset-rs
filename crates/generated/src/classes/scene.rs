//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

/// 版本枚举：SceneMStaticRenderersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SceneMStaticRenderersVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SceneMStaticRenderersVariant {
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

impl crate::ToValue for SceneMStaticRenderersVariant {
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

/// 版本枚举：SceneMPortalsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SceneMPortalsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SceneMPortalsVariant {
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

impl crate::ToValue for SceneMPortalsVariant {
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
pub struct Scene {
    pub m_pvs_data: Option<Vec<u8>>,
    pub m_scene_guid: Option<Guid>,
    pub m_pvs_objects_array: Option<Vec<PPtr>>,
    pub m_query_mode: Option<u32>,
    pub m_occlusion_culling_data: Option<PPtr>,
    pub m_pvs_portals_array: Option<Vec<PPtr>>,
    pub m_static_renderers: Option<SceneMStaticRenderersVariant>,
    pub m_portals: Option<SceneMPortalsVariant>,
}

impl crate::Parse for Scene {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_pvs_data = None;
        let mut m_scene_guid = None;
        let mut m_pvs_objects_array = None;
        let mut m_query_mode = None;
        let mut m_occlusion_culling_data = None;
        let mut m_pvs_portals_array = None;
        let mut m_static_renderers = None;
        let mut m_portals = None;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_scene_guid = Some(Guid::parse(rdr, ver)?);
            m_occlusion_culling_data = Some(crate::PPtr::parse(rdr, ver)?);
            m_static_renderers = Some(SceneMStaticRenderersVariant::parse(rdr, ver)?);
            m_portals = Some(SceneMPortalsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_pvs_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_pvs_objects_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_pvs_portals_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_pvs_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_query_mode = Some(rdr.read_u32()?);
            m_pvs_objects_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_pvs_portals_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_pvs_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_pvs_objects_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }

        Ok(Scene {
            m_pvs_data,
            m_scene_guid,
            m_pvs_objects_array,
            m_query_mode,
            m_occlusion_culling_data,
            m_pvs_portals_array,
            m_static_renderers,
            m_portals,
        })
    }
}

impl crate::ToValue for Scene {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_PVSData", &self.m_pvs_data),
            crate::opt_field("m_SceneGUID", &self.m_scene_guid),
            crate::opt_field("m_PVSObjectsArray", &self.m_pvs_objects_array),
            crate::opt_field("m_QueryMode", &self.m_query_mode),
            crate::opt_field("m_OcclusionCullingData", &self.m_occlusion_culling_data),
            crate::opt_field("m_PVSPortalsArray", &self.m_pvs_portals_array),
            crate::opt_field("m_StaticRenderers", &self.m_static_renderers),
            crate::opt_field("m_Portals", &self.m_portals),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_PVSData" => (self.m_pvs_data).inspect_value(path, offset, limit),
                "m_SceneGUID" => (self.m_scene_guid).inspect_value(path, offset, limit),
                "m_PVSObjectsArray" => {
                    (self.m_pvs_objects_array).inspect_value(path, offset, limit)
                }
                "m_QueryMode" => (self.m_query_mode).inspect_value(path, offset, limit),
                "m_OcclusionCullingData" => {
                    (self.m_occlusion_culling_data).inspect_value(path, offset, limit)
                }
                "m_PVSPortalsArray" => {
                    (self.m_pvs_portals_array).inspect_value(path, offset, limit)
                }
                "m_StaticRenderers" => (self.m_static_renderers).inspect_value(path, offset, limit),
                "m_Portals" => (self.m_portals).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_pvs_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PVSData".to_string(), v)),
            ((self.m_scene_guid).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneGUID".to_string(), v)),
            ((self.m_pvs_objects_array).inspect_value(path, offset, limit))
                .map(|v| ("m_PVSObjectsArray".to_string(), v)),
            ((self.m_query_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_QueryMode".to_string(), v)),
            ((self.m_occlusion_culling_data).inspect_value(path, offset, limit))
                .map(|v| ("m_OcclusionCullingData".to_string(), v)),
            ((self.m_pvs_portals_array).inspect_value(path, offset, limit))
                .map(|v| ("m_PVSPortalsArray".to_string(), v)),
            ((self.m_static_renderers).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticRenderers".to_string(), v)),
            ((self.m_portals).inspect_value(path, offset, limit))
                .map(|v| ("m_Portals".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
