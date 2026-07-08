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

/// 版本枚举：SceneMPortalsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SceneMPortalsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SceneMPortalsVariant {
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
        r: &mut R,
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
            m_scene_guid = Some(Guid::parse(r, ver)?);
            m_occlusion_culling_data = Some(crate::PPtr::parse(r, ver)?);
            m_static_renderers = Some(SceneMStaticRenderersVariant::parse(r, ver)?);
            m_portals = Some(SceneMPortalsVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_pvs_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_pvs_objects_array = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_pvs_portals_array = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_pvs_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_query_mode = Some(r.read_u32()?);
            m_pvs_objects_array = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_pvs_portals_array = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
            m_pvs_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_pvs_objects_array = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
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
