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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = rdr.read_bool()?;
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_size = None;
        let mut m_center = None;
        let mut m_is_view_volume = None;
        let mut m_is_target_volume = None;
        let mut m_target_resolution = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_size = Some(Vector3f::parse(rdr, ver)?);
            m_center = Some(Vector3f::parse(rdr, ver)?);
            m_is_view_volume = Some(OcclusionAreaMIsViewVolumeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_is_target_volume = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_resolution = Some(rdr.read_i32()?);
        }

        Ok(OcclusionArea {
            m_game_object: m_game_object.unwrap(),
            m_size: m_size.unwrap(),
            m_center: m_center.unwrap(),
            m_is_view_volume: m_is_view_volume.unwrap(),
            m_is_target_volume,
            m_target_resolution,
        })
    }
}
