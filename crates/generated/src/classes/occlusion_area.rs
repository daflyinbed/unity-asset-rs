//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：OcclusionAreaMIsViewVolumeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum OcclusionAreaMIsViewVolumeVariant {
    Default(bool),
    V4_3(bool),
}

impl crate::Parse for OcclusionAreaMIsViewVolumeVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct OcclusionArea {
    pub m_game_object: PPtr,
    pub m_size: Vector3f,
    pub m_center: Vector3f,
    pub m_is_view_volume: OcclusionAreaMIsViewVolumeVariant,
    pub m_is_target_volume: Option<bool>,
    pub m_target_resolution: Option<i32>,
}

impl crate::Parse for OcclusionArea {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_size = Vector3f::parse(r, ver)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_is_view_volume = OcclusionAreaMIsViewVolumeVariant::parse(r, ver)?;
        let m_is_target_volume = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_target_resolution = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(OcclusionArea {
            m_game_object,
            m_size,
            m_center,
            m_is_view_volume,
            m_is_target_volume,
            m_target_resolution,
        })
    }
}
