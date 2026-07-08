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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OcclusionScene::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OcclusionScene::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_pvs_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_scenes = OcclusionCullingDataMScenesVariant::parse(r, ver)?;

        Ok(OcclusionCullingData {
            m_name,
            m_pvs_data,
            m_scenes,
        })
    }
}
