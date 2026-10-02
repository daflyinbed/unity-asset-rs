//! 自动生成。请勿手动修改。

use crate::subclasses::occlusion_scene::OcclusionScene;

/// 版本枚举：OcclusionCullingDataMScenesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum OcclusionCullingDataMScenesVariant {
    Default(Vec<OcclusionScene>),
    V2017_1(Vec<OcclusionScene>),
}

impl crate::Parse for OcclusionCullingDataMScenesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OcclusionScene::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OcclusionScene::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for OcclusionCullingDataMScenesVariant {
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
pub struct OcclusionCullingData {
    pub m_name: String,
    pub m_pvs_data: Vec<u8>,
    pub m_scenes: OcclusionCullingDataMScenesVariant,
}

impl crate::Parse for OcclusionCullingData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_pvs_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_scenes = OcclusionCullingDataMScenesVariant::parse(rdr, ver)?;

        Ok(OcclusionCullingData {
            m_name,
            m_pvs_data,
            m_scenes,
        })
    }
}

impl crate::ToValue for OcclusionCullingData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_PVSData".to_string(), self.m_pvs_data.to_value())),
            Some(("m_Scenes".to_string(), self.m_scenes.to_value())),
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
                "m_PVSData" => (self.m_pvs_data).inspect_value(path, offset, limit),
                "m_Scenes" => (self.m_scenes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_pvs_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PVSData".to_string(), v)),
            ((self.m_scenes).inspect_value(path, offset, limit))
                .map(|v| ("m_Scenes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
